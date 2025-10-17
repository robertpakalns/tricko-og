use crate::generate_html::{create_html, default_html};

fn base(name: &str, desc: &str) -> String {
    create_html(name, desc, "/icons/games/cryzen.png", "#443d35")
}

// Not fully implemented yet: dynamic check for /cryzen/leaderboard/?query
pub fn html(path: &str) -> String {
    match path {
        "/cryzen" => base(
            "Cryzen.io",
            "Engage in an epic online multiplayer shooter experience. Play instantly with no downloads",
        ),
        "/cryzen/player" => base(
            "Cryzen.io Player Search",
            "Browse Cryzen.io players and view their in-game stats",
        ),
        p if p.starts_with("/cryzen/player/") => {
            match p.strip_prefix("/cryzen/player/").unwrap_or("") {
                "" => base(
                    "Cryzen.io Player Search",
                    "Browse Cryzen.io players and view their in-game stats",
                ),
                name => base(name, &format!("View stats for Cryzen.io player {name}")),
            }
        }
        "/cryzen/leaderboard" => base(
            "Cryzen.io Leaderboard Search",
            "Browse Cryzen.io leaderboards by various stats",
        ),
        "/cryzen/changelog" => base(
            "Cryzen.io Changelog",
            "View the latest updates in Cryzen.io",
        ),
        _ => default_html(),
    }
}
