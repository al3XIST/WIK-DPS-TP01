// Importation des bibliothèques nécessaires
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde_json::{Map, Value};

// Définition d'une route HTTP pour le chemin "/ping"
#[get("/ping")]

async fn header(req: web::HttpRequest) -> impl Responder {

    // Accédez aux en-têtes HTTP de la requête
    let headers = req.headers();

    // Convertissez les en-têtes HTTP en une structure JSON
    let mut headers_json = Map::new();
    for (key, value) in headers.iter() {
        headers_json.insert(key.as_str().to_string(), Value::String(value.to_str().unwrap().to_string()));
    }

    // Renvoyez les en-têtes HTTP en tant que réponse JSON
    HttpResponse::Ok().json(headers_json)
}

// Fonction principale
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Configuration du serveur HTTP
    HttpServer::new(|| {
        App::new()
            .service(header) // Associe la route "/ping" à la fonction header
    })
    .bind("127.0.0.1:8080")? // Lie le serveur à l'adresse et au port spécifiés
    .run() // Lance le serveur 
    .await // Attend que le serveur s'arrête
}
