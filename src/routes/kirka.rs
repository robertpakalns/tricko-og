use crate::generate_html::{create_html, default_html};

fn base(path: &str, name: &str, desc: &str) -> String {
    create_html(path, name, desc, "/icons/games/kirka.png", "#adadad")
}

pub fn html(path: &str) -> String {
    match path {
        "/kirka" => base(
            path,
            "Kirka.io",
            "A multiplayer FPS in the browser. No download required to play this io game",
        ),
        "/kirka/player" => base(
            path,
            "Kirka.io Player Search",
            "Browse Kirka.io players and view their in-game stats",
        ),
        p if p.starts_with("/kirka/player/") => {
            let id = p.strip_prefix("/kirka/player/").unwrap_or("");

            match id.len() {
                0 => base(
                    path,
                    "Kirka.io Player Search",
                    "Browse Kirka.io players and view their in-game stats",
                ),
                6 => {
                    let id_up = id.to_uppercase();
                    base(
                        path,
                        &format!("#{id_up}"),
                        &format!("View stats for Kirka.io player with ID #{id_up}"),
                    )
                }
                _ => base(
                    path,
                    &format!("Kirka.io Player"),
                    &format!("View stats for Kirka.io player with ID {id}"),
                ),
            }
        }
        "/kirka/clan" => base(
            path,
            "Kirka.io Clan Search",
            "Browse Kirka.io clans and view information about them",
        ),
        p if p.starts_with("/kirka/clan/") => match p.strip_prefix("/kirka/clan/").unwrap_or("") {
            "" => base(
                path,
                "Kirka.io Clan Search",
                "Browse Kirka.io clans and view information about them",
            ),
            name => base(
                path,
                name,
                &format!("View information about Kirka.io clan {name}"),
            ),
        },
        "/kirka/leaderboard" => base(
            path,
            "Kirka.io Leaderboard Search",
            "View Kirka.io leaderboards and explore top players and clans",
        ),
        "/kirka/leaderboard/players" => base(
            path,
            "Kirka.io Player Leaderboard",
            "View the top Kirka.io players on the leaderboard",
        ),
        "/kirka/leaderboard/clans" => base(
            path,
            "Kirka.io Clan Leaderboard",
            "View the top Kirka.io clans on the leaderboard",
        ),
        "/kirka/market" => base(
            path,
            "Kirka.io Market Search",
            "Browse the Kirka.io market and explore skins by various parameters",
        ),
        p if p.starts_with("/kirka/market") => {
            match p.strip_prefix("/kirka/market/").unwrap_or("") {
                "" => base(
                    path,
                    "Kirka.io Market Search",
                    "Browse the Kirka.io market and explore skins by various parameters",
                ),
                name => base(
                    path,
                    name,
                    &format!("View information about Kirka.io skin {name}"),
                ),
            }
        }
        "/cryzen/changelog" => base(
            path,
            "Kirka.io Changelog",
            "View the latest updates in Kirka.io",
        ),
        _ => default_html(path),
    }
}
