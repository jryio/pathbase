use maud::{html, Markup, DOCTYPE};

static TITLE: &str = "Pathbase";
static CHARSET: &str = "utf-8";
static STYLESHEET: &str = "stylesheet";
static STYLES: &str = "assets/styles.css";
static LANG: &str = "en";
static HTMX: &str = "https://unpkg.com/htmx.org@1.9.9";
static HTMX_HASH: &str = "sha384-QFjmbokDn2DjBjq+fM+8LUIVrAgqcNW2s0PjAxHETgRn9l4fvX31ZxDxvwQnyMOX";

pub fn base(title: Option<&str>, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang=(LANG);
        (head(title));
        body {
            (body)
        }
    }
}

pub fn head(title: Option<&str>) -> Markup {
    html! {
        head {
            title { (title.map_or(TITLE, |t| t )) }
            meta charset=(CHARSET);
            meta name="viewport" contenct="width=device-width, initial-scale=1.0";
            link rel=(STYLESHEET) href=(STYLES);
            script src=(HTMX) integrity=(HTMX_HASH) crossorigin="anonymous" {}
        }
    }
}

pub fn footer() -> Markup {
    todo!()
}
