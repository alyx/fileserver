use actix_files as fs;
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{get, App, Error, HttpRequest, HttpServer, middleware::Logger};
use std::env;
use std::ffi::OsStr;

#[get("/{filename:.*}")]
async fn index(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    let root_path: String = env::var("FILESERVER_ROOT").unwrap_or("".to_string());
    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
    let ext: &str = path.extension().and_then(OsStr::to_str).unwrap();
    if ext == "php" {
        return Err(actix_web::error::ErrorForbidden("Forbidden")); 
    }
    let file = fs::NamedFile::open(root_path + path.to_str().unwrap())?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: String = env::var("FILESERVER_PORT").unwrap_or("8080".to_string());
    let host: String = env::var("FILESERVER_HOST").unwrap_or("0.0.0.0".to_string());
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| App::new().wrap(Logger::default()).service(index))
        .bind((host, port.parse::<u16>().unwrap()))?
        .run()
        .await
}
