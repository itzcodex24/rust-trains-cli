use chrono::NaiveTime;
use clap::Parser;
use dotenv::dotenv;

#[derive(Debug)]
struct Data {
    stationName: String,
    crsCode: String,
    lat: f64,
    lon: f64,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    from: String,
    #[arg(short, long)]
    to: String,
}

struct Routes {
    from: String,
    to: String,
    departure_time: String,
    from_coords: String,
    to_coords: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let args = Args::parse();
    let _from = args.from;
    let _to = args.to;

    let mut routes: Vec<Routes> = vec![];

    println!("From: {}", _from);
    println!("To: {}", _to);

    let mut from_crs = String::new();
    let mut to_crs = String::new();
    let mut from_coords = String::new();
    let mut to_coords = String::new();

    let res = reqwest::get(
        "https://raw.githubusercontent.com/davwheat/uk-railway-stations/main/stations.json",
    )
    .await?
    .json::<serde_json::Value>()
    .await?;

    res.as_array().iter().for_each(|x| {
        x.iter().for_each(|y| {
            if y["stationName"] == _from {
                from_crs = y["crsCode"].as_str().unwrap().to_owned();
                from_coords = format!("{},{}", y["lat"], y["long"])
            }
            if y["stationName"] == _to {
                to_crs = y["crsCode"].as_str().unwrap().to_owned();
                to_coords = format!("{},{}", y["lat"], y["long"])
            }
        })
    });

    if from_crs.is_empty() {
        return Err("From station not found".into());
    }

    if to_crs.is_empty() {
        return Err("To station not found".into());
    }

    let client = reqwest::Client::new();

    let rtt_username = std::env::var("RTTAPI_USERNAME").expect("No RTTAPI username found!");
    let rtt_password = std::env::var("RTTAPI_PASSWORD").expect("No RTTAPI password found!");

    let trains = client
        .get(format!(
            "https://api.rtt.io/api/v1/json/search/{}/to/{}",
            from_crs, to_crs
        ))
        .basic_auth(rtt_username, Some(rtt_password))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    if trains["services"].is_null() {
        return Err("No trains found".into());
    }

    trains["services"].as_array().iter().for_each(|x| {
        x.iter().for_each(|s| {
            let dep_time = s["locationDetail"]["gbttBookedDeparture"]
                .as_str()
                .unwrap()
                .to_owned();

            let middle_index = dep_time.len() / 2;
            let result = format!(
                "{}:{}",
                &dep_time[..middle_index],
                &dep_time[middle_index..],
            );

            let time = NaiveTime::parse_from_str(&result, "%H:%M").expect("Invalid time format");
            let today = chrono::Local::today();
            let dateTime = today.and_time(time);
            let unix_timestamp: i64 = dateTime.expect("Nope").timestamp();

            println!("Time: {}", unix_timestamp);

            get_time(from_coords.clone(), to_coords.clone(), unix_timestamp);

            routes.push(Routes {
                from: trains["location"]["name"].as_str().unwrap().to_owned(),
                to: trains["filter"]["destination"]["name"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                departure_time: result,
                from_coords: from_coords.clone(),
                to_coords: to_coords.clone(),
            });
        })
    });

    Ok(())
}

async fn get_time(from_coords: String, to_coords: String, unix_timestamp: i64) {
    let api_key = std::env::var("DISTANCE_MATRIX_API_KEY").expect("No API key found!");

    let res = reqwest::get(
 format!("https://api.distancematrix.ai/maps/api/distancematrix/json?origins={}&destinations={}&transit_mode=train&mode=transit&departure_time={}&key={}", from_coords, to_coords, unix_timestamp, api_key));

    let value = res
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .expect("Couldn't resolve JSON");

    println!("Time: {}", unix_timestamp);

    println!("{:?}", value.as_str().unwrap());
}
