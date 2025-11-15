// TODO:
// 4. instead of relying on ordering, check for home/away status for each competitor.
// 5. Provide a nice mapping to make League selection easier, e.g. EPL -> eng.1
// 6. pad the team names to handle 2 and 4 char abbreviations
// longer term: have a persistent TUI that updates periodically
use std::collections::HashMap;
use std::sync::LazyLock;

use clap::Parser;
use serde::Deserialize;

// this feels like unnecessary complication just go get a constant defined... maybe just deal with this
// and build the hash at runtime
static LEAGUE_ID_MAPPINGS: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from([
        ("premier", "eng.1"),
        ("efl-championship", "eng.2"),
        ("efl-1", "eng.3"),
        ("efl-2", "eng.4"),
        ("mls", "usa.1"),
        ("bundesliga", "ger.1"),
        ("bundesliga-2", "ger.2"),
        ("laliga", "esp.1"),
        ("ligue-1", "fra.1"),
        ("eredivisie", "ned.1"),
    ])
});

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

    // sport to fetch scores for
    #[arg(short, long, default_value = "soccer")]
    sport: String,
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

fn parse_league(input: &str) -> &str {
    match input {
        "premier" => "eng.1",
        "efl-championship" => "eng.2",
        "championship" => "eng.2",
        "efl-1" => "eng.3",
        "efl-2" => "eng.4",
        "bundesliga" => "ger.1",
        "bundesliga-2" => "ger.2",
        "laliga" => "esp.1",
        "ligue-1" => "fra.1",
        "eredevisie" => "ned.1",
        "mls" => "usa.1",
        "cfb" => "college-football",
        "nfl" => "nfl",
        "nhl" => "nhl",
        "mlb" => "mlb",
        _ => "eng.1", // default
    }
}

fn fetch_events(
    date_str: Option<String>,
    league: &str,
    sport: &str,
) -> Result<EventsList, Box<dyn std::error::Error>> {
    let date_str = match date_str {
        Some(date_str) => format!("?dates={date_str}"),
        None => String::new(),
    };

    let req_url = format!(
        "https://site.api.espn.com/apis/site/v2/sports/{sport}/{league}/scoreboard{date_str}"
    );

    let events = reqwest::blocking::get(req_url)?.json::<EventsList>()?;

    Ok(events)
}

fn main() {
    println!("Welcome to scors!");

    let args = Args::parse();

    println!("You selected the league: {}", args.league);

    let todays_games = match fetch_events(args.date, &args.league, &args.sport) {
        Ok(games) => games,
        Err(e) => {
            println!("Failed to fetch events: {e}");
            return;
        }
    };

    if todays_games.events.is_empty() {
        println!("No games scheduled");
    } else {
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
}
