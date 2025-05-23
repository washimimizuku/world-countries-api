# World Countries API

A simple REST API built with Rust and Actix-web that provides information about countries around the world.

## Features

- Get a list of all countries
- Get a specific country by its code
- Get a list of all regions
- Get all countries in a specific region

## API Endpoints

- `GET /countries` - Returns a list of all countries
- `GET /countries/{code}` - Returns a specific country by its code (e.g., US, CA)
- `GET /regions` - Returns a list of all regions
- `GET /countries/region/{region}` - Returns all countries in a specific region

## Running the API

1. Make sure you have Rust and Cargo installed
2. Clone this repository
3. Run the following command:

```
cargo run
```

The API will be available at `http://127.0.0.1:8080`

## Example Usage

```
curl http://127.0.0.1:8080/countries
curl http://127.0.0.1:8080/countries/US
curl http://127.0.0.1:8080/regions
curl http://127.0.0.1:8080/countries/region/Europe
```
