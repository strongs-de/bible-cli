#[macro_use] extern crate clap;
#[macro_use] extern crate num_derive;

extern crate num;

mod constants;
mod routes;
extern crate bible;

use flexi_logger::{Logger, opt_format, Criterion, Naming, Cleanup};
use log::info;
use actix_web::{App as ActixApp, web, middleware, HttpServer};
use routes::{info, chapter, search};

use bible::zefania_impl::{ZefaniaBible};
use bible::traits::{BibleSearcher, BibleExporter, BibleParser};
use constants::BOOKS;

use std::fs;
use std::time::Instant;
use std::sync::{Arc, Mutex};
use clap::App;
use serde_json;
use glob::glob;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Configure logging
    Logger::with_env_or_str("api=info")
       .log_to_file()
       .directory("logs")
       .rotate(Criterion::Size(1024 * 1024), Naming::Timestamps, Cleanup::KeepLogFiles(10))
       .format(opt_format)
       .start()
       .unwrap();

    info!("Started bible ...");


    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let bible = matches.value_of("BIBLE").unwrap();

    if let Some(matches) = matches.subcommand_matches("search") {
        let bible = ZefaniaBible::parse(bible).unwrap();
        let term = String::from(matches.value_of("TERM").unwrap());
        let count = value_t!(matches.value_of("times"), u32).unwrap_or(1);

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
            let outdir = String::from(matches.value_of("outdir").unwrap_or("./static"));
            bible.write(outdir).unwrap();
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
