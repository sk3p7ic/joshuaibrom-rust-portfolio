use actix_files as afs;
use actix_web::{get, Result as AwResult};

use crate::views::base_layout::{layout, LayoutState};
use crate::views::pages::{about_view, home_view, projects_view};

#[get("/")]
pub async fn index_handler() -> AwResult<maud::Markup> {
    Ok(layout(home_view::page(), LayoutState { title: None }))
}

#[get("/about")]
pub async fn about_handler() -> AwResult<maud::Markup> {
    Ok(layout(about_view::page(), LayoutState { title: Some("About") }))
}

#[get("/projects")]
pub async fn projects_handler() -> AwResult<maud::Markup> {
    Ok(layout(projects_view::page(), LayoutState { title: Some("Projects") }))
}

pub fn res_dir_handler() -> afs::Files {
    afs::Files::new("/res", "./public/res")
}
