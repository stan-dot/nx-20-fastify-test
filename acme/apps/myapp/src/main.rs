use actix_web::{App, HttpServer};
use cats::{create_cat_data, create_cat_scope};

#[actix_web::main]
async fn main() ->std::io::Result<()>{
    let data = create_cat_data();
    HttpServer::new(move || {
        App::new()
            .service(create_cat_scope(&data))
    }).bind(("127.0.0.1", 8080))?
        .run()
        .await

}
