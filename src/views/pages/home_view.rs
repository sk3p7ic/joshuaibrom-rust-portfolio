use maud::{html, Markup};

pub fn page() -> Markup {
    html! {
        main .flex.flex-col.gap-24 {
            section #featured-projects.home-section { "Featured projects" }
            section #about-me.home-section { "About me" }
        }
    }
}
