use serde::{Deserialize, Serialize};

// reference https://www.yelp.com/developers/documentation/v3/business_search
#[derive(Debug, Serialize, Deserialize)]
pub struct YelpBusinessSearchResult {
    pub businesses: Vec<Businesses>,
    pub total: i32,
    pub region: Region,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Businesses {
    pub categories: Vec<Categories>,
    pub coordinates: Coordinates,
    pub display_phone: String,
    pub distance: f32,
    pub id: String,
    pub alias: String,
    pub image_url: String,
    pub is_closed: bool,
    pub location: Location,
    pub name: String,
    pub phone: String,
    pub price: Option<String>, // Price level of the business. Value is one of $, $$, $$$ and $$$$.
    pub rating: f32,           // Rating for this business (value ranges from 1, 1.5, ... 4.5, 5).
    pub review_count: i32,
    pub url: String,
    pub transactions: Vec<String>, // List of Yelp transactions that the business is registered for. Current supported values are pickup, delivery and restaurant_reservation.
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Categories {
    pub alias: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub address3: Option<String>,
    pub city: String,
    pub country: String, // ISO 3166-1 alpha-2 country code of this business.
    pub display_address: Vec<String>, // Array of strings that if organized vertically give an address that is in the standard address format for the business's country.
    pub state: String, // ISO 3166-2 (with a few exceptions) state code of this business.
    pub zip_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Region {
    pub center: Center, // Center position of map area.
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Center {
    pub latitude: f32,  // Latitude position of map bounds center.
    pub longitude: f32, // Longitude position of map bounds center.
}
