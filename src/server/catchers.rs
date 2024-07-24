use rocket::Request;

#[catch(400)]
pub fn bad_request(req: &Request) -> String {
    format!("Bad Request: {}", req.uri())
}

#[catch(401)]
pub fn unauthorized(req: &Request) -> String {
    format!("Unauthorized: {}", req.uri())
}

#[catch(403)]
pub fn forbidden(req: &Request) -> String {
    format!("Forbidden: {}", req.uri())
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Not Found: {}", req.uri())
}

#[catch(500)]
pub fn internal_error(req: &Request) -> String {
    format!("Internal Error: {}", req.uri())
}
