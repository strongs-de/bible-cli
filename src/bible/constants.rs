use std::fmt;

pub static BOOKS: &'static [&str] = &["Genesis", "Exodus", "Leviticus", "Numbers", "Deuteronomy", "Joshua", "Judges", "Ruth", "1.Samuel", "2.Samuel", "1.Kings", "2.Kings", "1.Chronicles", "2.Chronicles", "Ezra", "Nehemiah", "Esther", "Job", "Psalms", "Proverbs", "Ecclesiastes", "Song of Solomon", "Isaiah", "Jeremiah", "Lamentations", "Ezekiel", "Daniel", "Hosea", "Joel", "Amos", "Obadiah", "Jonah", "Micah", "Nahum", "Habakkuk", "Zephaniah", "Haggai", "Zechariah", "Malachi", "Matthew", "Mark", "Luke", "John", "Acts", "Romans", "1.Corinthians", "2.Corinthians", "Galatians", "Ephesians", "Philippians", "Colossians", "1.Thessalonians", "2.Thessalonians", "1.Timothy", "2.Timothy", "Titus", "Philemon", "Hebrew", "James", "1.Peter", "2.Peter", "1.John", "2.John", "3.John", "Jude", "Revelation"];

#[derive(Debug, FromPrimitive)]
pub enum BookNames {
    Genesis = 1,
    Exodus = 2
}

impl fmt::Display for BookNames {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

// impl From<i32> for BookNames {
//     fn from(value: i32) -> Self {
//         BookNames
//     }
// }