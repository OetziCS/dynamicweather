# Dynamicweather

This is the server-side code for my Dynamic Weather System that interacts with the OpenWeatherMap API to regulate payment for their API. You'll have to code the weathers for yourself since I don't want to open-source them. You can see how many Main Weather Types there are below.

## Setting Up

### Your Server

1. Purchase a server.
2. Purchase a domain that you can use for HTTP GET requests.
3. Clone this repository onto your server.
4. Insert your weather region (latitude, longitude).

### OpenWeather API

1. Go to [OpenWeather API](https://openweathermap.org/api) and create your API key.
2. Insert your API key on your server.
3. Run the server; if you encounter any errors, please check out our help pages linked below.

### Roblox Game

1. Insert a script in `ServerScriptService`.
2. Make an HTTP GET request to your server that you have set up above.
3. Code your different weather "types." Remember that the HTTP request returns a string, like "Rain."

If you need more help setting up the Roblox side of this project, please read this [Help Site](#).

## Weather Types

There are different weather types your GET request can return. Here, we will list those types in JSON format (JSON format below). If you don't want to script anything for every one of the IDs provided for whatever reasons, we recommend making use of the `main` table row in your code, like `if Weather.main == "Clouds" then`.

Main Types:
- Thunderstorm
- Drizzle
- Rain
- Snow
- Atmosphere
- Clear
- Clouds

See all types and the returning JSON format [here](https://openweathermap.org/weather-conditions).

## Why Use This Complicated Method When I Can Just Access the API Directly in the Game?

As your game grows, the number of game instances will increase, resulting in more API requests to the OpenWeatherAPI. If you exceed the API request limit for the free plan, you'll have to pay some money to ensure smooth operation. With this method, you'll only have to pay $2 or less (depending on your server). You can even create a server for free (e.g., on Repl, Oracle, or other platforms that offer free tiers).

