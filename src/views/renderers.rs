use comrak::{markdown_to_html, Options as CkOptions};
use maud::{html, Markup, PreEscaped, Render};

pub struct Css(pub &'static str);

impl Render for Css {
    fn render(&self) -> Markup {
        html! {
            link rel="stylesheet" type="text/css" href=(self.0);
        }
    }
}

pub struct Markdown<T>(pub T);

impl<T: AsRef<str>> Render for Markdown<T> {
    fn render(&self) -> Markup {
        PreEscaped(markdown_to_html(self.0.as_ref(), &CkOptions::default()))
    }
}
