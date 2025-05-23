use actix_web::{test, web, App};
use world_countries_api::*;

#[actix_web::test]
async fn test_all_countries() {
    // Arrange
    let app = test::init_service(
        App::new().configure(config)
    ).await;
    
    // Act
    let req = test::TestRequest::get().uri("/countries").to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert
    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    let countries: Vec<Country> = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(countries.len(), 10);
    assert!(countries.iter().any(|c| c.code == "US"));
    assert!(countries.iter().any(|c| c.code == "JP"));
}

#[actix_web::test]
async fn test_country_by_code_found() {
    // Arrange
    let app = test::init_service(
        App::new().configure(config)
    ).await;
    
    // Act
    let req = test::TestRequest::get().uri("/countries/US").to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert
    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    let country: Country = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(country.code, "US");
    assert_eq!(country.name, "United States");
    assert_eq!(country.capital, "Washington, D.C.");
}

#[actix_web::test]
async fn test_country_by_code_not_found() {
    // Arrange
    let app = test::init_service(
        App::new().configure(config)
    ).await;
    
    // Act
    let req = test::TestRequest::get().uri("/countries/XX").to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_country_by_code_case_insensitive() {
    // Arrange
    let app = test::init_service(
        App::new().configure(config)
    ).await;
    
    // Act
    let req = test::TestRequest::get().uri("/countries/us").to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert
    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    let country: Country = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(country.code, "US");
}

#[actix_web::test]
async fn test_get_regions() {
    // Arrange
    let app = test::init_service(
        App::new().configure(config)
    ).await;
    
    // Act
    let req = test::TestRequest::get().uri("/regions").to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert
    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    let regions: Vec<String> = serde_json::from_slice(&body).unwrap();
    
    assert!(regions.contains(&"Europe".to_string()));
    assert!(regions.contains(&"Asia".to_string()));
    assert!(regions.contains(&"North America".to_string()));
    assert_eq!(regions.len(), 5); // Europe, Asia, North America, Oceania, South America, Africa
}

#[actix_web::test]
async fn test_countries_by_region() {
    // Arrange
    let app = test::init_service(
        App::new().configure(config)
    ).await;
    
    // Act
    let req = test::TestRequest::get().uri("/countries/region/Europe").to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert
    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    let countries: Vec<Country> = serde_json::from_slice(&body).unwrap();
    
    assert!(countries.iter().all(|c| c.region == "Europe"));
    assert!(countries.iter().any(|c| c.code == "GB"));
    assert!(countries.iter().any(|c| c.code == "DE"));
    assert!(countries.iter().any(|c| c.code == "FR"));
}

#[actix_web::test]
async fn test_countries_by_region_case_insensitive() {
    // Arrange
    let app = test::init_service(
        App::new().configure(config)
    ).await;
    
    // Act
    let req = test::TestRequest::get().uri("/countries/region/europe").to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert
    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    let countries: Vec<Country> = serde_json::from_slice(&body).unwrap();
    
    assert!(!countries.is_empty());
    assert!(countries.iter().all(|c| c.region.to_lowercase() == "europe"));
}

#[actix_web::test]
async fn test_countries_by_region_not_found() {
    // Arrange
    let app = test::init_service(
        App::new().configure(config)
    ).await;
    
    // Act
    let req = test::TestRequest::get().uri("/countries/region/Unknown").to_request();
    let resp = test::call_service(&app, req).await;
    
    // Assert
    assert_eq!(resp.status(), 404);
}
