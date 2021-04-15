use actix_web::{App, HttpResponse, HttpServer, Responder, middleware::Logger, post, web};
use sanitize_filename::sanitize;
use serde::Deserialize;
use dotenv::dotenv;
use futures::StreamExt;
use rand;
use std::io::Write;

struct Config {
    bind_address: String,
}

#[derive(Clone, Debug)]
struct ServiceData {
    image_folder: String,
}

#[derive(Debug, Deserialize)]
struct UploadParams {
    time: Option<String>,
    location: Option<String>,
}

#[post("/upload")]
async fn upload(mut body: web::Payload, service_data: web::Data<ServiceData>, web::Query(query_params): web::Query<UploadParams>) -> impl Responder {
    let mut chunk_count = 0usize;
    let time = query_params.time.unwrap_or("none".to_string());
    let location = query_params.location.unwrap_or("none".to_string());
    let filename = sanitize(rand::random::<u32>().to_string());
    let filepath = format!("{}/{}.jpeg", service_data.image_folder, filename);
    let mut f = web::block(|| std::fs::File::create(filepath)).await.unwrap();
    while let Some(Ok(chunk)) = body.next().await {
        f = web::block(move || f.write_all(&chunk).map(|_| f)).await.unwrap();
    }
    HttpResponse::Ok().body(filename)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = Config {
        bind_address: std::env::var("BIND_ADDRESS").unwrap(),
    };
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(ServiceData {
                image_folder: std::env::var("IMAGE_FOLDER").unwrap(),
            })
            .service(upload)
            .service(actix_files::Files::new("/images", std::env::var("IMAGE_FOLDER").unwrap()))
    })
    .bind(&config.bind_address)?
    .run()
    .await
}
