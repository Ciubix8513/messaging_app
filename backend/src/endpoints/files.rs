use std::{fs, io::Write, println};

use actix_multipart::Multipart;
use actix_web::{post, HttpResponse, Responder};

use common_lib::{grimoire::UPLOAD_METADATA_NAME, UploadFile};
use futures::{StreamExt, TryStreamExt};

#[post("/files/upload")]
pub async fn upload_file(
    // session: actix_session::Session,
    // pool: web::Data<DbPool>,
    mut payload: Multipart,
) -> impl Responder {
    //First item has to be the metadata
    let mut field = payload.next().await.unwrap().unwrap();
    if field.name() != UPLOAD_METADATA_NAME {
        return HttpResponse::BadRequest().body("No metadata provided");
    }
    let json = String::from_utf8(field.next().await.unwrap().unwrap().into()).unwrap();
    let metadata = serde_json::from_str::<UploadFile>(&json);
    if metadata.is_err() {
        return HttpResponse::BadRequest().body("Wrong metadata structure");
    }
    let metadata = metadata.unwrap();

    //Perform auth checks here

    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        let path = std::env::current_dir().unwrap().join("./test");
        let mut file = fs::File::create(path).unwrap();
        let mut i = 0;
        while let Some(chunk) = field.try_next().await.unwrap() {
            file.write_all(&chunk).unwrap();
            i += 1;
        }
        println!("Recieved {i} chunks");
    }

    HttpResponse::Ok().body("")
}
