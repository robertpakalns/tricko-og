use crate::generate_html::{create_html, default_html};

fn base(name: &str, desc: &str) -> String {
    create_html(name, desc, "/logo/favicon-32x32.png", "#000000")
}

pub fn html(path: &str) -> String {
    match path {
        "/games" => base(
            "Tricko.pro Content",
            "Explore content for Cryzen.io, Kirka.io, Vectaria.io, and Voxiom.io, as well as the Redline and Voxtulate clients",
        ),
        "/changelog" => base(
            "Tricko.pro Changelog",
            "View the latest updates on Tricko.pro",
        ),
        "/bot" => base(
            "Tricko Bot",
            "The official Discord integration for Tricko.pro",
        ),
        "/bot/changelog" => base(
            "Tricko Bot Changelog",
            "View the latest updates for the Tricko Bot",
        ),
        "/terms" => base(
            "Tricko.pro Terms of Service",
            "Before using Tricko.pro, review its Terms of Service",
        ),
        "/privacy" => base(
            "Tricko.pro Privacy Policy",
            "Before using Tricko.pro, review its Privacy Policy",
        ),
        _ => default_html(),
    }
}
