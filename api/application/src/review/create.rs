use diesel::prelude::*;
use domain::models::review::{NewReview, Review};
use infrastructure::ServerState;
use rocket::{
    response::status::{Conflict, Created},
    serde::json::Json,
    State,
};
use shared::response_models::ResponseMessage;

pub fn create_review(
    review: Json<NewReview>,
    state: &State<ServerState>,
) -> Result<Created<String>, Conflict<String>> {
    use domain::schema::reviews;

    let pooled = &mut state.db_pool.get().unwrap();

    let review = review.into_inner();

    let review_count: i64 = match pooled.transaction(|c| {
        reviews::table
            .select(reviews::all_columns)
            .filter(reviews::unit_code.eq(&review.unit_code))
            .filter(reviews::user_id.eq(&review.user_id))
            .count()
            .load(c)
    }) {
        Ok(review_count) => review_count[0],
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    };

    if review_count > 0 {
        let response = ResponseMessage {
            message: String::from("Account has already review the desired unit"),
        };

        return Err(Conflict(Some(
            serde_json::to_string(&response).expect("Return 500 internal server error."),
        )));
    }

    let review = match pooled.transaction(|c| {
        diesel::insert_into(reviews::table)
            .values(&review)
            .get_result::<Review>(c)
    }) {
        Ok(reviews) => reviews,
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    };

    Ok(Created::new("")
        .tagged_body(serde_json::to_string(&review).expect("Return 500 internal server error.")))
}
