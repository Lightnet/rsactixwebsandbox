#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
// https://actix.rs/docs/response/
// 

use actix_web::{get, post, put, delete, web, Responder, HttpResponse};
use serde::Serialize;
use crate::{AppState};

#[derive(Serialize)]
struct MyObj {
  name: String,
}

#[get("/api/auth/echo")]
async fn get_echo(data: web::Data<AppState>) -> impl Responder{
  //HttpResponse::Ok().json(data.todolist_entries.lock().unwrap().to_vec())
  //HttpResponse::Ok().json()

  let obj = MyObj {
    //name: name.to_string(),
    name: "Test".into(),
  };
  HttpResponse::Ok().json(web::Json(obj))
}
// HttpResponse::Ok().body("data")
// HttpResponse::Ok().json()


pub fn config(cfg: &mut web::ServiceConfig){
  cfg
  .service(get_echo)
  //.service(get_entries)
  //.service(create_entry)
  //.service(update_entry)
  //.service(delete_entry)
  ;
}