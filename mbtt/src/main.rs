/// Main - main.rs 
/// ==============
/// A command line interface (CLI) to display departure boards for buses in Wellington, New Zealand. 
/// This make an HTTP request to the Metlink API (https://opendata.metlink.org.nz/) to retrieve times.
/// The CLI filters the departure board for a stop by service, and supports cancellations, wheelchair access.

use clap::Parser;
use serde_json::{Value};

#[derive(Parser)]
struct Cli {
    stop_id: String,
    service_id: String,
}

/// Prints a departure objective nicely to the terminal. 
fn pretty_print_departure(departure: &serde_json::Value, service_id: String) {
    let destination = departure["destination"]["name"].as_str().unwrap();
    let departure_time;
    // When buses are far away, the only estimate is the scheduled departure time.
    if departure["arrival"]["expected"].is_null() {
        departure_time = departure["arrival"]["aimed"].as_str().unwrap();
    // When buses are close to arrivial, the API can return expected time calculate via their GPS location. 
    } else {
        departure_time = departure["arrival"]["expected"].as_str().unwrap();
    }   
    // Some buses are cancelled, minutes to arrival is replaced with a cancellation notice.
    if departure["status"] == "cancelled" {
        println!("{}\t{}\t{}", service_id, destination, "CAN");
    } else {
        // Caculate minutes to arrival in minutes. 
        let now = chrono::Local::now();
        let departure_time = chrono::DateTime::parse_from_rfc3339(departure_time).unwrap();
        let minutes = departure_time.signed_duration_since(now).num_minutes();
        
        // Buses less than minutes away are due. 
        if minutes < 0 {
            if departure["wheelchair_accessible"] == true {
                println!("{}\t{}\t{}\t♿", service_id, destination, "DUE");
            } else {
                println!("{}\t{}\t{}", service_id, destination, "DUE");
            }
        // Buses more than an hour away are given in time, not hours. 
        } else if minutes > 60 {
            if departure["wheelchair_accessible"] == true {
                println!("{}\t{}\t{}\t♿", service_id, destination, departure_time.format("%H:%M%p"));
            } else {
                println!("{}\t{}\t{}", service_id, destination, departure_time.format("%H:%M%p"));
            }
        // All other buses are given in minutes to arrival. 
        } else {
            println!("{}\t{}\t{}min", service_id, destination, minutes);
            if departure["wheelchair_accessible"] == true {
                println!("{}\t{}\t{}min\t♿", service_id, destination, minutes);
            } else {
                println!("{}\t{}\t{}min", service_id, destination, minutes);
            }
        }
    }
}

#[tokio::main]
/// A command line tool to query the Metlink API for departure times for a given service and stop. 
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let request_url = format!(
        "https://api.opendata.metlink.org.nz/v1/stop-predictions?stop_id={stop_id}", 
        stop_id = args.stop_id,
    );
    let client = reqwest::Client::new();
    let resp = client.
        get(request_url)
        .header("x-api-key", "gyvdsui0lN1yehMHCsyIN3MejCwkIszh3NOj513P")
        .header("accept", "application/json")        
        .send()
        .await?
        .text()
        .await?;
    let json_value: Value = serde_json::from_str(&resp).unwrap();
    let departures = json_value["departures"].as_array().unwrap();
    departures.into_iter()
        .filter(|departure| departure["service_id"] == args.service_id.clone())
        .for_each(|departure| pretty_print_departure(departure, args.service_id.clone()));
    Ok(())
}