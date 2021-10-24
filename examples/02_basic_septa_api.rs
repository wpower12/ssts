use serde::Deserialize;  
use reqwest::Error;
use reqwest::header::USER_AGENT;


// WHY DOES THIS FEEL SO NICE TO WRITE?
// Similar to the 00 example, we want to make a
// struct that matches the json we expect from
// the api call.
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

    let station_s = "Temple U";
    let station_e = "Ft Washington";
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

    println!("       |   Departing  |   Arriving ");
    println!(" Train | {:^12} | {:^12} ", station_s, station_e);
    for train in trains.iter() {
        println!("{:6} - {:12} - {:12} ",
            train.orig_train,
            train.orig_departure_time,
            train.orig_arrival_time)
    }

    Ok(())
}

