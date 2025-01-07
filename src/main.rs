use actix_web::{get, App, HttpServer, Result as AwResult};
use comrak::{markdown_to_html, Options as CkOptions};
use maud::{html, Markup, PreEscaped, Render};

struct Markdown<T>(T);

impl<T: AsRef<str>> Render for Markdown<T> {
    fn render(&self) -> Markup {
        PreEscaped(markdown_to_html(self.0.as_ref(), &CkOptions::default()))
    }
}

#[get("/")]
async fn hello() -> AwResult<Markup> {
    Ok(html! {
        html {
            body {
                h1 { (Markdown("Hello, world! _Ital_.")) }
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
