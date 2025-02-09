use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use hostname::get;
mod tests; // Includes the tests.rs file

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[get("/")]
async fn index() -> impl Responder {
    let hostname = get_hostname();
    if let Some(host_index) = extract_index(&hostname) {
        HttpResponse::Ok().body(format!("Hello from Rust Web Server! Hostname: {} with index {}",hostname, host_index))

    } else {
        HttpResponse::Ok().body(format!("No index found for {}", hostname))
    }

}

fn get_hostname() -> String {
    get()
        .unwrap_or_else(|_| "Unknown".into())
        .to_string_lossy()
        .to_string()

}

fn extract_index(hostname: &str) -> Option<u32> {
    let prefix = "rust-data-cacher-";
    if hostname.starts_with(prefix) {
        hostname[prefix.len()..].parse::<u32>().ok()
    } else {
        None
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run() 
        .await
}
