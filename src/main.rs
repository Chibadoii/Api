mod mods_and_handlers;

use actix_web::{web, get, App, HttpServer, HttpRequest, HttpResponse, Responder, Error, post};
use actix_web::web::Bytes;
use futures::StreamExt;
use std::fs::{self, File};
use std::io::prelude::*;
//use actix_web::dev::Payload;
use actix_web::web::Payload;

use image::{DynamicImage, ImageFormat};





async fn we()->impl Responder{
    "hello dude"
}
#[get("/")]
async fn t_serv()-> impl Responder{
    "good morning vietnam"
}

async fn api_reqwest(/*mut data: web::Payload*/mut data:Payload) ->HttpResponse{

    if let Some(dat) = data.next().await{
        let snip_filters = data;
        let data_response = mods_and_handlers::handler::handler_reqwest().expect("{Error run reqwest}");
        HttpResponse::Ok().json(data_response)
    }else{
        HttpResponse::NoContent().body("err")
    }


}
async fn upload_image( mut bytes: web::Payload) -> HttpResponse {
    let mut response_str = String::new();

    while let Some(by) = bytes.next().await {
        let image_by = by.unwrap();
        
        let filename = format!("./ex/upload_image_{}", chrono::Utc::now().timestamp());
        fs::write(&filename, image_by).expect(" Err save image");
        response_str = format!("image upload {}", filename);
    }

    HttpResponse::Ok()
                .body(response_str)
}

/*async fn download_im(mut bytes: web::Payload) -> HttpResponse{
        //получение файла
        let mut file_name = String::new();
        let mut image_data = vec![];
    match fs::read_dir("./ex") {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    
                    &mut file_name.push_str(entry.file_name().to_string_lossy().as_ref());
                    break
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    while let Some(_) = bytes.next().await{
        //let image_name = by.to_string.expect("err payload convert to string");
        image_data.push(fs::read(file_name).expect("err read file for download"));

    }
    HttpResponse::Ok()
            .content_type("image/jpeg")
            .body(image_data)
}*/


#[actix_web::main]
async fn main() -> std::io::Result<()> {
   
    HttpServer::new(|| {
        App::new()
            .service(t_serv)
            .route("/upload", web::post().to(upload_image))
            .route("/hey", web::get().to(we))
            .route("/api_reqwest", web::post().to(api_reqwest))
            //.route("/download", web::get().to(download_im))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
