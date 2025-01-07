use maud::{html, Markup, PreEscaped};

pub fn header(header_height: u8) -> Markup {
    let style = format!("height: {header_height}px;");
    html! {
        header #site-header.bg-poimandres-bg.transition-colors.z-50 style=(style) {
            (navigation())
        }
        script { (header_script()) }
    }
}

fn navigation() -> Markup {
    html! {
        a href="/" class="text-poimandres-brightMint text-lg lg:text-xl" title="Go home" {
            "Joshua Ibrom"
        }
        nav {
            a href="/" { "home" }
            a href="/projects" { "projects" }
            a href="/about" { "about" }
        }
    }
}

fn header_script() -> Markup {
    html! {
        (PreEscaped("
            const header = document.querySelector('#site-header');

            const defaultBg = 'bg-poimandres-bg';
            const scrolledBg = 'bg-poimandresStorm-bg/[0.99]';

            function updateHeaderClasses() {
                if (window.scrollY > 72) {
                    header.classList.add(scrolledBg);
                    header.classList.remove(defaultBg);
                } else {
                    header.classList.add(defaultBg);
                    header.classList.remove(scrolledBg);
                }
            }

            document.addEventListener('scroll', () => {
                updateHeaderClasses();
            });

            updateHeaderClasses(); // Ensure we update on page load as well

            header.querySelectorAll('nav').forEach(n => {
                n.querySelectorAll('a').forEach(a => {
                    if (a.href === window.location.href) {
                        a.classList.add('current-page');
                    }
                });
            });
        "))
    }
}
