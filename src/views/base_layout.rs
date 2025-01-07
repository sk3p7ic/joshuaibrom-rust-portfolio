use maud::{html, Markup, PreEscaped, DOCTYPE};

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
            (PreEscaped("
                <link rel='preconnect' href='https://fonts.googleapis.com'>
                <link rel='preconnect' href='https://fonts.gstatic.com' crossorigin>
                <link href='https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&display=swap' rel='stylesheet'>"))
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
