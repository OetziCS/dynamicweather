use reqwest::blocking::Client;
use serde::Deserialize;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use std::fs;
use std::thread;
use std::time::Duration;

// structure weather information
#[derive(Debug, Deserialize, Clone)]
pub struct WeatherInfo {
    pub main: String,
    pub description: String,
}

// structure config
#[derive(Deserialize)]
struct Config {
   latitude: f64,
   longitude: f64,
   api_key: String,
   weather_check_interval: f64,
   output_file: String,
}

pub static WEATHER_INFO: Lazy<Arc<Mutex<Option<WeatherInfo>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

// Function to retrieve the weather information
pub fn get_current_weather() -> Option<WeatherInfo> {
    WEATHER_INFO.lock().unwrap().clone()
}

// Function to fetch the API
async fn get_weather(api_key: &str, lat: f64, lon: f64) -> Result<(), reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&appid={}",
        lat, lon, api_key
    );

    let response = reqwest::get(url).await?;
    let weather_data_1 = response.text().await?;
    
    let weather_data: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&weather_data_1);
    let weather_data = weather_data.map_err(|err| {
        reqwest::Error::new(reqwest::StatusCode::INTERNAL_SERVER_ERROR, err)
    })?;

    // Extract weather information
    let weather_info = WeatherInfo {
        main: weather_data["current"]["weather"][0]["main"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        description: weather_data["current"]["weather"][0]["description"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
    };

    // (Print and) write weather information to the specified file
    //println!("{:#?}", weather_info);
    *WEATHER_INFO.lock().unwrap() = Some(weather_info);

    Ok(())
}

#[tokio::main]
async fn main() {
    let config_content = fs::read_to_string("config.json").expect("Unable to read config.json");
    let config: Config = serde_json::from_str(&config_content).expect("Error parsing config.json");

    let interval_duration = Duration::from_secs_f64(config.weather_check_interval * 60.0); // Convert minutes to seconds

    loop {
        match get_weather(&config.api_key, config.latitude, config.longitude).await {
            Ok(_) => println!("Weather data fetched successfully."),
            Err(err) => eprintln!("Error fetching weather data: {:?}", err),
        }

        // Sleep for the specified interval
        thread::sleep(interval_duration);
    }
}
