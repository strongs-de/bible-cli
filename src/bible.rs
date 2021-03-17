use serde::{Serialize, Deserialize};
use std::cell::{Cell, RefCell};

// Declarations
#[derive(Clone, Serialize)]
pub struct Bible {
    pub identifier: &'static str,
    pub name: &'static str,
    pub books: Vec<Book>,
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

#[derive(Clone, Serialize)]
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
    pub origin_word: String
}

fn mut_find_or_insert<T: PartialEq>(vec: &mut Vec<T>, val: T) -> &mut T {
    if let Some(i) = (0..vec.len()).find(|&i| vec[i] == val) {
        &mut vec[i]
    } else {
        vec.push(val);
        vec.last_mut().unwrap()
    }
}
// fn mut_find_or_insert<T: PartialEq, F: Fn(T) -> bool>(vec: &mut Vec<T>, default: T, comparator: F) -> &mut T {
//     if let Some(i) = (0..vec.len()).find(|&i| comparator(vec[i])) {
//         &mut vec[i]
//     } else {
//         vec.push(default);
//         vec.last_mut().unwrap()
//     }
// }

// Iterator
// struct VerseIterator<'a> {
//     bible: &'a mut Bible,
//     book: usize,
//     chapter: usize,
//     verse: usize,
// }

// impl<'a> IntoIterator for &'a mut Bible {
//     type Item = &'a mut VerseRef;
//     type IntoIter = VerseIterator<'a>;

//     fn into_iter(self) -> Self::IntoIter {
//         VerseIterator {
//             bible: self,
//             book: 0,
//             chapter: 0,
//             verse: 0
//         }
//     }
// }

// impl Iterator for VerseIterator {
//     type Item = VerseRef;

//     fn next(&mut self) -> Option<Self::Item> {
//         let mut verse_ref = self.bible.get_verse_ref(self.book, self.chapter, self.verse);
//         if verse_ref.is_none() {
//             self.verse += 1;
//             verse_ref = self.bible.get_verse_ref(self.book, self.chapter, self.verse);
//         }
//         if verse_ref.is_none() {
//             self.chapter += 1;
//             self.verse = 0;
//             verse_ref = self.bible.get_verse_ref(self.book, self.chapter, self.verse);
//         }
//         if verse_ref.is_none() {
//             self.book += 1;
//             self.chapter = 0;
//             self.verse = 0;
//             verse_ref = self.bible.get_verse_ref(self.book, self.chapter, self.verse);
//         }
//         verse_ref
//     }
// }


// Implementation
impl Bible {
    pub fn new(identifier: &'static str, name: &'static str) -> Bible {
        Bible { identifier: identifier, name: name, books: vec![] }
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
        // self.chunks.iter().map(|x| x.to_string() + " ").collect::<String>()
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

    pub fn new_strong(text: String, strong_number: i32) -> Chunk {
        Chunk { text: text, strong: Some(RefCell::new(StrongNumber::new(strong_number, String::from("")))) }
    }

    pub fn to_string(&self) -> String {
        self.text.clone()
    }
}

impl StrongNumber {
    pub fn new(number: i32, origin_word: String) -> StrongNumber {
        StrongNumber { number: number, origin_word: origin_word }
    }
}

impl VerseRef {
    pub fn new(book: usize, chapter: usize, verse: usize) -> Self {
        Self { book: book, chapter: chapter, verse: verse }
    }
}