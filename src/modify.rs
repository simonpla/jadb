/*
TABLE
|----------|
|a|79|94725| <- ROW
|hi|6|FIELD|
|----------|

FORMATTING
t = text
n = number
d = date
\n = delimiter
|o = old field content
*/

use std::fs;
use chrono;
use std::path::Path;

pub struct Table<'a> { // Table
    pub path: &'a str, // name of db, absolute or relative path
}

pub struct Row { // Row
    pub pos: i32, // position (line) in Table
}

pub struct Field {
    pub pos: i32, // position in Row
}

impl Table<'_> {
    pub fn create(path: &str) -> i8 {
        if path.is_empty() { // can't create table without name
            println!("no table path given, cannot create table");
            return 1;
        }
        println!("creating in {}", path);
        let mut name = path.clone();
        if path.contains("/") || path.contains(r#"\"#) { // get actual name of table without rest of path
            let (_, substr) = path.rsplit_once('/').unwrap();
            name = substr;
        }
        let info = format!("jadb database\ntablename: {}\ncreated on: {}\npath: {}", name, chrono::offset::Local::now(), path); // info file content
        fs::create_dir(path).expect("Couldn't create db directory.");
        fs::write(format!("{}/{}", path, "info.jadb"), info).expect("Couldn't create info file."); // write info file
        0 // if ok return 0
    }
    pub fn write(content: &str, table: Table, row: i32) -> i8 {
        if content.is_empty() { // No need to create new row if no content
            println!("Not writing because no content given.");
            return 0;
        }
        println!("Writing {} to table path {} in Row {}", content, table.path, row);
        let path = format!("{}/{}", table.path, row); // path for row file
        if Path::new(&path).exists() { // if row already exists
            let mut con_w_form: String = String::from(""); // save contents here
            let mut con_str: Vec<&str> = content.split("\n").collect(); // split fields
            let old_row = fs::read_to_string(&path).expect("Couldn't read old Row contents"); // get old content of row
            let con_old_row: Vec<&str> = old_row.split("\n").collect(); // split fields of old content
            for i in 0..con_str.len() {
                if con_str[i] == "|o" { // if told to get old content...
                    con_str[i] = con_old_row[i]; // overwrite '|o' with old content
                }
                con_w_form.push_str(con_str[i]);
                con_w_form.push_str("\n"); // add delimiter
            }
            fs::write(&path, con_w_form).expect("Couldn't write Row");
        } else {
            fs::write(&path, content).expect("Couldn't write Row");
        }
        0 // if ok return 0
    }
}

#[derive(Debug)]
pub enum LenType {
    Fields,
    Characters,
}

impl Row {
    pub fn length(utype: LenType) -> i32 {
        dbg!(&utype);
        -1 // if not ok return -1
    }
    pub fn hash(&self) -> &'static str {
        "error" // if not ok return error
    }
}

impl Field {
    pub fn length() -> i32 {
        -1 // if not ok return -1
    }
    pub fn hash(&self) -> &'static str {
        "error" // if not ok return error
    }
}