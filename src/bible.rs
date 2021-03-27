use serde::{Serialize, Serializer};
use std::cell::{RefCell};
use std::collections::HashMap;

// Declarations
#[derive(Clone, Serialize)]
pub struct Bible {
    pub identifier: String,
    pub name: String,
    pub books: Vec<Book>,
    pub greek_strong_dict: HashMap<i32, StrongDictEntry>,
    pub hebrew_strong_dict: HashMap<i32, StrongDictEntry>,
}

#[derive(Clone, Serialize)]
pub struct StrongDictEntry {
    pub variants: HashMap<String, u64>,
    pub refs: Vec<VerseRef>,
}

#[derive(Clone, Serialize)]
pub struct Book {
    pub nr: usize,
    pub name: &'static str,
    pub chapters: Vec<Chapter>,
}

#[derive(Clone, Serialize)]
pub struct Chapter {
    pub chapter: usize,
    pub verses: Vec<Verse>,
}

#[derive(Clone, Serialize)]
pub struct Verse {
    pub verse: usize,
    pub chunks: Vec<Chunk>,
}

#[derive(Clone)]
pub struct VerseRef {
    pub book: usize,
    pub chapter: usize,
    pub verse: usize
}

#[derive(Clone, Serialize)]
pub struct Chunk {
    pub text: String,
    pub strong: Option<RefCell<StrongNumber>>
}

#[derive(Clone, Serialize)]
pub struct StrongNumber {
    pub number: i32,
    pub grammar: Option<String>
}

fn mut_find_or_insert<T: PartialEq>(vec: &mut Vec<T>, val: T) -> &mut T {
    if let Some(i) = (0..vec.len()).find(|&i| vec[i] == val) {
        &mut vec[i]
    } else {
        vec.push(val);
        vec.last_mut().unwrap()
    }
}

// Implementation
impl Bible {
    pub fn new(identifier: &'static str, name: &'static str) -> Bible {
        Bible { identifier: String::from(identifier), name: String::from(name), books: vec![], greek_strong_dict: HashMap::new(), hebrew_strong_dict: HashMap::new() }
    }

    pub fn add_book(&mut self, book: usize) {
        self.books.push(Book::new(book));
    }

    pub fn get_book_mut(&mut self, book: usize) -> &mut Book {
        mut_find_or_insert(&mut self.books, Book::new(book))
    }

    pub fn get_chapter_mut(&mut self, book: usize, chapter: usize) -> &mut Chapter {
        self.get_book_mut(book).get_chapter_mut(chapter)
    }

    pub fn get_verse_mut(&mut self, book: usize, chapter: usize, verse: usize) -> &mut Verse {
        self.get_book_mut(book).get_chapter_mut(chapter).get_verse_mut(verse)
    }

    pub fn get_verse_mut2(&mut self, verse: &VerseRef) -> &mut Verse {
        self.get_book_mut(verse.book).get_chapter_mut(verse.chapter).get_verse_mut(verse.verse)
    }

    pub fn get_book(&self, book: usize) -> Option<&Book> {
        // let books: Vec<Book> = self.books.iter().filter(|b| b.nr == book).map(|b| *b).collect();
        // return books.first();
        if self.books.len() > book {
            Some(&self.books[book])
        } else {
            None
        }
    }

    pub fn get_chapter(&self, book: usize, chapter: usize) -> Option<&Chapter> {
        self.get_book(book)?.get_chapter(chapter)
    }

    pub fn get_verse(&self, book: usize, chapter: usize, verse: usize) -> Option<&Verse> {
        self.get_chapter(book, chapter)?.get_verse(verse)
    }

    pub fn get_verse_ref(&self, book: usize, chapter: usize, verse: usize) -> Option<VerseRef> {
        self.get_verse(book, chapter, verse).map(|v| VerseRef::new(book, chapter, verse))
    }

    pub fn insert_strong_variant(&mut self, strong_nr: i32, text: String, verse_ref: VerseRef) {
        let entry = if verse_ref.book < 39 {
            &mut self.hebrew_strong_dict
        } else {
            &mut self.greek_strong_dict
        }.entry(strong_nr).or_insert(StrongDictEntry::new());
        let count = entry.variants.entry(text.to_lowercase()).or_insert(0);
        *count += 1;
        entry.refs.push(verse_ref);
    }
}

impl StrongDictEntry {
    pub fn new() -> StrongDictEntry {
        StrongDictEntry { refs: vec![], variants: HashMap::new() }
    }
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.nr == other.nr
    }
}

impl Book {
    pub fn new(book: usize) -> Book {
        Book { nr: book, name: "", chapters: vec![] }
    }

    pub fn add_chapter(&mut self, chapter: usize) {
        self.chapters.push(Chapter::new(chapter));
    }

    pub fn get_chapter_mut(&mut self, chapter: usize) -> &mut Chapter {
        mut_find_or_insert(&mut self.chapters, Chapter::new(chapter))
    }

    pub fn get_chapter(&self, chapter: usize) -> Option<&Chapter> {
        if self.chapters.len() > chapter {
            Some(&self.chapters[chapter])
        } else {
            None
        }
    }
}

impl PartialEq for Chapter {
    fn eq(&self, other: &Self) -> bool {
        self.chapter == other.chapter
    }
}

impl Chapter {
    pub fn new(chapter: usize) -> Chapter {
        Chapter { chapter: chapter, verses: vec![] }
    }

    pub fn add_verse(&mut self, verse: Verse) {
        self.verses.push(verse);
    }

    pub fn get_verse_mut(&mut self, verse: usize) -> &mut Verse {
        mut_find_or_insert(&mut self.verses, Verse::new(verse))
    }

    pub fn get_verse(&self, verse: usize) -> Option<&Verse> {
        if self.verses.len() > verse {
            Some(&self.verses[verse])
        } else {
            None
        }
    }
}

impl PartialEq for Verse {
    fn eq(&self, other: &Self) -> bool {
        self.verse == other.verse
    }
}

impl Verse {
    pub fn new(verse: usize) -> Verse {
        Verse { verse: verse, chunks: vec![] }
    }

    pub fn add_chunk(&mut self, text: String) {
        if !text.is_empty() {
            self.chunks.push(Chunk::new(text));
        }
    }

    pub fn add_strong(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    pub fn to_string(&self) -> String {
        self.chunks.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")
    }

    pub fn get_chunk_mut(&mut self, index: usize) -> Option<&mut Chunk> {
        if self.chunks.len() > index {
            Some(&mut self.chunks[index])
        } else {
            None
        }
    }
}

impl Chunk {
    pub fn new(text: String) -> Chunk {
        Chunk { text: text, strong: None }
    }

    pub fn new_strong(text: String, strong_number: i32, grammar: Option<String>) -> Chunk {
        Chunk { text: text, strong: Some(RefCell::new(StrongNumber::new(strong_number, grammar))) }
    }

    pub fn to_string(&self) -> String {
        self.text.clone()
    }
}

impl StrongNumber {
    pub fn new(number: i32, grammar: Option<String>) -> StrongNumber {
        StrongNumber { number: number, grammar: grammar }
    }
}

impl VerseRef {
    pub fn new(book: usize, chapter: usize, verse: usize) -> Self {
        Self { book: book, chapter: chapter, verse: verse }
    }
}

impl Serialize for VerseRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(format!("{}_{}_{}", self.book, self.chapter, self.verse).as_str())
    }
}