use std::path::Path;

use tokio::fs;

use crate::{response::HttpResponse, HttpStatusCode};

pub async fn serve_file<P: AsRef<Path>>(path: P) -> HttpResponse {
    match fs::read(path).await {
        Ok(content) => HttpResponse::new(HttpStatusCode::Ok, content),
        Err(_err) => HttpResponse::new(HttpStatusCode::NotFound, ()),
    }
}

pub async fn save_file<P: AsRef<Path>>(path: P, content: &[u8]) -> HttpResponse {
    match fs::write(path, content).await {
        Ok(_) => HttpResponse::new(HttpStatusCode::Created, ()),
        Err(_) => HttpResponse::new(HttpStatusCode::InternalServerError, ()),
    }
}
