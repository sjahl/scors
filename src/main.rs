// TODO:
// 3. accept a command line argument with a date
use serde::Deserialize;

#[derive(Deserialize)]
struct Team {
    abbreviation: String,
}

#[derive(Deserialize)]
struct Competitor {
    score: String,
    team: Team,
}

#[derive(Deserialize)]
struct Competitions {
    competitors: Vec<Competitor>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Event {
    // might use these fields later?
    // id: String,
    // uid: String,
    // date: String,
    // name: String,
    // short_name: String,
    competitions: Vec<Competitions>,
}

#[derive(Deserialize)]
struct EventsList {
    events: Vec<Event>,
}

fn fetch_events() -> Result<EventsList, Box<dyn std::error::Error>> {
    let events = reqwest::blocking::get(
        "http://site.api.espn.com/apis/site/v2/sports/soccer/eng.2/scoreboard",
    )?
    .json::<EventsList>()?;

    Ok(events)
}

fn main() {
    println!("Welcome to the score checker!");
    println!("Checking the scores for the EFL championship");

    let todays_games = match fetch_events() {
        Ok(games) => games,
        Err(e) => {
            println!("Failed to fetch events: {e}");
            return;
        }
    };

    for event in &todays_games.events {
        println!(
            "{} {} - {} {}",
            event.competitions[0].competitors[0].team.abbreviation,
            event.competitions[0].competitors[0].score,
            event.competitions[0].competitors[1].score,
            event.competitions[0].competitors[1].team.abbreviation,
        );
    }
}
