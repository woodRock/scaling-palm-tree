use clap::Parser;
use serde_json::{Value};

#[derive(Parser)]
struct Cli {
    stop_id: String,
    service_id: String,
}

#[tokio::main]
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

    for departure in departures {
        if departure["service_id"] == args.service_id {
            let destination = departure["destination"]["name"].as_str().unwrap();
            let departure_time;
            if departure["arrival"]["expected"].is_null() {
                departure_time = departure["arrival"]["aimed"].as_str().unwrap();
            } else {
                departure_time = departure["arrival"]["expected"].as_str().unwrap();
            }   
            if departure["status"] == "cancelled" {
                println!("{}\t{}\t{}", args.service_id, destination, "CAN");
            } else {
                let now = chrono::Local::now();
                let departure_time = chrono::DateTime::parse_from_rfc3339(departure_time).unwrap();
                let minutes = departure_time.signed_duration_since(now).num_minutes();
                if minutes < 0 {
                    if departure["wheelchair_accessible"] == true {
                        println!("{}\t{}\t{}\t♿", args.service_id, destination, "DUE");
                    } else {
                        println!("{}\t{}\t{}", args.service_id, destination, "DUE");
                    }
                } else if minutes > 60 {
                    if departure["wheelchair_accessible"] == true {
                        println!("{}\t{}\t{}\t♿", args.service_id, destination, departure_time.format("%H:%M%p"));
                    } else {
                        println!("{}\t{}\t{}", args.service_id, destination, departure_time.format("%H:%M%p"));
                    }
                } else {
                    println!("{}\t{}\t{}min", args.service_id, destination, minutes);
                    if departure["wheelchair_accessible"] == true {
                        println!("{}\t{}\t{}min\t♿", args.service_id, destination, minutes);
                    } else {
                        println!("{}\t{}\t{}min", args.service_id, destination, minutes);
                    }
                }
            }
        }
    }
    
    Ok(())
}