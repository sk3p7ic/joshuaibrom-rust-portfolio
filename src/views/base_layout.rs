use maud::{html, Markup, PreEscaped, DOCTYPE};

use crate::views::renderers;
use crate::views::components::header::header as header_component;

pub const HEADER_HEIGHT: u8 = 72;

pub struct LayoutState {
    pub title: Option<&'static str>
}

pub fn layout(children: Markup, state: LayoutState) -> Markup {
    let style = format!("padding-top: {HEADER_HEIGHT}px;");

    html! {
        (DOCTYPE)
        html {
            (head(&state))
            body .bg-poimandres-bg.text-poimandres-white.min-h-screen.flex.flex-col {
                (header(HEADER_HEIGHT))
                .grow style=(style) {
                    (children)
                }
                (footer())
            }
        }
    }
}

fn head(state: &LayoutState) -> Markup {
    html! {
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width";
            title {
                @if let Some(t) = state.title {
                    "Joshua Ibrom | " (t)
                } @else {
                    "Joshua Ibrom"
                }
            }
            (PreEscaped("
                <link rel='preconnect' href='https://fonts.googleapis.com'>
                <link rel='preconnect' href='https://fonts.gstatic.com' crossorigin>
                <link href='https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&display=swap' rel='stylesheet'>"))
            (PreEscaped("<link rel='stylesheet' href='https://cdn.jsdelivr.net/npm/bootstrap-icons@1.11.3/font/bootstrap-icons.min.css'>"))
            (renderers::Css("/res/css/globals.css"))
        }
    }
}

fn header(header_height: u8) -> Markup {
    html! {
        (header_component(header_height))
    }
}

fn footer() -> Markup {
    html! {
        footer .mt-24.py-4.text-center.border-t.border-t-poimandres-darkerGray {
            "Joshua Ibrom, 2025"
        }
    }
}
