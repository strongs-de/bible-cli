use crate::bible::{Bible, VerseRef};
use std::error::Error;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::iter::Flatten;

pub trait BibleParser {
    fn parse(path: &str, greek_path: Option<&str>) -> Result<Bible, Box<dyn Error>>;
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
        let mut res = vec![];
        for book in &self.books {
            for chapter in &book.chapters {
                for verse in &chapter.verses {
                    for chunk in &verse.chunks {
                        if let Some(_) =chunk.text.find(search_text) {
                            res.push(VerseRef::new(book.nr, chapter.chapter, verse.verse))
                        }
                    }
                }
            }
        }
        Ok(res)
    }

    fn search_parallel(self: &Self, search_text: &String) -> Result<Vec<VerseRef>, Box<dyn Error>> {
    //     let res: Vec<Vec<VerseRef>> = self.books.par_iter().map(|book| {
    //         let mut par_res = vec![];
    //         for chapter in &book.chapters {
    //             for verse in &chapter.verses {
    //                 for chunk in &verse.chunks {
    //                     if let Some(_) = chunk.text.find(search_text) {
    //                         par_res.push(VerseRef::new(book.nr, chapter.chapter, verse.verse));
    //                     }
    //                 }
    //             }
    //         }
    //         par_res
    //     }).collect();
    //     Ok(res.concat())
        self.search(search_text)
    }
}