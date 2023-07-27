use clap::Parser;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let _from = args.from;
    let _to = args.to;

    println!("From: {}", _from);
    println!("To: {}", _to);

    let mut from_crs = String::new();
    let mut to_crs = String::new();
    //
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
            }
            if y["stationName"] == _to {
                to_crs = y["crsCode"].as_str().unwrap().to_owned();
            }
        })
    });

    println!("From CRS: {}", from_crs);
    println!("To CRS: {}", to_crs);

    let client = reqwest::Client::new();

    let trains = client
        .get(format!(
            "https://api.rtt.io/api/v1/json/search/{}/to/{}",
            from_crs, to_crs
        ))
        .basic_auth(
            "rttapi_codex",
            Some("7ed3dc722fffe0bcea307bfbb72263484bdb2834"),
        )
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

            println!("The next departure is at {}", result)
        })
    });

    Ok(())
}
