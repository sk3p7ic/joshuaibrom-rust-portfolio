use maud::{DOCTYPE, html, Markup};

use crate::views::renderers;

pub struct LayoutState {
    pub route: &'static str,
    pub title: Option<&'static str>
}

pub fn layout(children: Markup, state: LayoutState) -> Markup {
    html! {
        (DOCTYPE)
        html {
            (head(&state))
            body {
                (header())
                h1 class="underline" { "Base layout." }
                (children)
                (footer())
            }
        }
    }
}

fn head(state: &LayoutState) -> Markup {
    html! {
        head {
            title {
                @if let Some(t) = state.title {
                    (t)
                } @else {
                    "Joshua Ibrom"
                }
            }
            (renderers::Css("/res/css/globals.css"))
        }
    }
}

fn header() -> Markup {
    html! {}
}

fn footer() -> Markup {
    html! {
        footer { "Joshua Ibrom, 2025" }
    }
}
