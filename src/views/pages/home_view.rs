use maud::{html, Markup};

use crate::views::components::common;

pub fn page() -> Markup {
    html! {
        (common::hero())
        main .flex.flex-col.gap-24 {
            section #featured-projects.home-section {
                div .flex.flex-col.gap-12 {
                    (common::main_section_divider("Projects"))
                }
            }
            section #about-me.home-section {
                div .flex.flex-col.gap-8 {
                    (common::main_section_divider("About"))
                    (common::about_statement())
                }
            }
        }
    }
}
