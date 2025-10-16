fn create_html(title: &str, desc: &str, img: &str, color: &str) -> String {
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

fn default_html() -> String {
    create_html(
        "Tricko.pro",
        "The best stats site! Explore content for Cryzen.io, Kirka.io, Vectaria.io, and Voxiom.io, as well as the Redline and Voxtulate clients.",
        "/assets/icon.webp",
        "#ffffff",
    )
}

// Not implemented yet: dynamic cyheck for /cryzen/player/:name or /cryzen/leaderboard/?query
fn cryzen_html(path: &str) -> String {
    match path {
        "/cryzen" => create_html(
            "Cryzen.io",
            "Engage in an epic online multiplayer shooter experience. Play instantly with no downloads",
            "/icons/games/cryzen.png",
            "#443d35",
        ),
        "/cryzen/player" => create_html(
            "Cryzen.io Player Search",
            "Browse Cryzen.io players and view their in-game stats",
            "/icons/games/cryzen.png",
            "#443d35",
        ),
        "/cryzen/leaderboard" => create_html(
            "Cryzen.io Leaderboard Search",
            "Browse Cryzen.io leaderboards by various stats",
            "/icons/games/cryzen.png",
            "#443d35",
        ),
        "/cryzen/changelog" => create_html(
            "Cryzen.io Changelog",
            "View the latest updates in Cryzen.io",
            "/icons/games/cryzen.png",
            "#443d35",
        ),
        _ => default_html(),
    }
}

fn redline_html(path: &str) -> String {
    match path {
        "/redline" => create_html(
            "Redline Client",
            "Unofficial Electron client for Kirka.io",
            "/icons/redline.png",
            "#9c2220",
        ),
        "/redline/changelog" => create_html(
            "Redline Client Changelog",
            "View the latest updates in the Redline Client",
            "/icons/redline.png",
            "#9c2220",
        ),
        "/redline/privacy" => create_html(
            "Redline Client Privacy Policy",
            "Before using the Redline Client, review its Privacy Policy",
            "/icons/redline.png",
            "#9c2220",
        ),
        _ => default_html(),
    }
}

fn voxtulate_html(path: &str) -> String {
    match path {
        "/voxtulate" => create_html(
            "Voxtulate Client",
            "Unofficial Electron client for Voxiom.io",
            "/icons/voxtulate.png",
            "#7b7b7b",
        ),
        "/voxtulate/changelog" => create_html(
            "Voxtulate Client Changelog",
            "View the latest updates in the Voxtulate Client",
            "/icons/voxtulate.png",
            "#7b7b7b",
        ),
        _ => default_html(),
    }
}

pub fn get_html(path: Option<&str>) -> String {
    match path {
        Some(p) if p.starts_with("/cryzen") => cryzen_html(p),
        Some(p) if p.starts_with("/redline") => redline_html(p),
        Some(p) if p.starts_with("/voxtulate") => voxtulate_html(p),
        _ => default_html(),
    }
}
