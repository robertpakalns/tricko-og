use crate::routes;

pub fn create_html(title: &str, desc: &str, img: &str, color: &str) -> String {
    format!(
        "<html>\
            <head>\
                <meta property=\"og:title\" content=\"{title}\" />\
                <meta property=\"og:description\" content=\"{desc}\" />\
                <meta property=\"og:image\" content=\"{img}\" />\
                <meta name=\"theme-color\" content=\"{color}\" />\
            </head>\
        </html>"
    )
}

pub fn default_html() -> String {
    create_html(
        "Tricko.pro",
        "The best stats site! Explore content for Cryzen.io, Kirka.io, Vectaria.io, and Voxiom.io, as well as the Redline and Voxtulate clients",
        "/assets/icon.webp",
        "#ffffff",
    )
}

// It does not handle routes with a slash at the end: e.g., /redline and /redline/ should return be the same OG
pub fn get_html(path: Option<&str>) -> String {
    match path {
        Some(p) if p.starts_with("/cryzen") => routes::cryzen::html(p),
        Some(p) if p.starts_with("/kirka") => routes::kirka::html(p),
        Some(p) if p.starts_with("/vectaria") => routes::vectaria::html(p),
        Some(p) if p.starts_with("/redline") => routes::redline::html(p),
        Some(p) if p.starts_with("/voxtulate") => routes::voxtulate::html(p),
        Some(p) => routes::tricko::html(p),
        _ => default_html(),
    }
}
