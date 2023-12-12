use std::path::Path;

use tokio::fs;

use crate::{response::HttpResponse, HttpStatusCode};

pub async fn serve_file<P: AsRef<Path>>(path: P) -> HttpResponse {
    match fs::read(path).await {
        Ok(content) => HttpResponse::new(HttpStatusCode::Ok, content),
        Err(_err) => HttpResponse::new(HttpStatusCode::NotFound, ()),
    }
}
