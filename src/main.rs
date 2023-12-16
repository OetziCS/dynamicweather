extern crate actix_web;

use std::{env, io};
use actix_web::{middleware, web, App, HttpServer, HttpResponse};
use serde::Serialize;
use crate::weather::{get_current_weather, weathermain};

mod weather;

#[derive(Serialize)]
struct WeatherInfoResponse {
    status: i32,
    main: String,
    description: String,
}

async fn index() -> HttpResponse {
    if let Some(weather_info) = get_current_weather() {
        let json_response = WeatherInfoResponse {
            status: 200,
            main: weather_info.main,
            description: weather_info.description,
        };

        HttpResponse::Ok().json(json_response)
    } else {
        let error_response = WeatherInfoResponse {
            status: 404,
            main: "null".to_string(),
            description: "null".to_string(),
        };

        HttpResponse::NotFound().json(error_response)
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let weather_task = tokio::spawn(weathermain());

    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let server = HttpServer::new(|| {
        App::new()
          .wrap(middleware::Logger::default())
          .service(web::resource("/").route(web::get().to(index)))
        })
    .bind("0.0.0.0:9050")?
    .run();

    tokio::select! {
        _ = server => (),
        result = weather_task => {
            if let Err(e) = result {
                eprintln!("Error in weathermain: {:?}", e);
            }
        }
    }

    Ok(())
}

