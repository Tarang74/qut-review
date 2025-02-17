use diesel::prelude::*;
use domain::models::unit::Unit;
use infrastructure::ServerState;
use rocket::{response::status::NotFound, serde::json::Json, State};
use shared::response_models::ResponseMessage;

pub fn list_unit(
    unit_code: &str,
    state: &State<ServerState>,
) -> Result<Json<Unit>, NotFound<Json<ResponseMessage>>> {
    use domain::schema::units;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| units::table.find(unit_code).first::<Unit>(c)) {
        Ok(unit) => {
            return Ok(Json(unit));
        }
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = ResponseMessage {
                    message: (format!("Error: unit with unit code {} not found - {}", unit_code, err)),
                };

                return Err(NotFound(Json(response)));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn list_units(state: &State<ServerState>) -> Json<Vec<Unit>> {
    use domain::schema::units;

    let pooled = &mut state.db_pool.get().unwrap();

    match pooled.transaction(move |c| units::table.load::<Unit>(c)) {
        Ok(units) => Json(units),
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
