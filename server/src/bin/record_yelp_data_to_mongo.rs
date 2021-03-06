extern crate server;
use dotenv::dotenv;
use futures::StreamExt;
use server::database_utils::pool;
use server::domain::entity::yelp_business::Business;
use server::driver::hit_yelp_api::YelpApiDriver;
use server::hit_api_utils::error::YelpAPIAccessError;
use server::hit_api_utils::setting::MAX_TOTAL_NUM;
use server::usecase::hit_yelp_api::hit_business_search_api::{self, RequestParams};
use structopt::StructOpt;

#[derive(StructOpt)]
struct CliArgument {
    #[structopt(long)]
    term: Option<String>, // Optional. Search term, for example "food" or "restaurants". The term may also be business names, such as "Starbucks". If term is not included the endpoint will default to searching across businesses from a small number of popular categories.
    #[structopt(long)]
    location: Option<String>, // Required if either latitude or longitude is not provided. This string indicates the geographic area to be used when searching for businesses. Examples: "New York City", "NYC", "350 5th Ave, New York, NY 10118". Businesses returned in the response may not be strictly within the specified location.
    #[structopt(long)]
    latitude: Option<String>, // Required if location is not provided. Latitude of the location you want to search nearby.
    #[structopt(long)]
    longitude: Option<String>, // Required if location is not provided. Longitude of the location you want to search nearby.
    #[structopt(long)]
    radius: Option<String>, // Optional. A suggested search radius in meters. This field is used as a suggestion to the search. The actual search radius may be lower than the suggested radius in dense urban areas, and higher in regions of less business density. If the specified value is too large, a AREA_TOO_LARGE error may be returned. The max value is 40000 meters (about 25 miles).
    #[structopt(long)]
    categories: Option<String>, // Optional. Categories to filter the search results with. See the list of supported categories. The category filter can be a list of comma delimited categories. For example, "bars,french" will filter by Bars OR French. The category identifier should be used (for example "discgolf", not "Disc Golf").
    #[structopt(long)]
    locale: Option<String>, // Optional. Specify the locale into which to localize the business information. See the list of supported locales. Defaults to en_US.
    #[structopt(long)]
    limit: Option<i32>, // Optional. Number of business results to return. By default, it will return 20. Maximum is 50.
    #[structopt(long)]
    offset: Option<i32>, // Optional. Offset the list of returned business results by this amount.
    #[structopt(long)]
    sort_by: Option<String>, // Optional. Suggestion to the search algorithm that the results be sorted by one of the these modes: best_match, rating, review_count or distance. The default is best_match. Note that specifying the sort_by is a suggestion (not strictly enforced) to Yelp's search, which considers multiple input parameters to return the most relevant results. For example, the rating sort is not strictly sorted by the rating value, but by an adjusted rating value that takes into account the number of ratings, similar to a Bayesian average. This is to prevent skewing results to businesses with a single review.
    #[structopt(long)]
    price: Option<String>, // Optional. Pricing levels to filter the search result with: 1 = $, 2 = $$, 3 = $$$, 4 = $$$$. The price filter can be a list of comma delimited pricing levels. For example, "1, 2, 3" will filter the results to show the ones that are $, $$, or $$$.
    #[structopt(long)]
    open_now: Option<bool>, // Optional. Default to false. When set to true, only return the businesses open now. Notice that open_at and open_now cannot be used together.
    #[structopt(long)]
    open_at: Option<i32>, // Optional. An integer represending the Unix time in the same timezone of the search location. If specified, it will return business open at the given time. Notice that open_at and open_now cannot be used together.
    #[structopt(long)]
    attributes: Option<String>, // Optional. Try these additional filters to return specific search results!
                                // ???hot_and_new - popular businesses which recently joined Yelp
                                // ???request_a_quote - businesses which actively reply to Request a Quote inquiries
                                // ???reservation - businesses with Yelp Reservations bookings enabled on their profile page
                                // ???waitlist_reservation - businesses with Yelp Waitlist bookings enabled on their profile screen (iOS/Android)
                                // ???deals - businesses offering Yelp Deals on their profile page
                                // ???gender_neutral_restrooms - businesses which provide gender neutral restrooms
                                // ???open_to_all - businesses which are Open To All
                                // ???wheelchair_accessible - businesses which are Wheelchair Accessible
                                // You can combine multiple attributes by providing a comma separated like "attribute1,attribute2".
                                // If multiple attributes are used, only businesses that satisfy ALL attributes will be returned in search results.
                                // For example, the attributes "hot_and_new,request_a_quote" will return businesses that are Hot and New AND offer Request a Quote.
}

#[tokio::main]
async fn main() -> pool::MongoResult<()> {
    println!("*** ?????? ***");
    dotenv().ok();
    let mongo_db_client = pool::mongo_db_client().await?;
    let coll = mongo_db_client.database("yelp").collection("business");
    let arg = CliArgument::from_args();

    let params = RequestParams {
        term: arg.term,
        location: arg.location,
        latitude: arg.latitude,
        longitude: arg.longitude,
        radius: arg.radius,
        categories: arg.categories,
        locale: arg.locale,
        limit: arg.limit,
        offset: arg.offset,
        sort_by: arg.sort_by,
        price: arg.price,
        open_now: arg.open_now,
        open_at: arg.open_at,
        attributes: arg.attributes,
    };

    let yelp_api_driver = YelpApiDriver::new();
    let res = hit_business_search_api::execute(yelp_api_driver, params.clone())
        .await
        .map_err(|e| YelpAPIAccessError::InternalErrorWithMessage(e.to_string()))
        .unwrap();
    if res.total > MAX_TOTAL_NUM {
        println!("total: {} > {}", res.total, MAX_TOTAL_NUM);
        println!("{} ??????????????????????????????????????????????????????????????????????????????radius?????????????????????????????????????????????", MAX_TOTAL_NUM);
        println!("{:?} ", params);
    }
    println!("{} ????????????????????????", res.businesses.len());
    let insert_data = res
        .businesses
        .iter()
        .map(|business| {
            return Business {
                categories: business.categories.clone(),
                coordinates: business.coordinates.clone(),
                longitude_latitude: [
                    business.coordinates.longitude,
                    business.coordinates.latitude,
                ]
                .to_vec(),
                display_phone: business.display_phone.clone(),
                distance: business.distance,
                id: business.id.clone(),
                alias: business.alias.clone(),
                image_url: business.image_url.clone(),
                is_closed: business.is_closed.clone(),
                location: business.location.clone(),
                name: business.name.clone(),
                phone: business.phone.clone(),
                price: business.price.clone(),
                rating: business.rating,
                review_count: business.review_count,
                url: business.url.clone(),
                transactions: business.transactions.clone(),
            };
        })
        .collect::<Vec<Business>>();
    coll.insert_many(insert_data, None).await?;

    // DB??????????????????
    let cursor = coll.find(None, None).await?;
    let collections_count = cursor.count().await;
    println!("DB ????????????: {}", collections_count);
    println!("*** ?????? ***");
    Ok(())
}
