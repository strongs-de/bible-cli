
extern crate num;

mod routes;

use clap::ArgMatches;
use log4rs::{self, config::RawConfig};
use log::info;
use actix_web::{App as ActixApp, web, middleware, HttpServer, http};
use actix_cors::Cors;
use actix_files;
use routes::{info, chapter, search, translations, verse, greek_strongs, single_page_app};

use bible::{Bible, ZefaniaBible, BibleSearcher, BibleParser, BOOKS, Translation};

use std::fs;
use std::time::Instant;
use std::sync::{Arc, Mutex};
use clap::{arg, command, Command};
use serde_json;
use glob::glob;



#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Configure logging
    let config = String::from_utf8(include_bytes!("log4rs.yaml").to_vec()).unwrap();
    let log4rs_config: RawConfig = serde_yaml::from_str(config.as_str()).unwrap();
	log4rs::init_raw_config(log4rs_config).unwrap();

    info!("Started bible ...");

    let matches = command!()
        .arg(arg!(-v --verbose ... "Sets the level of verbosity"))
        .subcommand(
            Command::new("export")
                .about("Exports the bible into static json files")
                .arg(arg!([BIBLE] "Sets the bible xml file to use").required(true))
                .arg(arg!(-o --outdir ... "Output directory"))
        )
        .subcommand(
            Command::new("search")
                .about("searches in the bible")
                .arg(arg!([BIBLE] "Sets the bible xml file to use").required(true))
                .arg(arg!([TERM] "search term"))
                .arg(arg!(-t --times [time] "Execute search given times"))
        )
        .subcommand(
            Command::new("serve")
                .about("serves the bible REST api")
                .arg(arg!(-p --port [port] "Port to host the API (default: 8000)"))
                .arg(arg!(-f --folder [folder] "Path to the bible XML files"))
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("search") {
        let bible = matches.value_of("BIBLE").unwrap();
        let bible = ZefaniaBible::parse(bible).unwrap();
        let term = String::from(matches.value_of("TERM").unwrap());
        let count = ArgMatches::value_of_t(matches,"times").unwrap_or(1);

        println!("Search for {} {} times ...", term, count);

        let now = Instant::now();
        let mut res = vec![];
        for _ in 0..count {
            res = bible.search_parallel(&term).unwrap();
        }
        println!("Found {} occurrences parallel in {}ms (searched {} times)!", res.len(), (now.elapsed().as_millis() as f32 / count as f32), count);
        for v in res {
            println!("  {} {},{}", BOOKS[v.book as usize - 1], v.chapter, v.verse);
        }
    } else if let Some(matches) = matches.subcommand_matches("export") {
        let bible = matches.value_of("BIBLE").unwrap();
        let outdir = String::from(matches.value_of("outdir").unwrap_or("./static"));
        let mut translations: Vec<Translation> = vec![];
        for path in glob(bible).expect("") {
            let bible = ZefaniaBible::parse(path.unwrap().to_str().unwrap()).unwrap();
            translations.push(bible.get_translation());
            println!("Export json files for {} ...", bible.name);
            for book in bible.books {
                let dir = format!("{}/bibles/{}/{}", &outdir, bible.identifier, book.nr);
                fs::create_dir_all(&dir)?;
                for chapter in book.chapters {
                    // Write json files
                    let path = format!("{}/{}.json", dir, chapter.chapter);
                    let chapter_string = serde_json::to_string(&chapter)?;
                    fs::write(path, chapter_string)?;

                    let dir = format!("{}/{}", &dir, chapter.chapter);
                    fs::create_dir_all(&dir)?;
                    for verse in chapter.verses {
                        let path = format!("{}/{}.json", dir, verse.verse);
                        let verse_string = serde_json::to_string(&verse)?;
                        fs::write(path, verse_string)?;
                    }
                }
            }
            println!("  ... done.\nExport json files for the strong numbers ...");
            let dir = format!("{}/bibles/{}/greek_strongs", &outdir, bible.identifier);
            fs::create_dir_all(&dir)?;
            for (strong_number, entry) in bible.greek_strong_dict {
                let path = format!("{}/{}.json", dir, strong_number);
                let strong_string = serde_json::to_string(&entry)?;
                fs::write(path, strong_string)?;
            }

            let dir = format!("{}/bibles/{}/hebrew_strongs", &outdir, bible.identifier);
            fs::create_dir_all(&dir)?;
            for (strong_number, entry) in bible.hebrew_strong_dict {
                let path = format!("{}/{}.json", dir, strong_number);
                let strong_string = serde_json::to_string(&entry)?;
                fs::write(path, strong_string)?;
            }
        }

        println!("Export translations.json file ...");
        let path = format!("{}/bibles/translations.json", outdir);
        let translations_string = serde_json::to_string(&translations)?;
        fs::write(path, translations_string)?;

        println!("  ... done.");
    } else if let Some(serve_args) = matches.subcommand_matches("serve") {
        let port = ArgMatches::value_of_t(serve_args,"port").unwrap_or(8000);
        let folder = String::from(ArgMatches::value_of(serve_args, "folder").unwrap_or("./bibles"));
        let mut bibles: Vec<Bible> = vec![];
        for path in fs::read_dir(folder)? {
            if let Ok(path) = path {
                if path.file_name().into_string().unwrap().ends_with("xml") {
                    let bible = ZefaniaBible::parse(path.path().into_os_string().into_string().unwrap().as_str()).unwrap();
                    bibles.push(bible);
                }
            }
        }
        let bibles = Arc::new(Mutex::new(bibles));

        return HttpServer::new(move || {
            let cors = Cors::default()
              .allowed_origin_fn(|origin, _req_head| {
                  origin.as_bytes().ends_with(b".strongs.de") ||
                  origin.as_bytes().starts_with(b"http://localhost:")
              })
              .allowed_methods(vec!["GET", "OPTIONS"])
              .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT])
              .max_age(3600);

            ActixApp::new()
                .wrap(cors)
                .app_data(web::Data::new(bibles.clone()))
                // enable logger
                .wrap(middleware::Logger::default())
                .app_data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
                .route("/bibles/translations.json", web::get().to(translations))
                .route("/bibles/{identifier}/greek_strongs/{strong}.json", web::get().to(greek_strongs))
                .route("/bibles/{identifier}/{book}/{chapter}.json", web::get().to(chapter))
                .route("/bibles/{identifier}/{book}/{chapter}/{verse}.json", web::get().to(verse))
                .route("/{identifier}/info", web::get().to(info))
                .route("/{identifier}/{book}/{chapter}", web::get().to(chapter))
                .route("/{identifier}/{search}", web::get().to(search))
                .route("/", web::get().to(single_page_app))

                .route("/{book}/{chapter}", web::get().to(single_page_app))
                .route("/strongs/greek/{nr}", web::get().to(single_page_app))
                .route("/strongs/hebrew/{nr}", web::get().to(single_page_app))
                .service(actix_files::Files::new("/", "./static/").index_file("index.html"))
        })
        .bind(("0.0.0.0", port))?
        .run()
        .await
    }

    Ok(())
}
