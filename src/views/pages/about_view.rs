use maud::{html, Markup};

pub fn page() -> Markup {
    html! {
        main .flex.flex-col.gap-24 {
            "About me"
        }
    }
}
