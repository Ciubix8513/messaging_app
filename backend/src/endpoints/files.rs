use std::{io::Write, path::PathBuf, println};

use actix_multipart::Multipart;
use actix_web::{get, http::header::ContentDisposition, post, web, HttpResponse, Responder};
use chrono::Local;
use common_lib::{grimoire::UPLOAD_METADATA_NAME, UploadFile};

use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use futures::{Stream, StreamExt, TryStreamExt};

use tokio::io::AsyncReadExt;

use crate::{
    grimoire,
    models::{CreateFile, CreateMessage},
    utils::{generate_uuid, is_logged_in},
    DbPool,
};

use super::Param;

#[post("/files/upload")]
pub async fn upload_file(
    session: actix_session::Session,
    pool: web::Data<DbPool>,
    mut payload: Multipart,
) -> impl Responder {
    //Perform initial auth check here, just so we don't have to waste resources if it doesn't pass
    let sender_id = is_logged_in(&session);
    if sender_id.is_err() {
        return HttpResponse::Unauthorized().body("");
    }
    let u_id: i32 = sender_id.unwrap();
    let connection = &mut pool.get().unwrap();

    let mut got_metadata = false;
    let mut msg_id = 0;
    while let Some(item) = payload.next().await {
        println!("Got an item");
        let mut field = item.unwrap();

        //First item has to be the metadata
        //This metadata receiving should be fine
        //I can't extract it out of the loop so i gotta do this got_metadata thing
        if !got_metadata {
            if field.name() != UPLOAD_METADATA_NAME {
                return HttpResponse::BadRequest().body("No metadata provided");
            }
            let json = String::from_utf8(field.next().await.unwrap().unwrap().into());
            if json.is_err() {
                return HttpResponse::BadRequest().body("Wrong metadata structure");
            }
            let metadata_value = serde_json::from_str::<UploadFile>(&json.unwrap()).ok();
            //Get metadata
            if let Some(m_data) = metadata_value {
                //Check if the user is in the chat they want to send files in
                use crate::schema::group_chat_members::dsl::*;
                let result = group_chat_members
                    .find((m_data.chat_id, u_id))
                    .execute(connection);
                match result {
                    Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
                    Ok(0) => return HttpResponse::Unauthorized().body(""),
                    _ => (),
                }
                {
                    //Create the message files are attached to
                    use crate::schema::messages::dsl::*;
                    let time = Local::now().naive_utc();
                    let result = diesel::insert_into(messages)
                        .values(CreateMessage {
                            chat_id: m_data.chat_id,
                            user_id: u_id,
                            message_text: m_data.message_text.clone(),
                            sent_at: time,
                        })
                        .execute(connection);
                    if let Err(result) = result {
                        return HttpResponse::InternalServerError().body(format!("{result}"));
                    }
                    //Get id of the message, bc I can't know it before doing the insert
                    msg_id = messages
                        .filter(sent_at.eq(time))
                        .filter(user_id.eq(u_id))
                        .filter(chat_id.eq(m_data.chat_id))
                        .select(message_id)
                        .first(connection)
                        .unwrap();
                }
                got_metadata = true;
            } else {
                return HttpResponse::BadRequest().body("Wrong metadata structure");
            }
            continue;
        }
        //At this point we can be sure that user should be able to send files
        //It may be a good idea to do a check on the file size here, but i'm too lazy, so i'm gonna
        //do it on the client

        //The probability of generate 2 identical file paths is 1 in 2 ^ 128 which is astronomically
        //unlikely so we're not gonna bother with it
        let path = grimoire::FILE_LOCATION
            .clone()
            .join(generate_uuid().to_string());
        //Receive the file
        let mut file = tempfile::NamedTempFile::new().unwrap();
        while let Some(chunk) = field.try_next().await.unwrap() {
            file.write_all(&chunk).unwrap();
        }
        file.persist(&path).unwrap();
        {
            //Insert new data into the db
            use crate::schema::files::dsl as f;
            diesel::insert_into(f::files)
                .values(CreateFile {
                    message_id: msg_id,
                    filename: field.name().to_string(),
                    path: path.display().to_string(),
                })
                .execute(connection)
                .unwrap();
        }
    }

    HttpResponse::Ok().body("")
}

#[get("/files/download")]
async fn download_file(
    session: actix_session::Session,
    pool: web::Data<DbPool>,
    web::Query(param): web::Query<Param>,
) -> impl Responder {
    //First check if the user is logged in
    let sender_id = is_logged_in(&session);
    if sender_id.is_err() {
        return HttpResponse::Unauthorized().body("");
    }
    let u_id: i32 = sender_id.unwrap();

    let connection = &mut pool.get().unwrap();
    //Now check if the user is in the same chat as the file
    let filename;
    let path;
    {
        use crate::schema::files::dsl as f;
        use crate::schema::group_chat_members::dsl as gcm;
        use crate::schema::messages::dsl as msg;

        let result = f::files
            .inner_join(msg::messages)
            .inner_join(gcm::group_chat_members.on(gcm::chat_id.eq(msg::chat_id)))
            .filter(gcm::user_id.eq(u_id))
            .filter(f::id.eq(param.id))
            .select((f::filename, f::path))
            .first::<(String, String)>(connection);
        match result {
            Err(diesel::result::Error::NotFound) => return HttpResponse::Unauthorized().body(""),
            Err(e) => return HttpResponse::InternalServerError().body(format!("{e}")),
            Ok(values) => {
                filename = values.0;
                path = values.1;
            }
        }
    }
    //The user should have access to the file, now we need to get the file

    let path = PathBuf::from(path);
    let file = tokio::fs::File::open(path).await.unwrap();

    // Create a buffer to read the file contents into
    let mut buffer = vec![0; 1024]; // Adjust the buffer size as needed

    // Read the file in chunks and convert it into a stream
    let stream = async_stream::stream! {
        let mut reader = file;
        loop {
            // Read a chunk of data from the file
            let bytes_read = reader.read(&mut buffer).await?;

            // Check if we've reached the end of the file
            if bytes_read == 0 {
                break;
            }

            // Emit the chunk of data as a stream item
            let chunk : web::Bytes = web::Bytes::from(buffer[..bytes_read].to_vec());
            yield Ok(chunk);
        }

    };
    HttpResponse::Ok()
        .insert_header(ContentDisposition::attachment(filename))
        .streaming(stream)
}
