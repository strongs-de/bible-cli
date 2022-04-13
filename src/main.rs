#[macro_use] extern crate clap;
#[macro_use] extern crate num_derive;

extern crate num;

mod bible;
mod traits;
mod zefania_impl;
mod constants;
mod routes;

use clap::ArgMatches;
use log4rs::{self, config::RawConfig};
use log::info;
use actix_web::{App as ActixApp, web, middleware, HttpServer};
use routes::{info, chapter, search};

use zefania_impl::{ZefaniaBible};
use traits::BibleSearcher;
use traits::BibleParser;
use constants::BOOKS;

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
        .arg(arg!([BIBLE] "Sets the bible xml file to use").required(true))
        .arg(arg!(-v --verbose ... "Sets the level of verbosity"))
        .subcommand(
            Command::new("export")
                .about("Exports the bible into static json files")
                .arg(arg!(-o --outdir ... "Output directory")),
        )
        .subcommand(
            Command::new("search")
                .about("searches in the bible")
                .arg(arg!([TERM] "search term"))
                .arg(arg!(-t --times [time] "Execute search given times")),
        )
        .subcommand(Command::new("serve").about("serves the bible REST api"))
        .get_matches();

    let bible = matches.value_of("BIBLE").unwrap();

    if let Some(matches) = matches.subcommand_matches("search") {
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
            // let bookname: BookNames = num::FromPrimitive::from_u8(1);
            println!("  {} {},{}", BOOKS[v.book as usize - 1], v.chapter, v.verse);
        }
    } else if let Some(matches) = matches.subcommand_matches("export") {
        for path in glob(bible).expect("") {
            let bible = ZefaniaBible::parse(path.unwrap().to_str().unwrap()).unwrap();
            println!("Export json files for {} ...", bible.name);
            let outdir = String::from(matches.value_of("outdir").unwrap_or("./static"));
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

            println!("  ... done.");
        }
    } else if let Some(_) = matches.subcommand_matches("serve") {
        let bible = ZefaniaBible::parse(bible).unwrap();
        let bible = Arc::new(Mutex::new(bible));

        return HttpServer::new(move || {
            ActixApp::new()
                .app_data(web::Data::new(bible.clone()))
                // enable logger
                .wrap(middleware::Logger::default())
                .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
                .service(web::resource("/info").route(web::get().to(info)))
                .service(web::resource("/{book}/{chapter}").route(web::get().to(chapter)))
                .service(web::resource("/{search}").route(web::get().to(search)))
        })
        .bind("127.0.0.1:8000")?
        .run()
        .await
    }

    Ok(())
}
