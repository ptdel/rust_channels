use maud::{DOCTYPE, html, Markup, Render};


fn header(title: &str) -> Markup {
    let css = Css("style.css");
    html! {
        head {
             (DOCTYPE) (css) title { (title) }
        }
    }
}

fn footer() -> Markup {
    html! {
        footer {
            span { "rust channels" }
        }
    }
}

pub fn page(title: &str, body: Markup) -> Markup {
    html! {
        (header(title))
        h1 { (title) }
        (body)
        (footer())
    }
}

pub struct Css(&'static str);

impl Render for Css {
    fn render(&self) -> Markup {
        html! {
            link rel="stylesheet" type="text/css" href=(self.0);
        }
    }
}
