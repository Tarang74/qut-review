use application::review::{create, delete, read, update};
use domain::models::review::{NewReview, Review};
use infrastructure::ServerState;
use okapi::openapi3::OpenApi;
use rocket::response::status::{Created, NotFound, Conflict};
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::{delete, get, post, put, State};
use rocket_okapi::settings::OpenApiSettings;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use shared::response_models::ResponseMessage;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![
        settings: list_review_handler,
        list_reviews_handler,
        list_user_reviews_handler,
        create_review_handler,
        approve_review_handler,
        delete_review_handler,
        update_review_handler,
    ]
}

/// Get a list of all reviews
#[openapi(tag = "Reviews")]
#[get("/?<_page>&<_limit>")]
pub fn list_reviews_handler(_page: Option<i64>, _limit: Option<i64>, state: &State<ServerState>) -> Json<Vec<Review>> {
    read::list_reviews(_page, _limit, state)
}

/// Get a review by ID
#[openapi(tag = "Reviews")]
#[get("/<review_id>")]
pub fn list_review_handler(
    review_id: i32,
    state: &State<ServerState>,
) -> Result<Json<Review>, NotFound<Json<ResponseMessage>>> {
    read::list_review(review_id, state)
}

/// Get all reviews by a user
#[openapi(tag = "Reviews")]
#[get("/user/<user_id>")]
pub fn list_user_reviews_handler(
    user_id: Uuid,
    state: &State<ServerState>,
) -> Result<Json<Vec<Review>>, NotFound<Json<ResponseMessage>>> {
    read::list_user_reviews(user_id, state)
}

/// Create a new review
#[openapi(tag = "Reviews")]
#[post("/create", format = "application/json", data = "<review>")]
pub fn create_review_handler(
    review: Json<NewReview>,
    state: &State<ServerState>,
) -> Result<Created<String>, Conflict<String>> {
    create::create_review(review, state)
}

/// Approve or disapprove a review
#[openapi(tag = "Reviews")]
#[put("/approve/<review_id>?<status>")]
pub fn approve_review_handler(
    review_id: i32,
    status: Option<bool>,
    state: &State<ServerState>,
) -> Result<Json<Review>, NotFound<Json<ResponseMessage>>> {
    update::approve_review(review_id, status.unwrap_or(true), state)
}

/// Delete a review
#[openapi(tag = "Reviews")]
#[delete("/<review_id>")]
pub fn delete_review_handler(
    review_id: i32,
    state: &State<ServerState>,
) -> Result<Json<ResponseMessage>, NotFound<Json<ResponseMessage>>> {
    delete::delete_review(review_id, state)
}

/// Update a review
#[openapi(tag = "Reviews")]
#[post("/<review_id>", format = "application/json", data = "<review>")]
pub fn update_review_handler(
    review_id: i32,
    review: Json<NewReview>,
    state: &State<ServerState>,
) -> Result<Created<String>, NotFound<Json<ResponseMessage>>> {
    update::update_review(review_id, review, state)
}
