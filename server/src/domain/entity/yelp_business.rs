use super::yelp_businesses_search_api::{Categories, Coordinates, Location};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Business {
    pub categories: Vec<Categories>,
    pub coordinates: Coordinates,
    pub longitude_latitude: Vec<f32>,
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
