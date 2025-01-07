use actix_web::{get, App, HttpServer, Result as AwResult};
use comrak::{markdown_to_html, Options as CkOptions};
use maud::{html, Markup, PreEscaped, Render};

mod views {
    pub mod base_layout;
}

struct Markdown<T>(T);

impl<T: AsRef<str>> Render for Markdown<T> {
    fn render(&self) -> Markup {
        PreEscaped(markdown_to_html(self.0.as_ref(), &CkOptions::default()))
    }
}

#[get("/")]
async fn hello() -> AwResult<Markup> {
    use views::base_layout::{layout, LayoutState};
    let body = html! { h1 { (Markdown("Hello, world! _Ital_.")) } };
    Ok(layout(body, LayoutState { route: "/", title: None }))
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
