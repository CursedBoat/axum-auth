use crate::common::structs::SessionsQuery;
use axum::{
    extract::Path, http::{header, StatusCode}, response::Response, Extension
};
use serde::Deserialize;
use serde_json::json;
use crate::common::error::return_response_error;

// struct for extracting uuid from request endpoint
#[derive(Deserialize)]
pub struct UserParams {
    pub user_id: String,
}

pub async fn get_profile_handler(
    Extension(session_query): Extension<SessionsQuery>,
    Path(params): Path<UserParams>,
) -> Response<String> {
    // return error if session userid and parameter userid does not match
    if params.user_id != session_query.user_id.unwrap() {
        return return_response_error(
            "You are not authorized to view this profile".to_string(),
            StatusCode::FORBIDDEN,
        );
    }

    // return success response if userid does match
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(
            json!({
                "success": true,
                "data": {
                    "message": format!("Logged in as user with id: {:?}", params.user_id),
                }
            })
            .to_string(),
        )
        .unwrap_or_default()
}