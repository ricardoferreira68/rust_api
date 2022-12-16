use actix_web::*;

#[actix_web::main]
async fn main() -> Result<(), std::io:Error> {
    let api = HttpServer::new(|| {
        App::new()
        .route("/ping", web::get().to(ping))
    });

    let port = 9091;
    let api = api.bind(format!("127.0.0.1{}", port))
    .expect("Não conseguiu conectar");

    println!("Conectado com sucesso!");
    println!("http://localhost:{}/ping", port);
    
    api.run()
    .await
}

async fn ping() -> HttpResponse {
    HttpResponse::ok().body("Conectado!")
}
