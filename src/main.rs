// TODO:
// 4. instead of relying on ordering, check for home/away status for each competitor.
use clap::Parser;

use serde::Deserialize;

// command line
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Date to fetch scores for
    #[arg(short, long)]
    date: Option<String>,

    // League to fetch scores for
    #[arg(short, long, default_value = "eng.1")]
    league: String,
}

// API Response data structures
#[derive(Deserialize, Debug)]
struct Team {
    abbreviation: String,
}

#[derive(Deserialize, Debug)]
struct Competitor {
    score: String,
    team: Team,
}

#[derive(Deserialize, Debug)]
struct Competitions {
    competitors: Vec<Competitor>,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
struct EventsList {
    events: Vec<Event>,
}

fn fetch_events(
    date_str: Option<String>,
    league: String,
) -> Result<EventsList, Box<dyn std::error::Error>> {
    let date_str = match date_str {
        Some(date_str) => format!("?dates={}", date_str),
        None => "".to_string(),
    };
    let req_url = format!(
        "http://site.api.espn.com/apis/site/v2/sports/soccer/{}/scoreboard{}",
        league, date_str
    );
    let events = reqwest::blocking::get(req_url)?.json::<EventsList>()?;

    Ok(events)
}

fn main() {
    println!("Welcome to the score checker!");

    let args = Args::parse();

    println!("You selected the league: {}", args.league);

    let todays_games = match fetch_events(args.date, args.league) {
        Ok(games) => games,
        Err(e) => {
            println!("Failed to fetch events: {e}");
            return;
        }
    };

    if todays_games.events.len() > 0 {
        for event in &todays_games.events {
            println!(
                "{} {} - {} {}",
                event.competitions[0].competitors[0].team.abbreviation,
                event.competitions[0].competitors[0].score,
                event.competitions[0].competitors[1].score,
                event.competitions[0].competitors[1].team.abbreviation,
            );
        }
    } else {
        println!("No games scheduled");
    }
}
