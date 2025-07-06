use actix_web::{post, web, App, HttpServer, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct AuditRequest {
    code: String,
}

#[post("/audit")]
async fn audit_code(req: web::Json<AuditRequest>) -> impl Responder {
    let code = &req.code;

    // Basic dummy audit logic
    let response = format!(
        "✅ Audit complete!\nCode Length: {} characters\nNo critical issues found.",
        code.len()
    );

    HttpResponse::Ok().body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 SecureEye backend running at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(audit_code)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
