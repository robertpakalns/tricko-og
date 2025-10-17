use crate::generate_html::{create_html, default_html};

fn base(name: &str, desc: &str) -> String {
    create_html(name, desc, "/icons/redline.png", "#9c2220")
}

pub fn html(path: &str) -> String {
    match path {
        "/redline" => base("Redline Client", "Unofficial Electron client for Kirka.io"),
        "/redline/changelog" => base(
            "Redline Client Changelog",
            "View the latest updates in the Redline Client",
        ),
        "/redline/privacy" => base(
            "Redline Client Privacy Policy",
            "Before using the Redline Client, review its Privacy Policy",
        ),
        "/redline/discord" => base(
            "Redline Client Discord Server",
            "Join the Redline Client community on Discord",
        ),
        "/redline/github" => base(
            "Redline Client GitHub",
            "View the Redline Client source code on GitHub",
        ),
        _ => default_html(),
    }
}
