use actix_files as afs;
use actix_web::{get, App, HttpServer, Result as AwResult};
use maud::{html, Markup};

mod views {
    pub mod base_layout;
    pub mod renderers;
}

#[get("/")]
async fn hello() -> AwResult<Markup> {
    use views::base_layout::{layout, LayoutState};
    let body = html! { h1 { (views::renderers::Markdown("Hello, world! _Ital_.")) } };
    Ok(layout(body, LayoutState { route: "/", title: None }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(afs::Files::new("/res", "./public/res"))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
