use actix_web::{get, App, HttpServer, Result as AwResult };
use maud::{html, Markup};

#[get("/")]
async fn hello() -> AwResult<Markup> {
    Ok(html! {
        html {
            body {
                h1 { "Hello, world!" }
            }
        }
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
