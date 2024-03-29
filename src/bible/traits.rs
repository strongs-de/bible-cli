use crate::{Bible, VerseRef};
use std::error::Error;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::iter::Flatten;
use shellwords;

pub trait BibleParser {
    fn parse(path: &str) -> Result<Bible, Box<dyn Error>>;
}

pub trait BibleExporter {
    fn write(path: String) -> Result<(), Box<dyn Error>>;
}

pub trait BibleSearcher {
    fn search(self: &Self, search_text: &String) -> Result<Vec<VerseRef>, Box<dyn Error>>;
    fn search_parallel(self: &Self, search_text: &String) -> Result<Vec<VerseRef>, Box<dyn Error>>;
}

impl BibleSearcher for Bible {
    fn search(self: &Self, search_text: &String) -> Result<Vec<VerseRef>, Box<dyn Error>> {
        let words = shellwords::split(&search_text.to_lowercase())?;
        println!("search_text: {}, words: {:?}", search_text, &words);
        let mut res = vec![];
        for book in &self.books {
            for chapter in &book.chapters {
                for verse in &chapter.verses {
                    let mut found = true;
                    for word in &words {
                        if let None = verse.to_string().to_lowercase().find(word) {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        res.push(VerseRef::new_with_chunks(book.nr, chapter.chapter, verse.verse, verse.chunks.clone()));
                    }
                }
            }
        }
        Ok(res)
    }

    fn search_parallel(self: &Self, search_text: &String) -> Result<Vec<VerseRef>, Box<dyn Error>> {
        // let res: Vec<Vec<VerseRef>> = &self.books.par_iter().map(|book| {
        //     let mut par_res = vec![];
        //     for chapter in &book.chapters {
        //         for verse in &chapter.verses {
        //             for chunk in &verse.chunks {
        //                 if let Some(_) = chunk.text.find(search_text) {
        //                     par_res.push(VerseRef::new(book.nr, chapter.chapter, verse.verse));
        //                 }
        //             }
        //         }
        //     }
        //     par_res
        // }).collect();
        // Ok(res.concat())
        self.search(search_text)
    }
}