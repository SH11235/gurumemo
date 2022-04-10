use axum::{http::Method, routing::get, Router};
use server::route::{
    get_yelp_data::businesses::mongo_businesses_search_handler,
    hit_yelp_api::businesses_search::businesses_search_handler,
};
use std::net::SocketAddr;
use tower_http::cors::{any, CorsLayer};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/yelp/businesses/search", get(businesses_search_handler))
        .route(
            "/mongo/businesses/search",
            get(mongo_businesses_search_handler),
        )
        .layer(
            // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
            // for more details
            CorsLayer::new()
                .allow_origin(any())
                .allow_methods(vec![Method::GET]),
        );
    let addr = SocketAddr::from(([0, 0, 0, 0], 8888));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
