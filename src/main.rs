use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::fs::{File, OpenOptions};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(remote = "Self")]
struct Book {
    name: String,
    author: String,
    pages: u16,
    year: u16
}

impl Serialize for Book {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
     S: Serializer , {
        let mut book = serializer.serialize_struct("Book", 4)?;
        book.serialize_field("name", &self.name)?;
        book.serialize_field("author", &self.author)?;
        book.serialize_field("pages", &self.pages)?;
        book.serialize_field("year", &self.year)?;
        book.end()
    }
}

fn encode_to_json(bk: &Book) -> String {
    serde_json::to_string(bk).unwrap()
}


#[derive(serde::Serialize, serde::Deserialize)]
struct LibraryBook {
    name: String,
    author: String,
    pages: u16,
    year: u16
}

fn str_to_json(js: &str) -> LibraryBook {
    serde_json::from_str(js).unwrap()
}

fn json_to_str(bk: &LibraryBook) -> String {
    serde_json::to_string(bk).unwrap()
}

fn book_from_file(filepath: &str) -> LibraryBook {
    let file = File::open(filepath).unwrap();
    serde_json::from_reader(file).unwrap()
}

fn book_to_file(filepath: &str, bk: &LibraryBook) {
    let file = OpenOptions::new().append(true).open(filepath).unwrap();
    serde_json::to_writer(file, &bk);
}

fn main() {
    let hp = Book {
        name: String::from("Harry Potter"),
        author: String::from("JK Rowling"),
        pages: 700,
        year: 2000
    };

    let hp_json = encode_to_json(&hp);

    println!("{:?}", hp_json);

    let rawdata = r#"{
        "name": "Harry Potter",
        "author": "JK Rowling",
        "pages": 700,
        "year": 2000
    }"#;

    let d: LibraryBook = str_to_json(rawdata);

    let encoded = json_to_str(&d);

    println!("{:?}", encoded);

    let tmpfilename = String::from("/tmp/sample.json");
    book_to_file(&tmpfilename, &d);

    let librarybook = book_from_file(&tmpfilename);
    println!("{:?}", json_to_str(&librarybook));

}
