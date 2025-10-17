use crate::generate_html::{create_html, default_html};

fn base(name: &str, desc: &str) -> String {
    create_html(name, desc, "/icons/games/voxiom.png", "#6b824c")
}

// Not fully implemented yet: dynamic check for /voxiom/leaderboard/parameters?query
pub fn html(path: &str) -> String {
    match path {
        "/voxiom" => base(
            "Voxiom.io",
            "A free browser game similar to Minecraft. Engage in building, crafting, digging, and much more",
        ),
        "/voxiom/player" => base(
            "Voxiom.io Player Search",
            "Browse Voxiom.io players and view their in-game stats",
        ),
        p if p.starts_with("/voxiom/player/") => {
            match p.strip_prefix("/voxiom/player/").unwrap_or("") {
                "" => base(
                    "Voxiom.io Player Search",
                    "Browse Voxiom.io players and view their in-game stats",
                ),
                name => base(name, &format!("View stats for Voxiom.io player {name}")),
            }
        }
        "/voxiom/clan" => base(
            "Voxiom.io Clan Search",
            "Browse Voxiom.io clans and view information about them",
        ),
        p if p.starts_with("/voxiom/clan/") => {
            match p.strip_prefix("/voxiom/clan/").unwrap_or("") {
                "" => base(
                    "Voxiom.io Clan Search",
                    "Browse Voxiom.io clans and view information about them",
                ),
                name => base(
                    name,
                    &format!("View information about Voxiom.io clan {name}"),
                ),
            }
        }
        "/voxiom/leaderboard" => base(
            "Voxiom.io Leaderboard Search",
            "Browse Voxiom.io leaderboards by various stats",
        ),
        "/voxiom/match" => base(
            "Voxiom.io Match Search",
            "Browse Voxiom.io Capture The Games and Battle Royale matches and view information about them",
        ),
        p if p.starts_with("/voxiom/match/ctg/") => {
            match p.strip_prefix("/voxiom/match/ctg/").unwrap_or("") {
                "" => base(
                    "Voxiom.io Match Search",
                    "Browse Voxiom.io Capture The Games and Battle Royale matches and view information about them",
                ),
                id => base(
                    "Voxiom.io Capture The Gems Match",
                    &format!(
                        "View information about the Voxiom.io Capture The Gems match with id {id}"
                    ),
                ),
            }
        }
        p if p.starts_with("/voxiom/match/br/") => {
            match p.strip_prefix("/voxiom/match/br/").unwrap_or("") {
                "" => base(
                    "Voxiom.io Match Search",
                    "Browse Voxiom.io Capture The Games and Battle Royale matches and view information about them",
                ),
                id => base(
                    "Voxiom.io Battle Royale Match",
                    &format!(
                        "View information about the Voxiom.io Battle Royale match with id {id}"
                    ),
                ),
            }
        }
        "/voxiom/market" => base(
            "Voxiom.io Market Search",
            "Browse the Voxiom.io market and explore skins by various parameters",
        ),
        p if p.starts_with("/voxiom/market") => {
            match p.strip_prefix("/voxiom/market/").unwrap_or("") {
                "" => base(
                    "Voxiom.io Market Search",
                    "Browse the Voxiom.io market and explore skins by various parameters",
                ),
                id => base(
                    "Voxiom.io Skin",
                    &format!("View information about Voxiom.io skin with ID {id}"),
                ),
            }
        }
        "/voxiom/changelog" => base(
            "Voxiom.io Changelog",
            "View the latest updates in Voxiom.io",
        ),
        _ => default_html(),
    }
}
