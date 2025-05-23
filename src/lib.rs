use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use rusqlite::{params, Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

/// Represents a country with its basic information
/// 
/// This struct contains the essential information about a country including
/// its name, country code, capital city, geographical region, and currency.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema)]
pub struct Country {
    /// The full name of the country
    pub name: String,
    /// The ISO 3166-1 alpha-2 country code (two letters)
    pub code: String,
    /// The name of the capital city
    pub capital: String,
    /// The geographical region where the country is located
    pub region: String,
    /// The currency code used in the country
    pub currency: String,
}

/// Shared state for database connection
pub struct AppState {
    pub db: Mutex<Connection>,
}

/// Initialize the SQLite database
pub fn init_db() -> SqliteResult<Connection> {
    let conn = Connection::open("countries.db")?;
    
    // Create countries table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS countries (
            code TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            capital TEXT NOT NULL,
            region TEXT NOT NULL,
            currency TEXT NOT NULL
        )",
        [],
    )?;
    
    Ok(conn)
}

/// Seeds the database with initial country data
///
/// Populates the database with predefined country data if it's empty.
pub fn seed_countries(conn: &mut Connection) -> SqliteResult<()> {
    // Check if the table is empty
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM countries", [], |row| row.get(0))?;
    
    if count == 0 {
        let countries = vec![
            Country {
                name: String::from("United States"),
                code: String::from("US"),
                capital: String::from("Washington, D.C."),
                region: String::from("North America"),
                currency: String::from("USD"),
            },
            Country {
                name: String::from("Canada"),
                code: String::from("CA"),
                capital: String::from("Ottawa"),
                region: String::from("North America"),
                currency: String::from("CAD"),
            },
            Country {
                name: String::from("United Kingdom"),
                code: String::from("GB"),
                capital: String::from("London"),
                region: String::from("Europe"),
                currency: String::from("GBP"),
            },
            Country {
                name: String::from("Germany"),
                code: String::from("DE"),
                capital: String::from("Berlin"),
                region: String::from("Europe"),
                currency: String::from("EUR"),
            },
            Country {
                name: String::from("France"),
                code: String::from("FR"),
                capital: String::from("Paris"),
                region: String::from("Europe"),
                currency: String::from("EUR"),
            },
            Country {
                name: String::from("Japan"),
                code: String::from("JP"),
                capital: String::from("Tokyo"),
                region: String::from("Asia"),
                currency: String::from("JPY"),
            },
            Country {
                name: String::from("Australia"),
                code: String::from("AU"),
                capital: String::from("Canberra"),
                region: String::from("Oceania"),
                currency: String::from("AUD"),
            },
            Country {
                name: String::from("Brazil"),
                code: String::from("BR"),
                capital: String::from("Brasília"),
                region: String::from("South America"),
                currency: String::from("BRL"),
            },
            Country {
                name: String::from("South Africa"),
                code: String::from("ZA"),
                capital: String::from("Pretoria"),
                region: String::from("Africa"),
                currency: String::from("ZAR"),
            },
            Country {
                name: String::from("India"),
                code: String::from("IN"),
                capital: String::from("New Delhi"),
                region: String::from("Asia"),
                currency: String::from("INR"),
            },
        ];
        
        let tx = conn.transaction()?;
        for country in countries {
            tx.execute(
                "INSERT INTO countries (code, name, capital, region, currency) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![country.code, country.name, country.capital, country.region, country.currency],
            )?;
        }
        tx.commit()?;
    }
    
    Ok(())
}

/// Endpoint handler that returns all countries
///
/// # Route
/// `GET /countries`
///
/// # Returns
/// A JSON array containing all countries in the database
#[utoipa::path(
    get,
    path = "/countries",
    responses(
        (status = 200, description = "List of all countries", body = [Country]),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/countries")]
pub async fn all_countries(data: web::Data<AppState>) -> impl Responder {
    let conn = data.db.lock().unwrap();
    
    let mut stmt = match conn.prepare("SELECT code, name, capital, region, currency FROM countries") {
        Ok(stmt) => stmt,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    };
    
    let country_iter = match stmt.query_map([], |row| {
        Ok(Country {
            code: row.get(0)?,
            name: row.get(1)?,
            capital: row.get(2)?,
            region: row.get(3)?,
            currency: row.get(4)?,
        })
    }) {
        Ok(countries) => countries,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    };
    
    let mut countries = Vec::new();
    for country in country_iter {
        match country {
            Ok(c) => countries.push(c),
            Err(e) => return HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
        }
    }
    
    HttpResponse::Ok().json(countries)
}

/// Endpoint handler that returns a specific country by its code
///
/// # Route
/// `GET /countries/{code}`
///
/// # Parameters
/// * `path` - The country code (e.g., "US", "GB") extracted from the URL path
///
/// # Returns
/// * `200 OK` with JSON data if the country is found
/// * `404 Not Found` with an error message if the country code doesn't exist
#[utoipa::path(
    get,
    path = "/countries/{code}",
    params(
        ("code" = String, Path, description = "ISO 3166-1 alpha-2 country code")
    ),
    responses(
        (status = 200, description = "Country found", body = Country),
        (status = 404, description = "Country not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/countries/{code}")]
pub async fn country_by_code(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let code = path.into_inner().to_uppercase();
    let conn = data.db.lock().unwrap();
    
    let result = conn.query_row(
        "SELECT code, name, capital, region, currency FROM countries WHERE code = ?1",
        params![code],
        |row| {
            Ok(Country {
                code: row.get(0)?,
                name: row.get(1)?,
                capital: row.get(2)?,
                region: row.get(3)?,
                currency: row.get(4)?,
            })
        },
    );
    
    match result {
        Ok(country) => HttpResponse::Ok().json(country),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            HttpResponse::NotFound().body(format!("Country with code {} not found", code))
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

/// Endpoint handler that returns all unique geographical regions
///
/// # Route
/// `GET /regions`
///
/// # Returns
/// A JSON array containing all unique regions from the countries database
/// (e.g., "Europe", "Asia", "North America")
#[utoipa::path(
    get,
    path = "/regions",
    responses(
        (status = 200, description = "List of all geographical regions", body = [String]),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/regions")]
pub async fn get_regions(data: web::Data<AppState>) -> impl Responder {
    let conn = data.db.lock().unwrap();
    
    let mut stmt = match conn.prepare("SELECT DISTINCT region FROM countries") {
        Ok(stmt) => stmt,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    };
    
    let region_iter = match stmt.query_map([], |row| row.get::<_, String>(0)) {
        Ok(regions) => regions,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    };
    
    let mut regions = Vec::new();
    for region in region_iter {
        match region {
            Ok(r) => regions.push(r),
            Err(e) => return HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
        }
    }
    
    HttpResponse::Ok().json(regions)
}

/// Endpoint handler that returns all countries in a specific region
///
/// # Route
/// `GET /countries/region/{region}`
///
/// # Parameters
/// * `path` - The region name (e.g., "Europe", "Asia") extracted from the URL path
///
/// # Returns
/// * `200 OK` with JSON array of countries if countries are found in the region
/// * `404 Not Found` with an error message if no countries exist in the specified region
#[utoipa::path(
    get,
    path = "/countries/region/{region}",
    params(
        ("region" = String, Path, description = "Geographical region name")
    ),
    responses(
        (status = 200, description = "List of countries in the region", body = [Country]),
        (status = 404, description = "No countries found in the region"),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/countries/region/{region}")]
pub async fn countries_by_region(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let region = path.into_inner();
    let conn = data.db.lock().unwrap();
    
    let mut stmt = match conn.prepare(
        "SELECT code, name, capital, region, currency FROM countries WHERE LOWER(region) = LOWER(?1)"
    ) {
        Ok(stmt) => stmt,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    };
    
    let country_iter = match stmt.query_map(params![region], |row| {
        Ok(Country {
            code: row.get(0)?,
            name: row.get(1)?,
            capital: row.get(2)?,
            region: row.get(3)?,
            currency: row.get(4)?,
        })
    }) {
        Ok(countries) => countries,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    };
    
    let mut countries = Vec::new();
    for country in country_iter {
        match country {
            Ok(c) => countries.push(c),
            Err(e) => return HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
        }
    }
    
    if countries.is_empty() {
        HttpResponse::NotFound().body(format!("No countries found in region {}", region))
    } else {
        HttpResponse::Ok().json(countries)
    }
}

/// Endpoint handler to add a new country
///
/// # Route
/// `POST /countries`
///
/// # Request Body
/// JSON object representing a country
///
/// # Returns
/// * `201 Created` with the created country data if successful
/// * `400 Bad Request` if the country code already exists
#[utoipa::path(
    post,
    path = "/countries",
    request_body = Country,
    responses(
        (status = 201, description = "Country created successfully", body = Country),
        (status = 400, description = "Country with this code already exists"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/countries")]
pub async fn add_country(country: web::Json<Country>, data: web::Data<AppState>) -> impl Responder {
    let conn = data.db.lock().unwrap();
    let new_country = country.into_inner();
    
    // Check if country with this code already exists
    let exists: Result<bool, rusqlite::Error> = conn.query_row(
        "SELECT 1 FROM countries WHERE code = ?1",
        params![new_country.code],
        |_| Ok(true),
    ).or_else(|e| {
        if let rusqlite::Error::QueryReturnedNoRows = e {
            Ok(false)
        } else {
            Err(e)
        }
    });
    
    match exists {
        Ok(true) => {
            return HttpResponse::BadRequest()
                .body(format!("Country with code {} already exists", new_country.code));
        },
        Ok(false) => {},
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Database error: {}", e));
        }
    }
    
    // Add the new country
    let result = conn.execute(
        "INSERT INTO countries (code, name, capital, region, currency) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            new_country.code,
            new_country.name,
            new_country.capital,
            new_country.region,
            new_country.currency
        ],
    );
    
    match result {
        Ok(_) => HttpResponse::Created().json(new_country),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

/// Endpoint handler to update an existing country
///
/// # Route
/// `PUT /countries/{code}`
///
/// # Parameters
/// * `path` - The country code (e.g., "US", "GB") extracted from the URL path
///
/// # Request Body
/// JSON object representing the updated country data
///
/// # Returns
/// * `200 OK` with the updated country data if successful
/// * `404 Not Found` if the country code doesn't exist
#[utoipa::path(
    put,
    path = "/countries/{code}",
    params(
        ("code" = String, Path, description = "ISO 3166-1 alpha-2 country code")
    ),
    request_body = Country,
    responses(
        (status = 200, description = "Country updated successfully", body = Country),
        (status = 404, description = "Country not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[put("/countries/{code}")]
pub async fn update_country(
    path: web::Path<String>,
    country: web::Json<Country>,
    data: web::Data<AppState>
) -> impl Responder {
    let code = path.into_inner().to_uppercase();
    let conn = data.db.lock().unwrap();
    let updated_country = country.into_inner();
    
    // Update the country
    let result = conn.execute(
        "UPDATE countries SET name = ?1, capital = ?2, region = ?3, currency = ?4 WHERE code = ?5",
        params![
            updated_country.name,
            updated_country.capital,
            updated_country.region,
            updated_country.currency,
            code
        ],
    );
    
    match result {
        Ok(rows) if rows > 0 => {
            let country_with_code = Country {
                code,
                name: updated_country.name,
                capital: updated_country.capital,
                region: updated_country.region,
                currency: updated_country.currency,
            };
            HttpResponse::Ok().json(country_with_code)
        },
        Ok(_) => HttpResponse::NotFound().body(format!("Country with code {} not found", code)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

/// Endpoint handler to delete a country
///
/// # Route
/// `DELETE /countries/{code}`
///
/// # Parameters
/// * `path` - The country code (e.g., "US", "GB") extracted from the URL path
///
/// # Returns
/// * `204 No Content` if the country was successfully deleted
/// * `404 Not Found` if the country code doesn't exist
#[utoipa::path(
    delete,
    path = "/countries/{code}",
    params(
        ("code" = String, Path, description = "ISO 3166-1 alpha-2 country code")
    ),
    responses(
        (status = 204, description = "Country deleted successfully"),
        (status = 404, description = "Country not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[delete("/countries/{code}")]
pub async fn delete_country(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let code = path.into_inner().to_uppercase();
    let conn = data.db.lock().unwrap();
    
    // Delete the country
    let result = conn.execute("DELETE FROM countries WHERE code = ?1", params![code]);
    
    match result {
        Ok(rows) if rows > 0 => HttpResponse::NoContent().finish(),
        Ok(_) => HttpResponse::NotFound().body(format!("Country with code {} not found", code)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

/// Configures the web service by registering all API endpoints
///
/// This function is used in the main application to set up all the routes.
///
/// # Parameters
/// * `cfg` - Service configuration object provided by Actix web
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(all_countries)
       .service(country_by_code)
       .service(get_regions)
       .service(countries_by_region)
       .service(add_country)
       .service(update_country)
       .service(delete_country);
}

/// API documentation with OpenAPI
#[derive(OpenApi)]
#[openapi(
    paths(
        all_countries,
        country_by_code,
        get_regions,
        countries_by_region,
        add_country,
        update_country,
        delete_country
    ),
    components(
        schemas(Country)
    ),
    tags(
        (name = "World Countries API", description = "API for accessing country information")
    ),
    info(
        title = "World Countries API",
        version = "1.0.0",
        description = "REST API providing information about countries around the world",
        contact(
            name = "API Support",
            email = "support@example.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    )
)]
pub struct ApiDoc;

/// Configure the API documentation and Swagger UI
pub fn configure_api_docs(cfg: &mut web::ServiceConfig) {
    let openapi = ApiDoc::openapi();
    
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}")
            .url("/api-docs/openapi.json", openapi)
    );
}
