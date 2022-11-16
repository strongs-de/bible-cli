use actix_web::{web, HttpResponse, Result};
use actix_files as fs;
use std::path::PathBuf;
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
    strongs(bibles, info, true).await
}

pub async fn hebrew_strongs(bibles: web::Data<Arc<Mutex<Vec<Bible>>>>, info: web::Path<(String, usize)>) -> HttpResponse {
    strongs(bibles, info, false).await
}

pub async fn strongs(bibles: web::Data<Arc<Mutex<Vec<Bible>>>>, info: web::Path<(String, usize)>, greek: bool) -> HttpResponse {
    let identifier = info.0.clone();
    let strong = info.1.clone();
    if let Some(bible) = bibles.lock().unwrap().iter().filter(|x| x.identifier == identifier).nth(0) {
        let strong_dict = if greek { bible.greek_strong_dict.get(&strong) } else { bible.hebrew_strong_dict.get(&strong) };
        if let Some(dict) = strong_dict {
            HttpResponse::Ok().json(dict.get_with_chunks(&bible))
        } else {
            HttpResponse::BadRequest().json(String::from("Could not find strong numbers."))
        }
    } else {
        HttpResponse::BadRequest().json(String::from("Could not find bible translation with given identifier."))
    }
}

pub async fn single_page_app() -> Result<fs::NamedFile> {
    let path: PathBuf = PathBuf::from("./static/index.html");
    Ok(fs::NamedFile::open(path)?)
}