use actix_web::HttpResponse;

pub fn to_login() -> HttpResponse {
    HttpResponse::Found()
        .append_header(("Location", "/login"))
        .finish()
}
