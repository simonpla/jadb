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

// filesystem access
use std::fs;
use chrono;
use std::path::Path;

// hashing
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone)]
pub struct Table<'a> { // Table
    pub path: &'a str, // name of db, absolute or relative path
}

#[derive(Copy, Clone, Hash)]
pub struct Row { // Row
    pub pos: i32, // position (line) in Table
}

#[derive(Copy, Clone, Hash)]
pub struct Field {
    pub pos: i32, // position in Row
}

impl Table<'_> {
    pub fn create(&self) -> i8 {
        if self.path.is_empty() { // can't create table without name
            println!("no table path given, cannot create table");
            return 1;
        }
        if Path::new(self.path).exists() {
            println!("table already exists at given path");
            return 1;
        }
        println!("creating in {}", self.path);
        let mut name = self.path.clone();
        if self.path.contains("/") || self.path.contains(r#"\"#) { // get actual name of table without rest of path
            let (_, substr) = self.path.rsplit_once('/').unwrap();
            name = substr;
        }
        let info = format!("jadb database\ntablename: {}\ncreated on: {}\npath: {}", name, chrono::offset::Local::now(), self.path); // info file content
        fs::create_dir(self.path).expect("Couldn't create db directory.");
        fs::write(format!("{}/{}", self.path, "info.jadb"), info).expect("Couldn't create info file."); // write info file
        0 // if ok return 0
    }
    pub fn write(&self, content: &str, row: Row) -> i8 {
        if content.is_empty() { // No need to create new row if no content
            println!("Not writing because no content given.");
            return 1;
        }
        println!("Writing {} to table path {} in Row {}", content, self.path, row.pos);
        let path = format!("{}/{}", self.path, row.pos); // path for row file
        let mut hasher = DefaultHasher::new();
        let mut hash_con: Vec<String> = vec![]; // save hashes here
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
                if i != con_str.len()-1 {
                    con_w_form.push_str("\n"); // add delimiter
                }
                con_str[i].hash(&mut hasher);
                hash_con.push(hasher.finish().to_string());
            }
            fs::write(&path, con_w_form).expect("Couldn't write Row");
            fs::write(format!("{}_hash", &path), hash_con.join("\n")).expect("Couldn't write hash of row");
        } else {
            let con_str: Vec<&str> = content.split("\n").collect(); // split fields
            for i in 0..con_str.len() {
                con_str[i].hash(&mut hasher);
                hash_con.push(hasher.finish().to_string());
            }
            fs::write(&path, content).expect("Couldn't write Row");
            fs::write(format!("{}_hash", &path), hash_con.join("\n")).expect("Couldn't write hash of row");
        }
        0 // if ok return 0
    }
    pub fn read(&self, row: Row) -> Vec<String> {
        let content = fs::read_to_string(format!("{}/{}", self.path, row.pos)).expect("Couldn't read row");
        let con_str: Vec<String> = content.split("\n").map(String::from).collect();
        con_str
    }
    pub fn delete(&self) -> i8 {
        let info_path = format!("{}/{}", self.path, "info.jadb"); // create path of info file
        if Path::new(&info_path).exists() { // use it to check if table exists
            fs::remove_dir_all(self.path).expect("Couldn't delete database files."); // delete folder
            return 0;
        } else {
            println!("Table doesn't exist at {}", self.path);
            return 1;
        }
    }
}

#[derive(PartialEq)]
pub enum LenType {
    Fields,
    Characters,
}

impl Row {
    pub fn length(&self, table: Table, utype: LenType) -> i32 {
        let con = table.read(*self);
        let mut len: i32 = 0;
        if utype == LenType::Characters {
            for i in 0..con.len() {
                len += con[i].len() as i32;
            }
        } else {
            len = con.len() as i32;
        }
        len
    }
    pub fn shash(&self, table: Table) -> u64 {
        let mut hasher = DefaultHasher::new();
        let a: Vec<String> = table.read(*self);
        a.hash(&mut hasher);
        hasher.finish()
    }
    pub fn shash_debug(&self, table: Table, test_con: &str) -> u64 { // debug version with content to compare against
        let mut hasher = DefaultHasher::new();
        let a: Vec<String> = table.read(*self);

        let mut test_hasher = DefaultHasher::new(); // make 'b' hash with same content
        let b: Vec<String> = vec![String::from(test_con)];

        println!("actual Row: {:?}, test Row: {:?}", a, b); // print unhashed contents

        b.hash(&mut test_hasher); // finish 'b' hash
        let res_b = test_hasher.finish();

        a.hash(&mut hasher); // finish 'a' hash
        let res_a = hasher.finish();

        assert_eq!(res_a, res_b); // check if are the same
        hasher.finish()
    }
    pub fn delete(&self, table: Table) -> i8 {
        let row_path = format!("{}/{}", table.path, self.pos); // create path of row
        if Path::new(&row_path).exists() { // use it to check if row exists
            fs::remove_file(row_path).expect("Couldn't delete Row."); // delete file
            return 0;
        } else {
            println!("Row doesn't exist at {}", row_path);
            return 1;
        }
    }
}

impl Field {
    pub fn length(&self, table: Table, row: Row) -> i32 {
        let con = table.read(row);
        con[self.pos as usize].len() as i32
    }
    pub fn shash(&self, table: Table, row: Row) -> u64 {
        let mut hasher = DefaultHasher::new();
        let a: Vec<String> = table.read(row);
        a[self.pos as usize].hash(&mut hasher);
        hasher.finish()
    }
    pub fn shash_debug(&self, table: Table, row: Row, test_con: &str) -> u64 { // debug version with content to compare against
        let mut hasher = DefaultHasher::new();
        let a: Vec<String> = table.read(row);

        let mut test_hasher = DefaultHasher::new(); // make 'b' hash with same content
        let b: Vec<String> = vec![String::from(test_con)];

        println!("actual Field: {:?}, test Field: {:?}", a[self.pos as usize], b[0]); // print unhashed contents

        b[0].hash(&mut test_hasher); // finish 'b' hash
        let res_b = test_hasher.finish();

         a[self.pos as usize].hash(&mut hasher); // finish 'a' hash
        let res_a = hasher.finish();

        assert_eq!(res_a, res_b); // check if are the same
        hasher.finish()
    }
    pub fn delete(&self, table: Table, row: Row) -> i8 {
        let mut wo_field = table.read(row); // read contents with field
        println!("{:?}", wo_field);
        wo_field.remove(self.pos as usize); // remove that field
        println!("{:?}", wo_field);
        let wo_field_str: &str = &wo_field.join("\n"); // make it into one string
        println!("{}", wo_field_str);
        table.write(wo_field_str, row) // rewrite row without field
    }
}

pub fn init(table: Table) -> i8 {
    0
}