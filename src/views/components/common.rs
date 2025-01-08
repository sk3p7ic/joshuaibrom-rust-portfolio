use maud::{html, Markup};

pub fn hero() -> Markup {
    html! {
        div #hero {
            h2 { "Howdy, I'm" }
            h1 { "Joshua Ibrom" }
            h2 { "junior developer & coffee addict" }
            p {
                "I'm a junior "
                span { "software engineer" }
                " and digital craftsman focused on building solutions that "
                span { "enable awesome digital experiences" }
                "."
            }
            div .socials {
                a href="https://github.com/sk3p7ic" title="sk3p7ic on GitHub" target="_blank" {
                    (social_icon(SocialIcon::GitHub))
                }
                a href="https://linkedin.com/in/Joshua-Ibrom" title="Joshua Ibrom on LinkedIn" target="_blank" {
                    (social_icon(SocialIcon::LinkedIn))
                }
                a href="https://x.com/real_sk3p7ic" title="real_sk3p7ic on X" target="_blank" {
                    (social_icon(SocialIcon::X))
                }
                a href="https://blog.joshuaibrom.com" title="My blog" target="_blank" {
                    (social_icon(SocialIcon::Blog))
                }
                a href="/static/Resume.pdf" title="My resume (PDF)" target="_blank" {
                    (social_icon(SocialIcon::Document))
                }
            }
        }
    }
}

enum SocialIcon {Blog, Document, GitHub, LinkedIn, X}

fn social_icon(icon: SocialIcon) -> Markup {
    let icon = match icon {
        SocialIcon::Blog => "bi bi-book-half",
        SocialIcon::Document => "bi bi-file-earmark-fill",
        SocialIcon::GitHub => "bi bi-github",
        SocialIcon::LinkedIn => "bi bi-linkedin",
        SocialIcon::X => "bi bi-twitter-x"
    };

    html! {
        i class=(icon) style="font-size: 2rem;" {}
    }
}


pub fn main_section_divider(title: &'static str) -> Markup {
    html! {
        div .flex.flex-col.gap-1 {
            hr .border-poimandres-lowerBlue;
            h3 class="flex items-center justify-center text-2xl md:text-4xl lg:text-5xl text-poimandres-brightMint h-[72px]" { (title) }
            hr .border-poimandres-lowerBlue;
        }
    }
}
