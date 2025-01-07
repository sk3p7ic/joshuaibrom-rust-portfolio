use maud::{html, Markup};

pub struct LayoutState {
    pub route: &'static str,
    pub title: Option<&'static str>
}

pub fn layout(children: Markup, state: LayoutState) -> Markup {
    html! {
        (head(&state))
        body {
            (header())
            h1 { "Base layout." }
            (children)
            (footer())
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
