extern crate clap;
use clap::{Arg, App};

use serde::Deserialize;  
use reqwest::Error;
use reqwest::header::USER_AGENT;


#[derive(Deserialize, Debug)]
struct Train {
    orig_train: String,
    orig_line: String,
    orig_departure_time: String,
    orig_arrival_time: String,
    orig_delay: String,
}


#[tokio::main]
async fn main() -> Result<(), Error> {

    let septa_app = App::new("simple septa train schedule")
        .version("0.1")
        .about("simple way to list the next arriving trains for a pair of stops on septa regional rail lines.")
        .arg(Arg::with_name("start")
            .long("start").short("s").value_name("START_STATION"))
        .arg(Arg::with_name("end")
            .long("end").short("e").value_name("END_STATION"))
        .get_matches();

    let station_s = septa_app.value_of("start").unwrap_or("Temple U");
    let station_e = septa_app.value_of("end").unwrap_or("Ft Washington");
    let n = 5;

    let request_url = format!("http://www3.septa.org/hackathon/NextToArrive/{s_start}/{s_end}/{num_trains}",
        s_start = station_s,
        s_end   = station_e,
        num_trains = n);

    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(USER_AGENT, "much user agent, such wow.")
        .send()
        .await?;

    let trains: Vec<Train> = response.json().await?;

    println!("           Departing    |   Arriving ");
    println!(" Train | {:^14} | {:^14} ", station_s, station_e);
    for train in trains.iter() {
        println!("{:6} |   {:12} |   {:12} ",
            train.orig_train,
            train.orig_departure_time,
            train.orig_arrival_time)
    }

    Ok(())
}