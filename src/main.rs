extern crate actix_web;

use std::{env, io};
use actix_web::{middleware, web, App, HttpServer, HttpResponse};
use crate::weather::{get_current_weather, weathermain};

mod weather;

async fn index() -> HttpResponse {
    if let Some(weather_info) = get_current_weather() {
        HttpResponse::Ok().body(format!(
            "Current Weather: Main - {}, Description - {}",
            weather_info.main, weather_info.description
        ))
    } else {
        HttpResponse::NotFound().body("Weather information not available yet.")
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

