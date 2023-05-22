use std::{fs, io::Write, print, println};

use actix_multipart::Multipart;
use actix_web::{post, HttpResponse, Responder};

use crate::grimoire;

use futures::{StreamExt, TryStreamExt};

#[post("/files/upload")]
pub async fn upload_file(
    // session: actix_session::Session,
    // pool: web::Data<DbPool>,
    mut payload: Multipart,
) -> impl Responder {
    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        let name = field.name();
        let content_type = field.content_type();
        println!("Name = {name}");
        if let Some(content_type) = content_type {
            for i in content_type.params() {
                println!("Param  {}/{}", i.0, i.1);
            }
        }

        let path = grimoire::FILE_LOCATION;
        let mut file = fs::File::create(format!("{path}/test")).unwrap();
        let mut i = 0;
        while let Some(chunk) = field.try_next().await.unwrap() {
            println!("Got a chunk");
            file.write_all(&chunk).unwrap();
            i += 1;
        }
        println!("Total of {i} chunks");
    }

    // let sender_id = is_logged_in(&session);
    // if sender_id.is_err() {
    //     return HttpResponse::Unauthorized().body("");
    // }
    // // let u_id: i32 = sender_id.unwrap();

    // // let connection = &mut pool.get().unwrap();

    HttpResponse::Ok().body("")
}
