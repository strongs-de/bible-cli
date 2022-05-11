use crate::{Bible, Verse, Chunk, VerseRef};
use crate::BibleParser;

use std::str::{self, FromStr};
use std::error::Error;
use regex::Regex;

use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};

pub type ZefaniaBible = Bible;

fn get_attribute<T>(e: &BytesStart, attr_name: &[u8]) -> T where T: FromStr + Default, <T as FromStr>::Err: std::fmt::Debug {
    match e.attributes().find(|x| x.as_ref().unwrap().key == attr_name) {
        Some(val) => str::from_utf8(&val.unwrap().value.into_owned()).unwrap().parse::<T>().unwrap(),
        _ => Default::default()
    }
}

impl BibleParser for ZefaniaBible {
    fn parse(path: &str) -> Result<Self, Box<dyn Error>> {
        println!("Parse translation ...");
        let mut parser = Reader::from_file(path)?;
        let (mut _count, mut bnumber, mut cnumber, mut vnumber, mut depth, mut strong_number, mut grammar) =
            (0, 0, 0, 0, 0, -1, String::new());
        let strong_regex = Regex::new(r"(?P<strong>\d{1,4})").unwrap();
        let mut buf = Vec::new();
        let mut bible = Bible::new("Unknown", "Unknown translation");
        let mut title_content = false;
        let mut identifier_content = false;
        loop {
            match parser.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"title" => {
                            title_content = true;
                        },
                        b"identifier" => {
                            identifier_content = true;
                        },
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
                            grammar = get_attribute(e, b"rmac");
                        },
                        _ => (),
                    }
                },

                Ok(Event::Text(e)) => {
                    if title_content {
                        bible.name = e.unescape_and_decode(&parser).unwrap();
                        title_content = false;
                    }
                    if identifier_content {
                        bible.identifier = e.unescape_and_decode(&parser).unwrap();
                        identifier_content = false;
                    }
                    if depth > 0 {
                        let text = e.unescape_and_decode(&parser).unwrap();
                        let text = text.trim();
                        if !text.is_empty() {
                            if strong_number > -1 {
                                let grammar_option = if !grammar.is_empty() {
                                    Some(String::from(&grammar))
                                } else {
                                    None
                                };
                                bible.get_verse_mut(bnumber, cnumber, vnumber)
                                    .add_strong(Chunk::new_strong(
                                        String::from(text),
                                        strong_number, grammar_option));
                                bible.insert_strong_variant(strong_number, String::from(text), VerseRef::new(bnumber, cnumber, vnumber));
                            } else {
                                bible.get_verse_mut(bnumber, cnumber, vnumber)
                                    .add_chunk(String::from(e.unescape_and_decode(&parser).unwrap().trim()));
                            }
                            strong_number = -1;
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

        println!("  ... done.");

        Ok(bible)
    }
}
