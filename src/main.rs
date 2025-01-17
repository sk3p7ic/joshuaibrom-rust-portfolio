use actix_web::{App, HttpServer};

mod handlers;
mod views {
    pub mod base_layout;
    pub mod renderers;

    pub mod components {
        pub mod header;
    }

    pub mod pages {
        pub mod home_view;
        pub mod about_view;
        pub mod projects_view;
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handlers::index_handler)
            .service(handlers::about_handler)
            .service(handlers::projects_handler)
            .service(handlers::res_dir_handler())
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
