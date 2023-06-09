use axum::{response:: { IntoResponse, Response }, http::StatusCode};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum  Error {
  LoginFail,
  
  // -- Model errors.
  TicketDeleteFailIdNotFound { id: u64 },
}

impl IntoResponse for Error {
  fn into_response(self) -> Response {
    println!("->> INTO_RESPONSE - {self:?}");

    (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
  }
}
