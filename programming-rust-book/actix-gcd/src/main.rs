use actix_web::{App, HttpResponse, HttpServer, web, get, Responder, post, FromRequest};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

pub fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(m > 0 && n > 0, "gcd requires positive integers");
    // Euclidean algorithm
    while n != 0 {
        if n < m {
            let t = n;
            n = m;
            m = t;
        }
        n = n % m;
    }
    m
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().service(get_index));

    println!("Serving on http://localhost:8080...");
    server
        .bind(("127.0.0.1", 8080))
        .expect(
            "error binding server to address",
        )
        .run()
        .await
}

#[get("/")]
async fn get_index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#" 
            <title>GCD Calculator</title> 
            <form action="/gcd" method="post"> 
                <input type="text" name="n"/> 
                <input type="text" name="m"/> 
                <button type="submit">Compute GCD</button> 
            </form> 
        "#,
    )
}

#[post("/gcd")]
async fn post_gcd() -> impl Responder {
    let params = web::Json::from_request().await.unwrap();
    println!("Received parameters: n = {}, m = {}, {}", 0, 1, params);
    // if params.n == 0 || params.m == 0 {
    //     return HttpResponse::BadRequest().content_type("text/html").body("Both numbers must be positive integers.");
    // }
    // let gcd = gcd(params.n, params.m);
    // HttpResponse::Ok().body(format!("GCD of {} and {} is {}", params.n, params.m, gcd))
    todo!("Implement GCD calculation and response");
}