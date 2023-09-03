use gris::http::HttpServer;

#[tokio::main]
async fn main() {
    let server = HttpServer::new("127.0.0.1:9991".to_owned()).await;

    if let Ok(mut server) = server {
        match server.start().await {
            Ok(_) => println!("recieved connection!"),
            Err(E) => println!("error: {E}")
        }
    }
}