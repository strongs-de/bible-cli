use crate::bible::{Bible, Verse, Chunk, StrongNumber};
use crate::traits::BibleParser;

use std::str::{self, FromStr};
use std::error::Error;
use std::cell::RefCell;
use regex::Regex;

use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};

pub type ZefaniaBible = Bible;

// pub trait ZefaniaParser {
//     fn parse_zefania(path: &'static str) -> Result<Bible, Box<dyn Error>>;
// }

fn get_attribute<T>(e: &BytesStart, attr_name: &[u8]) -> T where T: FromStr + Default, <T as FromStr>::Err: std::fmt::Debug {
    match e.attributes().find(|x| x.as_ref().unwrap().key == attr_name) {
        Some(val) => str::from_utf8(&val.unwrap().value.into_owned()).unwrap().parse::<T>().unwrap(),
        _ => Default::default()
    }
}

impl ZefaniaBible {
    fn parse_bible(path: &str) -> Result<Self, Box<dyn Error>> {
        let mut parser = Reader::from_file(path)?;
        let (mut _count, mut bnumber, mut cnumber, mut vnumber, mut depth, mut strong_number) =
            (0, 0, 0, 0, 0, -1);
        let strong_regex = Regex::new(r"(?P<strong>\d{1,4})").unwrap();
        let mut buf = Vec::new();
        let mut bible = Bible::new("ELB1905STR", "Elberfelder 1905 STR");
        loop {
            match parser.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"BIBLEBOOK" => {
                            bnumber = get_attribute::<usize>(e, b"bnumber") - 1;
                            bible.add_book(bnumber);
                            cnumber = 0;
                            depth = 0;
                        },
                        b"CHAPTER" => {
                            cnumber = get_attribute::<usize>(e, b"cnumber") - 1;
                            bible.get_book_mut(bnumber).add_chapter(cnumber);
                            vnumber = 0;
                            depth = 0;
                        },
                        b"VERS" => {
                            vnumber = get_attribute::<usize>(e, b"vnumber") - 1;
                            bible.get_chapter_mut(bnumber, cnumber)
                                .add_verse(Verse::new(vnumber));
                            _count += 1;
                            depth = 1;
                            strong_number = -1;
                        },
                        b"gr" => {
                            depth += 1;
                            let str_text: String = get_attribute(e, b"str");
                            let caps = strong_regex.captures(&str_text).unwrap();
                            strong_number = caps["strong"].parse::<i32>().unwrap();
                        },
                        _ => (),
                    }
                },

                Ok(Event::Text(e)) => {
                    if depth > 0 {
                        if strong_number > -1 {
                            bible.get_verse_mut(bnumber, cnumber, vnumber)
                                .add_strong(Chunk::new_strong(
                                    String::from(e.unescape_and_decode(&parser).unwrap().trim()),
                                    strong_number));
                        } else {
                            bible.get_verse_mut(bnumber, cnumber, vnumber)
                                .add_chunk(String::from(e.unescape_and_decode(&parser).unwrap().trim()));
                        }
                    }
                },
                Ok(Event::End(e)) => match e.name() {
                    b"gr" => depth -= 1,
                    _ => ()
                }
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", parser.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }

        Ok(bible)
    }
}

// impl ZefaniaParser for Bible {
//     fn parse_zefania(path: &'static str) -> Result<Self, Box<dyn Error>> {
impl BibleParser for ZefaniaBible {
    fn parse(path: &str, path_greek: Option<&str>) -> Result<Self, Box<dyn Error>> {
        println!("Parse translation ...");
        let mut bible: Bible = Self::parse_bible(path)?;
        if let Some(path) = path_greek {
            println!("Parse greek ...");
            let greek = Self::parse_bible(path)?;

            // Match grammar
            let mut chunk_count = 0;
            println!("Match greek with translation ...");

            // for verse in &mut bible.into_iter() {
                // for chunk in verse.chunks.iter().filter(|c| c.strong.is_some()) {
                //     let strong: &StrongNumber = chunk.strong.as_mut().unwrap();
                //     let greek_verse: Option<&Verse> = greek.get_verse(book.nr, chapter.chapter, verse.verse);
                //     if greek_verse.is_none() { continue; }
                //     let greek_verse = greek_verse.unwrap();
                //     let strong_vec: Vec<&StrongNumber> = greek_verse.chunks.iter()
                //         .filter(|c| c.strong.is_some())
                //         .map(|c| c.strong.as_ref().unwrap())
                //         .filter(|s| s.number == strong.number)
                //         .collect();
                //     if strong_vec.len() == 1 {
                //         chunk_count += 1;
                //         strong.number = strong_vec.first().unwrap().number;
                //     }
                // }
            // }

            // for book in &greek.books {
            //     println!("Book {}, Chapters {}, {}", book.nr, book.chapters.len(), book.chapters[0].verses[0].verse);
            //     let verse = bible.get_verse(book.nr, 0, 0);
            // }

            for book in &bible.books[39..66] {
                for chapter in &book.chapters {
                    for verse in &chapter.verses {
                        // Match chunks with strongs
                        for cnr in 0..verse.chunks.len() {
                            let greek_verse: Option<&Verse> = greek.get_verse(book.nr, chapter.chapter, verse.verse);
                            // println!("Book {}, Chapter {}, Verse {}", book.nr, chapter.chapter, verse.verse);
                            if greek_verse.is_none() { continue; }
                            let greek_verse = greek_verse.unwrap();

                            let chunk: &Chunk = &verse.chunks[cnr];
                            // let chunk: &Chunk = &bible.books[book.nr].chapters[chapter.chapter].verses[verse.verse].chunks[cnr];
                            // let chunk: &Chunk = &bible.get_verse(book.nr, chapter.chapter, verse.verse)
                            if chunk.strong.is_none() { continue; }
                            let strong: &RefCell<StrongNumber> = chunk.strong.as_ref().unwrap();
                            let strong_vec: Vec<&RefCell<StrongNumber>> = greek_verse.chunks.iter()
                                .filter(|c| c.strong.is_some())
                                .map(|c| c.strong.as_ref().unwrap())
                                .filter(|s| s.borrow().number == strong.borrow().number)
                                .collect();
                            if strong_vec.len() >= 1 {
                                chunk_count += 1;
                                // strong.number = strong_vec.first().unwrap().number;
                                let first = strong_vec.first().unwrap();
                                // chunk.strong = Some(Box::new(StrongNumber::new(first.number, String::from(""))))
                                chunk.strong.as_ref().unwrap().replace(StrongNumber::new(first.borrow().number, String::from("")));
                            } else {
                                println!("Could not found strong {} in greek", strong.borrow().number);
                            }
                        }
                    }
                }
            }
            println!("Searched for {} chunks", chunk_count);
        }

        Ok(bible)
    }
}