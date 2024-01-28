use actix_web::{error, get, post, web, HttpResponse, Responder};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test").route(web::get().to(|| async { HttpResponse::Ok().body("test") })),
    );
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

pub async fn handle_unauthorized() -> HttpResponse {
    HttpResponse::Unauthorized().body("Unauthorized")
}

pub async fn handle_404() -> HttpResponse {
    HttpResponse::NotFound().body("Not found")
}

// deserialize a json body with serde
#[derive(Deserialize, Serialize)]
pub struct Info {
    username: String,
}

#[post("/person/auto")]
pub async fn person_auto(info: web::Json<Info>) -> actix_web::Result<String> {
    return Ok(format!("welcome {}!", info.username));
}

// manual deserialization
#[derive(Serialize, Deserialize)]
struct Obj {
    name: String,
    number: Option<i32>,
}

const MAX_SIZE: usize = 262_144; //max payload size is 256k

#[post("/person/manual")]
pub async fn person_manual(
    mut payload: web::Payload,
) -> std::result::Result<HttpResponse, actix_web::Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;

        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }

        body.extend_from_slice(&chunk);
    }

    let obj = serde_json::from_slice::<Obj>(&body)?;
    return Ok(HttpResponse::Ok().json(obj));
}

// handling a form
#[derive(Deserialize)]
pub struct FormData {
    username: String,
    number: Option<i32>,
}

#[post("/form")]
pub async fn form(form: web::Form<FormData>) -> HttpResponse {
    let num = match form.number {
        Some(n) => n,
        None => 0,
    };

    HttpResponse::Ok().body(format!("Username: {}, Number: {}", form.username, num))
}

// stream request
#[get("/stream")]
pub async fn stream_request(
    mut body: web::Payload,
) -> std::result::Result<HttpResponse, actix_web::Error> {
    let mut bytes = web::BytesMut::new();

    while let Some(item) = body.next().await {
        let item = item?;
        println!("Chunk: {:?}", &item);
        bytes.extend_from_slice(&item);
    }

    Ok(HttpResponse::Ok().finish())
}

// json response
#[derive(Serialize)]
pub struct JsonResp {
    name: String,
}

#[get("/json/response/{name}")]
pub async fn json_response(name: web::Path<String>) -> actix_web::Result<impl Responder> {
    let obj = JsonResp {
        name: name.to_string(),
    };

    Ok(web::Json(obj))
}
