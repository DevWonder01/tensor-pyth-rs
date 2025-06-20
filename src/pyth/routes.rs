use actix_web::{get, web,HttpResponse};
use serde::{Serialize,Deserialize};

use crate::pyth::relay::get_price_feed;

#[derive(Deserialize, Serialize, Debug)]
struct PriceResponse {
    price: Option<i64>,
    error: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct QueryParams {
    feed: String,
}


#[get("/get-price-feed")]
async fn price_feed(
    web::Query(query): web::Query<QueryParams>,
) -> HttpResponse {

    match get_price_feed(&query.feed) {
        Ok(price) => HttpResponse::Ok().json(PriceResponse {
            price: Some(price),
            error: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(PriceResponse {
            price: None,
            error: Some(e.to_string()),
        }),
    }
}


pub fn relay_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(price_feed);
}