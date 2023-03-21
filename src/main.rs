use reqwest::header::USER_AGENT;
use serde_json::json;

mod structs;

use structs::WeatherData;

pub async fn get_current_weather(lat: f32, lon: f32) -> Result<WeatherData, reqwest::Error> {
    let url = format!(
        "https://api.met.no/weatherapi/locationforecast/2.0/complete?lat={}&lon={}",
        lat, lon
    );
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(USER_AGENT, "weatherpoem/0.1.0")
        .send()
        .await?;

    let res = response.json::<WeatherData>().await?;

    Ok(res)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lat = 53.34;
    let lon = -6.26;
    let weather = get_current_weather(lat, lon).await?;

    let obj = json!(weather);
    println!("{}", serde_json::to_string_pretty(&obj).unwrap());

    Ok(())
}
