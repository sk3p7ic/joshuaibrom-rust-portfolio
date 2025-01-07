use actix_files as afs;
use actix_web::{get, Result as AwResult};

use crate::views::base_layout::{layout, LayoutState};
use crate::views::pages::home_view;

#[get("/")]
pub async fn index_handler() -> AwResult<maud::Markup> {
    Ok(layout(home_view::page(), LayoutState { title: None }))
}

pub fn res_dir_handler() -> afs::Files {
    afs::Files::new("/res", "./public/res")
}
