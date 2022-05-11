use actix_web::{
    web, HttpResponse
};

use std::sync::{Mutex, Arc};
use bible::Bible;
use bible::BibleSearcher;


pub async fn info(bible: web::Data<Arc<Mutex<Bible>>>) -> HttpResponse {
    HttpResponse::Ok().json(bible.lock().unwrap().get_translation())
}

pub async fn chapter(bible: web::Data<Arc<Mutex<Bible>>>, info: web::Path<(usize, usize)>) -> HttpResponse {
    let book = info.0.clone();
    let chapter = info.1.clone();
    HttpResponse::Ok().json(bible.lock().unwrap().get_chapter(book, chapter))
}

pub async fn search(bible: web::Data<Arc<Mutex<Bible>>>, info: web::Path<(String,)>) -> HttpResponse {
    let search = info.0.clone();
    HttpResponse::Ok().json(bible.lock().unwrap().search_parallel(&search).unwrap())
}