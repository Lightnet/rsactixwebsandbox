// https://actix.rs/docs/getting-started
// https://actix.rs/docs/application
// https://github.com/actix/examples
// https://github.com/actix/examples/blob/master/auth/cookie-auth/src/main.rs


//fn main() {
//    println!("Hello, world!");
//}

use actix_web::{
    middleware,
    get, 
    web, 
    App, 
    HttpServer, 
    Responder, HttpRequest, HttpResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod todolist;
use todolist::services as todolistservices;

mod authaccess;
use authaccess::services as authservices;

struct AppState{
    todolist_entries:Mutex<Vec<ToDoListEntry>>
}

#[derive(Serialize, Deserialize, Clone)]
struct ToDoListEntry{
    id:i32,
    date:i64,
    title:String,
}

#[get("/")]
async fn index()->String{
    "This is a health check".to_string()
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[get("/collect")]
pub async fn collect(req: HttpRequest) -> impl Responder {
    //println!("Address!!!!");
    if let Some(val) = req.peer_addr() {
        println!("Address {:?}", val.ip());
    };

    HttpResponse::Ok().body("collect!")
}


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {

  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  let app_data = web::Data::new(AppState{
    todolist_entries: Mutex::new(vec![])
  });

  println!("web server init...");
  
  HttpServer::new(move || {

    App::new()
      // enable logger
      .wrap(middleware::Logger::default())
      .app_data(app_data.clone())
      .service(index)
      .service(collect)
      .configure(todolistservices::config)
      .configure(authservices::config)
      //.service(hello)//goes last due to order query url
  })
  .bind(("127.0.0.1",3000))?
  .run()
  .await // return result
  //; // not this it require return, Result need return
  //Ok(())
}

/*
#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
*/