use actix_web::{
    web, HttpResponse
};

use std::sync::{Mutex, Arc};
use bible::{Bible, BibleSearcher, Translation};

pub async fn translations(bibles: web::Data<Arc<Mutex<Vec<Bible>>>>) -> HttpResponse {
    let translations: Vec<Translation> = bibles.lock().unwrap().iter().map(|x| x.get_translation()).collect();
    HttpResponse::Ok().json(translations)
}

pub async fn info(bibles: web::Data<Arc<Mutex<Vec<Bible>>>>, info: web::Path<String>) -> HttpResponse {
    let identifier = info.clone();
    if let Some(bible) = bibles.lock().unwrap().iter().filter(|x| x.identifier == identifier).nth(0) {
        HttpResponse::Ok().json(bible.get_translation())
    } else {
        HttpResponse::BadRequest().json(String::from("Could not find bible translation with given identifier."))
    }
}

pub async fn chapter(bibles: web::Data<Arc<Mutex<Vec<Bible>>>>, info: web::Path<(String, usize, usize)>) -> HttpResponse {
    let identifier = info.0.clone();
    let book = info.1.clone();
    let chapter = info.2.clone();
    if let Some(bible) = bibles.lock().unwrap().iter().filter(|x| x.identifier == identifier).nth(0) {
        HttpResponse::Ok().json(bible.get_chapter(book, chapter))
    } else {
        HttpResponse::BadRequest().json(String::from("Could not find bible translation with given identifier."))
    }
}

pub async fn verse(bibles: web::Data<Arc<Mutex<Vec<Bible>>>>, info: web::Path<(String, usize, usize, usize)>) -> HttpResponse {
    let identifier = info.0.clone();
    let book = info.1.clone();
    let chapter = info.2.clone();
    let verse = info.3.clone();
    if let Some(bible) = bibles.lock().unwrap().iter().filter(|x| x.identifier == identifier).nth(0) {
        HttpResponse::Ok().json(bible.get_verse(book, chapter, verse))
    } else {
        HttpResponse::BadRequest().json(String::from("Could not find bible translation with given identifier."))
    }
}

pub async fn search(bibles: web::Data<Arc<Mutex<Vec<Bible>>>>, info: web::Path<(String, String,)>) -> HttpResponse {
    let identifier = info.0.clone();
    let search = info.1.clone();
    if let Some(bible) = bibles.lock().unwrap().iter().filter(|x| x.identifier == identifier).nth(0) {
        HttpResponse::Ok().json(bible.search_parallel(&search).unwrap())
    } else {
        HttpResponse::BadRequest().json(String::from("Could not find bible translation with given identifier."))
    }
}

pub async fn greek_strongs(bibles: web::Data<Arc<Mutex<Vec<Bible>>>>, info: web::Path<(String, usize)>) -> HttpResponse {
    let identifier = info.0.clone();
    let strong = info.1.clone();
    if let Some(bible) = bibles.lock().unwrap().iter().filter(|x| x.identifier == identifier).nth(0) {
        HttpResponse::Ok().json(bible.greek_strong_dict.get(&strong))
    } else {
        HttpResponse::BadRequest().json(String::from("Could not find bible translation with given identifier."))
    }
}
