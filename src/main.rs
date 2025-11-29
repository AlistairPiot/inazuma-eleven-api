use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Player {
    name: String,
    rarity: String,
    position: String,
    element: String,
}

#[derive(Serialize, Deserialize)]
struct Team {
    team: String,
    players: Vec<Player>,
}

#[derive(Serialize)]
struct PlayerWithTeam {
    team: String,
    name: String,
    rarity: String,
    position: String,
    element: String,
}

#[get("/players")]
async fn get_players() -> impl Responder {
    let data = std::fs::read_to_string("data.json");

    match data {
        Ok(content) => {
            let teams: Result<Vec<Team>, _> = serde_json::from_str(&content);

            match teams {
                Ok(teams_list) => {
                    let mut all_players: Vec<PlayerWithTeam> = vec![];
                    for team in teams_list {
                        for player in team.players {
                            all_players.push(PlayerWithTeam {
                                team: team.team.clone(),
                                name: player.name,
                                rarity: player.rarity,
                                position: player.position,
                                element: player.element,
                            });
                        }
                    }
                    HttpResponse::Ok().json(all_players)
                }
                Err(e) => HttpResponse::InternalServerError().body(format!("Erreur JSON: {:?}", e)),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Impossible de lire le fichier"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_players))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
