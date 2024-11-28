use std::collections::HashSet;
use std::sync::Mutex;

use actix_web::{web, Responder, get, post, HttpResponse, Scope};

pub struct Cats{
    cats:Mutex<HashSet<Cat>>,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug, serde::Serialize, serde::Deserialize)]
struct Cat{
    name:String,
    age:u8,
}


#[get("")]
async fn get_cats(data:Data<Cats>)->impl Responder{
    let cats = data.cats.lock().unwrap();
    println!("cats: {:?}", *cats);
    Json(cats.clone())
}

#[post("/add")]
async fn add_cat(cat:Json<Cat>, data:Data<Cats>)->impl Responder{
    let mut cats = data.cats.lock().unwrap();
    println!("cat: {:?}", cat);
    cats.insert(cat.into_inner());
    HttpResponse::Ok()
}

pub fn create_cat_data() -> Data<Cats>{
    Data::new(Cats{
        cats:Mutex::new(HashSet::new()),
    })
}

pub fn create_cat_scope(data: &Data<Cats>)->Scope{
    web::scope("/cats")
        .app_data(data.clone())
        .service(get_cats)
        .service(add_cat)
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
