use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

// Re-export the module from lib.rs
pub use world_countries_api::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting World Countries API server at http://127.0.0.1:8080");
    println!("API documentation available at http://127.0.0.1:8080/swagger-ui/");
    
    // Initialize the database
    let mut conn = match init_db() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to initialize database: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Database initialization failed"));
        }
    };
    
    // Seed the database with initial data
    if let Err(e) = seed_countries(&mut conn) {
        eprintln!("Failed to seed database: {}", e);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Database seeding failed"));
    }
    
    // Create app state with database connection
    let app_state = web::Data::new(AppState {
        db: std::sync::Mutex::new(conn),
    });
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
            
        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .configure(config)
            .configure(configure_api_docs)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
