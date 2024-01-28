use actix_web::dev::Service;
use actix_web::{get, middleware::Logger, HttpResponse};
use actix_web::{guard::GuardContext, post, web, App, HttpServer, Responder};
use std::sync::Mutex;
mod handlers;
mod middle_ware;
use env_logger::Env;
use futures_util::FutureExt;

// This struct represents state
struct AppState {
    app_name: String,
}

// Mutable shared state
struct AppStateWithCounter {
    counter: Mutex<i32>,
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}")
}

#[get("/counter")]
async fn counter(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();

    *counter += 1;
    format!("Request number: {counter}")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/user1")]
async fn user_by_id() -> impl Responder {
    HttpResponse::Ok().body("user 1")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// custom guard
fn verify_token(ctx: &GuardContext) -> bool {
    let auth_header = ctx.head().headers().get("authorization");

    if auth_header.is_none() {
        HttpResponse::Unauthorized().body("access denied");
        return false;
    }
    return true;
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let count = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        let user_scope = web::scope("/users").guard(verify_token).service(user_by_id);

        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i%D"))
            // NOTE: if you wrap() or wrap_fn() multiple times, the last occurrence will be
            // executed first.
            // add comperession middleware
            .wrap(actix_web::middleware::Compress::default())
            // use wrap_fn to create a small middleware
            .wrap_fn(|req, srv| {
                println!("Hi from start. You requested: {}", req.path());

                srv.call(req).map(|res| {
                    println!("Hi from response");
                    return res;
                })
            })
            .configure(handlers::config)
            .service(web::scope("/api").configure(handlers::scoped_config))
            .app_data(web::Data::new(AppState {
                app_name: String::from("actix web"),
            }))
            .app_data(count.clone())
            .service(counter)
            .service(hello)
            .service(echo)
            .service(user_scope)
            .route("hey", web::get().to(manual_hello))
            .default_service(web::route().to(handlers::handle_unauthorized))
            .default_service(web::route().to(handlers::handle_404))
            .service(handlers::person_auto)
            .service(handlers::person_manual)
            .service(handlers::form)
            .service(handlers::stream_request)
            .service(handlers::json_response)
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
