use reqwest::blocking::Client;
use std::thread;
use std::time::Duration;

async fn get_weather(api_key: &str, lat: f64, lon: f64) -> Result<(), reqwest::Error> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",
        lat, lon, api_key
    );

    let response = reqwest::get(&url).await?;
    let weather_data: serde_json::Value = response.json().await?;

    // Parse and use the weather_data as needed
    println!("{:#?}", weather_data);

    Ok(())
}

#[tokio::main]
async fn main() {
    let api_key = "YOUR_OPENWEATHERMAP_API_KEY";
    let lat = 000.000; // latitude
    let lon = 000.000; // longitude

    // Specify the interval in seconds
    let interval_seconds = 60 * 5; // 5 minutes

    loop {
        match get_weather(api_key, lat, lon).await {
            Ok(_) => println!("Weather data fetched successfully."),
            Err(err) => eprintln!("Error fetching weather data: {:?}", err),
        }

        // Sleep for the specified interval
        thread::sleep(Duration::from_secs(interval_seconds as u64));
    }
}
