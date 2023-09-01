# Dynamicweather

This is the server side code of my Dynamic Weather System interacting with the OpenWeatherMap API. You'll have to code the weathers for yourself, as I don't wont to OpenSource them. You can see how many Weather Types there are below.

## Setting-Up

Your Server
1. Purchase a Server
2. Purchase a Domain that you can use for HTTP GET Requests
3. Clone this repro onto your Server
4. Insert your Weather Region

OpenWeather API
1. Go to https://openweathermap.org/api and create your API Key
2. Insert your API Key in your Server
3. Run the Server; if you get any Error please check out our Help pages linked below

Roblox Game
1. Insert a Script in ServerScriptService
2. Make an HTTP GET Request to your Server that you have set up above
3. Code your different Weather "Types", Remember that the HTTP Request Returns a string, like "Rain"

If you need more help setting up the Roblox Side of this project, please read this Help Site.

## Weather Types

There are some different weather Types your GET Request can return. Here, we will list those Types. If you don't wont to script anything for one of these types for whatever reasons, we recommend to make ```or``` Statements in your code, like ```ìf Weather == "Storm" or Weather == "Heavy Storm"```.
