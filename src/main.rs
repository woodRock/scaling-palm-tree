/// Main - main.rs
/// ==============
/// A command line interface (CLI) to display departure boards for buses in Wellington, New Zealand.
/// This make an HTTP request to the Metlink API (https://opendata.metlink.org.nz/) to retrieve times.
/// The CLI filters the departure board for a stop by service, and supports cancellations, wheelchair access.
use clap::Parser;
use serde_json::Value;

#[derive(Parser)]
#[clap(
    author = "Jesse Wood",
    version,
    about = "Simple CLI for bus departures in Wellington, NZ."
)]
struct Cli {
    stop_id: String,
    service_id: String,
    limit: Option<usize>,
}

/// Make an HTTP request to the Metlink API to retrieve a departure board for a stop.
async fn get_departure_board(stop_id: String) -> Result<Value, Box<dyn std::error::Error>> {
    let request_url = format!(
        "https://api.opendata.metlink.org.nz/v1/stop-predictions?stop_id={stop_id}",
        stop_id = stop_id,
    );
    let client = reqwest::Client::new();
    let resp = client
        .get(request_url)
        .header("x-api-key", "gyvdsui0lN1yehMHCsyIN3MejCwkIszh3NOj513P")
        .header("accept", "application/json")
        .send()
        .await?;
    match resp.status() {
        reqwest::StatusCode::OK => {
            let body: String = resp.text().await?;
            let v: Value = serde_json::from_str(&body)?;
            return Ok(v);
        }
        reqwest::StatusCode::FORBIDDEN => {
            panic!("Forbidden. Please check your API key.");
        }
        _ => {
            panic!(
                "Unknown error: {}\nPlease check if service id {} exists.",
                resp.status(),
                stop_id
            );
        }
    };
}

/// Formats the departure string for a service.
fn pretty_format(departure: &serde_json::Value, service_id: &String) -> String {
    let destination = departure["destination"]["name"].as_str().unwrap();
    let mut print_str = format!("{}\t{}\t", service_id, destination);
    let departure_time;
    // When buses are far away, the only estimate is the scheduled departure time.
    if departure["arrival"]["expected"].is_null() {
        departure_time = departure["arrival"]["aimed"].as_str().unwrap();
    // When buses are close to arrivial, the API can return expected time calculate via their GPS location.
    } else {
        departure_time = departure["arrival"]["expected"].as_str().unwrap();
    };
    // Some buses are cancelled, minutes to arrival is replaced with a cancellation notice.
    if departure["status"] == "cancelled" {
        print_str.push_str("CAN\t");
        return print_str;
    };
    // Caculate minutes to arrival in minutes.
    let now = chrono::Local::now();
    let departure_time = chrono::DateTime::parse_from_rfc3339(departure_time).unwrap();
    let minutes = departure_time.signed_duration_since(now).num_minutes();
    // Buses less than minutes away are due.
    if minutes < 0 {
        print_str.push_str("Due\t");
    // Buses more than an hour away are given in time, not hours.
    } else if minutes > 60 {
        let time_format = format!("{}\t", departure_time.format("%H:%M%p"));
        print_str.push_str(&time_format);
    // All other buses are given in minutes to arrival.
    } else {
        let minutes_format = format!("{}min\t", minutes);
        print_str.push_str(&minutes_format);
    };
    // Provide a wheelchair access indicator for accesibility.
    if departure["wheelchair_accessible"] == true {
        print_str.push('♿');
    };
    return print_str;
}

#[tokio::main]
/// A command line tool to query the Metlink API for departure times for a given service and stop.
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let json_value = get_departure_board(args.stop_id).await?;
    let departures = json_value["departures"].as_array().unwrap();
    let limit = match args.limit {
        Some(x) => x,
        None => departures.len(),
    };
    departures
        .iter()
        .filter(|departure| departure["service_id"] == args.service_id)
        .take(limit)
        .map(|departure| pretty_format(departure, &args.service_id))
        .for_each(|departure| println!("{}", departure));
    Ok(())
}
