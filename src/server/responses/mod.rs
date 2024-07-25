use serde::Serialize;

// TODO: Implement Responders
// TODO: Implement Error Handling
pub enum ApiOk<T>
where
    T: Serialize,
{
    Success(T),
    Created(T),
}

pub enum ApiError {
    Unauthorized,
    BadRequest,
    NotFound,
    InternalServerError,
}
