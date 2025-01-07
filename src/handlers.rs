use actix_files as afs;
use actix_web::{get, Result as AwResult};

use crate::views::base_layout::{layout, LayoutState};

#[get("/")]
pub async fn index_handler() -> AwResult<maud::Markup> {
    let body = maud::html! { h1 { "Hello, world!" } }; // FIXME: Change to include page content.
    Ok(layout(body, LayoutState { title: None }))
}

pub fn res_dir_handler() -> afs::Files {
    afs::Files::new("/res", "./public/res")
}
