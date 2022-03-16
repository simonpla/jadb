//! # jadb
//!
//! This is **j**ust **a**nother **d**ata**b**ase software.
//! It aims to be simple and fast, while providing basic features like search and encryption. It also is designed to scale well on multiple systems.
//!
//! ## Formatting
//!
//! | Operator | Function |
//! | ----------- | ----------- |
//! | \n | delimiter between fields |
//! | \|o | replace with old content of row |

// time
extern crate chrono;

// hashing
use std::hash::{Hash, Hasher};

/// # Table
///
/// The table is a construct, where you can save rows. Every table has a unique id.
///
/// ## Examples
/// ```
/// use jadb;
///
/// let table = jadb::Table {
///     path: "mytable",
///     id: 0,
/// };
/// ```

#[derive(Copy, Clone)]
pub struct Table<'a> { // Table
    pub path: &'a str, // name of db, absolute or relative path
    pub id: usize, // table id
}

/// # Row
///
/// The row is a construct which is saved in tables. It consists of fields. Every Row has a position in the table.
///
/// ## Examples
/// ```
/// use jadb;
///
/// let table = jadb::Row {
///     pos: 0
/// };
/// ```

#[derive(Copy, Clone, Hash)]
pub struct Row { // Row
    pub pos: usize, // position (line) in Table
}

/// # Field
///
/// A field construct is saved in rows. It holds the data and has a position in its row.
///
/// ## Examples
/// ```
/// use jadb;
///
/// let table = jadb::Field {
///     pos: 0
/// };
/// ```

#[derive(Copy, Clone, Hash)]
pub struct Field {
    pub pos: usize, // position in Row
}

impl Table<'_> {
    /// # create()
    ///
    /// This creates a new table.
    ///
    /// A new directory is created, where rows can be saved in the future. This function takes a Table struct. The path can either be relative or full.
    ///
    /// ## Panic
    ///
    /// A 0 is returned if everything ran ok, else it returns 1 with a short explanation of what went wrong.
    ///
    /// ## Examples
    /// ```
    /// use jadb;
    ///
    /// let table = jadb::Table {
    ///   path: "mytable",
    ///   id: 0,
    /// };
    ///
    /// let mut hash_storage: Vec<Vec<std::collections::HashMap<String, usize>>> = vec![vec![std::collections::HashMap::new()]];
    ///
    /// table.create();
    ///
    /// table.delete(&mut hash_storage); // delete table afterwards
    /// ```
    pub fn create(&self) -> i8 {
        if self.path.is_empty() { // can't create table without name
            println!("no table path given, cannot create table");
            return 1;
        }
        if std::path::Path::new(self.path).exists() {
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
        std::fs::create_dir(self.path).expect("Couldn't create db directory.");
        std::fs::write(format!("{}/{}", self.path, "info.jadb"), info).expect("Couldn't create info file."); // write info file
        0 // if ok return 0
    }
    /// # write()
    ///
    /// This writes a new row to the table.
    ///
    /// A new file with the contents of the row is created. The fields are seperated using the delimiter `\n`.
    /// If a Row is rewritten and `|o` is used instead of new data for a field, the old content of the field will be used for the new one.
    /// A variable for storing the hash contents of all fields in all tables must be provided.
    ///
    /// ## Panic
    ///
    /// A `0` is returned if everything ran ok, else it returns `1` with a short explanation of what went wrong.
    ///
    /// ## Examples
    /// ```
    /// use jadb;
    ///
    /// let table = jadb::Table {
    ///   path: "mytable",
    ///   id: 0,
    /// };
    ///
    /// let row = jadb::Row {
    ///   pos: 0,
    /// };
    ///
    /// let mut hash_storage: Vec<Vec<std::collections::HashMap<String, usize>>> = vec![vec![std::collections::HashMap::new()]];
    ///
    /// table.create();
    ///
    /// jadb::init(table, &mut hash_storage); // Initialize the hash storage
    ///
    /// table.write("hi\nyou", row, &mut hash_storage); // write 'hi' and 'you' in seperate fields
    ///
    /// table.write("|o\neveryone", row, &mut hash_storage); // take the first field at index 0 and replace it with old content, overwrite the second field with 'everyone'
    ///
    /// table.delete(&mut hash_storage); // delete table afterwards
    /// ```
    pub fn write(&self, content: &str, row: Row, hash_var: &mut Vec<Vec<std::collections::HashMap<String, usize>>>) -> i8 {
        if content.is_empty() { // No need to create new row if no content
            println!("Not writing because no content given.");
            return 1;
        }
        println!("Writing {} to table path {} in Row {}", content, self.path, row.pos);
        let path = format!("{}/{}", self.path, row.pos); // path for row file
        if std::path::Path::new(&path).exists() { // if row already exists
            let mut con_w_form: String = String::from(""); // save contents here
            let mut con_str: Vec<&str> = content.split("\n").collect(); // split fields
            let old_row = std::fs::read_to_string(&path).expect("Couldn't read old Row contents"); // get old content of row
            let con_old_row: Vec<&str> = old_row.split("\n").collect(); // split fields of old content
            for i in 0..con_str.len() {
                if con_str[i] == "|o" { // if told to get old content...
                    con_str[i] = con_old_row[i]; // overwrite '|o' with old content
                }
                con_w_form.push_str(con_str[i]);
                if i != con_str.len()-1 {
                    con_w_form.push_str("\n"); // add delimiter
                }

                hash_var[self.id][row.pos].insert(con_str[i].parse().unwrap(), i); // add new content to hash variable
            }
            std::fs::write(&path, con_w_form).expect("Couldn't write Row");
        } else {
            std::fs::write(&path, content).expect("Couldn't write Row");

            let con_str: Vec<&str> = content.split("\n").collect(); // split fields
            for i in 0..con_str.len() {
                hash_var[self.id][row.pos].insert(con_str[i].parse().unwrap(), i); // add new content to hash variable
            }
        }
        if hash_var[self.id].len() < row.pos { // if row hash var is too small
            hash_var[self.id].resize(row.pos, std::collections::HashMap::new()); // resize
        }
        0 // if ok return 0
    }
    /// # read()
    ///
    /// Using this function you can read tables.
    ///
    /// This function returns a Vector with Strings. Each String consists of a field from the row that was read.
    ///
    /// ## Examples
    /// ```
    /// use jadb;
    ///
    /// let table = jadb::Table {
    ///   path: "mytable",
    ///   id: 0,
    /// };
    ///
    /// let row = jadb::Row {
    ///   pos: 0,
    /// };
    ///
    /// let mut hash_storage: Vec<Vec<std::collections::HashMap<String, usize>>> = vec![vec![std::collections::HashMap::new()]];
    ///
    /// table.create();
    ///
    /// jadb::init(table, &mut hash_storage); // Initialize the hash storage
    ///
    /// table.write("hi\nyou", row, &mut hash_storage); // write 'hi' and 'you' in seperate fields
    ///
    /// let row_contents: Vec<String> = table.read(row);
    ///
    /// table.delete(&mut hash_storage); // delete table afterwards
    /// ```
    pub fn read(&self, row: Row) -> Vec<String> {
        let content = std::fs::read_to_string(format!("{}/{}", self.path, row.pos)).expect("Couldn't read row");
        let con_str: Vec<String> = content.split("\n").map(String::from).collect();
        con_str
    }
    /// # search()
    ///
    /// Using this you can search a table for a string.
    ///
    /// The hash storage of this table is searched for the hash of the String that aims to be found.
    ///
    /// ## Panic
    ///
    /// If the term isn't found, a empty vector will be returned. Else a vector consisting of the table id, the row position and the field position will be returned.
    ///
    /// ## Examples
    /// ```
    /// use jadb;
    ///
    /// let table = jadb::Table {
    ///   path: "mytable",
    ///   id: 0,
    /// };
    ///
    /// let row = jadb::Row {
    ///   pos: 0,
    /// };
    ///
    /// let mut hash_storage: Vec<Vec<std::collections::HashMap<String, usize>>> = vec![vec![std::collections::HashMap::new()]];
    ///
    /// table.create();
    ///
    /// jadb::init(table, &mut hash_storage); // Initialize the hash storage
    ///
    /// table.write("hi\nyou", row, &mut hash_storage); // write 'hi' and 'you' in seperate fields
    ///
    /// let location: Vec<usize> = table.search(String::from("hi"), &mut hash_storage);
    ///
    /// table.delete(&mut hash_storage); // delete table afterwards
    /// ```
    pub fn search(&self, term: String, hash_var: &Vec<Vec<std::collections::HashMap<String, usize>>>) -> Vec<usize> {
        for i in 0..hash_var[self.id].len() { // search every row in table
            return match hash_var[self.id][i].get(&term) { // for term
                Some(result) => vec![self.id, i, *result], // return [Table, Row, pos]
                None => vec![]
            }
        }
        vec![]
    }
    /// # search()
    ///
    /// Deletes a table.
    ///
    /// This deletes the directory where the table is located in and clears the table's values in the hash storage.
    ///
    /// ## Panic
    ///
    /// If it can't find the table, it will return 1, if it can delete it, it will return 0.
    ///
    /// ## Examples
    /// ```
    /// use jadb;
    ///
    /// let table = jadb::Table {
    ///   path: "mytable",
    ///   id: 0,
    /// };
    ///
    /// let row = jadb::Row {
    ///   pos: 0,
     /// };
    ///
    /// let mut hash_storage: Vec<Vec<std::collections::HashMap<String, usize>>> = vec![vec![std::collections::HashMap::new()]];
    ///
    /// table.create();
    ///
    /// jadb::init(table, &mut hash_storage); // Initialize the hash storage
    ///
    /// table.delete(&mut hash_storage);
    /// ```
    pub fn delete(&self, hash_var: &mut Vec<Vec<std::collections::HashMap<String, usize>>>) -> i8 {
        let info_path = format!("{}/{}", self.path, "info.jadb"); // create path of info file
        return if std::path::Path::new(&info_path).exists() { // use it to check if table exists
            std::fs::remove_dir_all(self.path).expect("Couldn't delete database files."); // delete folder
            hash_var[self.id].clear(); // and the HashMap
            if self.id == hash_var.len()-1 { // if id of removed table is last element
                hash_var.pop(); // remove last element
            }
            0
        } else {
            println!("Table doesn't exist at {}", self.path);
            1
        }
    }
}
/// # LenType
///
/// This is needed for the Row::length() function to differentiate whether to count the fields in a row or the characters.
///
/// ## Examples
/// ```
/// use jadb;
///
/// let len_type = jadb::LenType::Characters;
/// ```
#[derive(PartialEq)]
pub enum LenType {
    Fields,
    Characters,
}

impl Row {
    /// # length()
    ///
    /// This returns the length of a row in either characters or fields.
    /// If you plan on using the length more than once without changing the row's content, consider saving it into a variable rather than using this function every time.
    ///
    /// ## Examples
    /// ```
    /// use jadb;
    ///
    /// let table = jadb::Table {
    ///   path: "mytable",
    ///   id: 0,
    /// };
    ///
    /// let row = jadb::Row {
    ///   pos: 0,
    /// };
    ///
    /// let mut hash_storage: Vec<Vec<std::collections::HashMap<String, usize>>> = vec![vec![std::collections::HashMap::new()]];
    ///
    /// table.create();
    ///
    /// jadb::init(table, &mut hash_storage); // Initialize the hash storage
    ///
    /// table.write("hi\nyou", row, &mut hash_storage); // write 'hi' and 'you' in seperate fields
    ///
    /// let length: i32 = row.length(table, jadb::LenType::Fields);
    ///
    /// table.delete(&mut hash_storage); // delete table afterwards
    /// ```
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
    /// # shash()
    ///
    /// This is used to compute the hash of a row.
    ///
    /// ## Examples
    /// ```
    /// use jadb;
    ///
    /// let table = jadb::Table {
    ///   path: "mytable",
    ///   id: 0,
    /// };
    ///
    /// let row = jadb::Row {
    ///   pos: 0,
    /// };
    ///
    /// let mut hash_storage: Vec<Vec<std::collections::HashMap<String, usize>>> = vec![vec![std::collections::HashMap::new()]];
    ///
    /// table.create();
    ///
    /// jadb::init(table, &mut hash_storage); // Initialize the hash storage
    ///
    /// table.write("hi\nyou", row, &mut hash_storage); // write 'hi' and 'you' in seperate fields
    ///
    /// let hash: u64 = row.shash(table);
    ///
    /// table.delete(&mut hash_storage); // delete table afterwards
    /// ```
    pub fn shash(&self, table: Table) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let a: Vec<String> = table.read(*self);
        a.hash(&mut hasher);
        hasher.finish()
    }
    /// # shash_debug()
    ///
    /// The debug version of shash(). It tests the hashing for a given string.
    ///
    /// ## Examples
    /// ```
    /// use jadb;
    ///
    /// let table = jadb::Table {
    ///   path: "mytable",
    ///   id: 0,
    /// };
    ///
    /// let row = jadb::Row {
    ///   pos: 0,
    /// };
    ///
    /// let mut hash_storage: Vec<Vec<std::collections::HashMap<String, usize>>> = vec![vec![std::collections::HashMap::new()]];
    ///
    /// table.create();
    ///
    /// jadb::init(table, &mut hash_storage); // Initialize the hash storage
    ///
    /// table.write("hey", row, &mut hash_storage); // write 'hi' and 'you' in seperate fields
    ///
    /// let hash: u64 = row.shash_debug(table, "hey");
    ///
    /// table.delete(&mut hash_storage); // delete table afterwards
    /// ```
    pub fn shash_debug(&self, table: Table, test_con: &str) -> u64 { // debug version with content to compare against
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let a: Vec<String> = table.read(*self);

        let mut test_hasher = std::collections::hash_map::DefaultHasher::new(); // make 'b' hash with same content
        let mut b: Vec<String> = Vec::with_capacity(1);
        b.push(String::from(test_con));

        println!("actual Row: {:?}, test Row: {:?}", a, b); // print unhashed contents

        b.hash(&mut test_hasher); // finish 'b' hash
        let res_b = test_hasher.finish();

        a.hash(&mut hasher); // finish 'a' hash
        let res_a = hasher.finish();

        assert_eq!(res_a, res_b); // check if are the same
        hasher.finish()
    }
    /// # delete()
    ///
    /// This deletes a row from a table and the hash storage.
    ///
    /// ## Panic
    ///
    /// Returns 1 if the row can't be found, if the deletion was successful, 0 is returned.
    ///
    /// ## Examples
    /// ```
    /// use jadb;
    ///
    /// let table = jadb::Table {
    ///   path: "mytable",
    ///   id: 0,
    /// };
    ///
    /// let row = jadb::Row {
    ///   pos: 0,
    /// };
    ///
    /// let mut hash_storage: Vec<Vec<std::collections::HashMap<String, usize>>> = vec![vec![std::collections::HashMap::new()]];
    ///
    /// table.create();
    ///
    /// jadb::init(table, &mut hash_storage); // Initialize the hash storage
    ///
    /// table.write("hi\nyou", row, &mut hash_storage); // write 'hi' and 'you' in seperate fields
    ///
    /// row.delete(table, &mut hash_storage);
    ///
    /// table.delete(&mut hash_storage); // delete table afterwards
    /// ```
    pub fn delete(&self, table: Table, hash_var: &mut Vec<Vec<std::collections::HashMap<String, usize>>>) -> i8 {
        let row_path = format!("{}/{}", table.path, self.pos); // create path of row
        return if std::path::Path::new(&row_path).exists() { // use it to check if row exists
            std::fs::remove_file(row_path).expect("Couldn't delete Row."); // delete file
            hash_var[table.id][self.pos].clear(); // and the HashMap
            if self.pos == hash_var[table.id].len()-1 { // if id of removed row is last element
                hash_var[table.id].pop(); // remove last element
            }
            0
        } else {
            println!("Row doesn't exist at {}", row_path);
            1
        }
    }
}

impl Field {
    /// # length()
    ///
    /// This returns the length of a field.
    /// If you plan on using the length more than once without changing the fields's content, consider saving it into a variable rather than using this function every time.
    ///
    /// ## Examples
    /// ```
    /// use jadb;
    ///
    /// let table = jadb::Table {
    ///   path: "mytable",
    ///   id: 0,
    /// };
    ///
    /// let row = jadb::Row {
    ///   pos: 0,
    /// };
    ///
    /// let field = jadb::Field {
    ///   pos: 0,
    /// };
    ///
    /// let mut hash_storage: Vec<Vec<std::collections::HashMap<String, usize>>> = vec![vec![std::collections::HashMap::new()]];
    ///
    /// table.create();
    ///
    /// jadb::init(table, &mut hash_storage); // Initialize the hash storage
    ///
    /// table.write("hi\nyou", row, &mut hash_storage); // write 'hi' and 'you' in seperate fields
    ///
    /// let length: i32 = field.length(table, row);
    ///
    /// table.delete(&mut hash_storage); // delete table afterwards
    /// ```
    pub fn length(&self, table: Table, row: Row) -> i32 {
        let con = table.read(row);
        con[self.pos].len() as i32
    }
    /// # shash()
    ///
    /// This is used to compute the hash of a field.
    ///
    /// ## Examples
    /// ```
    /// use jadb;
    ///
    /// let table = jadb::Table {
    ///   path: "mytable",
    ///   id: 0,
    /// };
    ///
    /// let row = jadb::Row {
    ///   pos: 0,
    /// };
    ///
    /// let field = jadb::Field {
    ///   pos: 0,
    /// };
    ///
    /// let mut hash_storage: Vec<Vec<std::collections::HashMap<String, usize>>> = vec![vec![std::collections::HashMap::new()]];
    ///
    /// table.create();
    ///
    /// jadb::init(table, &mut hash_storage); // Initialize the hash storage
    ///
    /// table.write("hi\nyou", row, &mut hash_storage); // write 'hi' and 'you' in seperate fields
    ///
    /// let hash: u64 = field.shash(table, row);
    ///
    /// table.delete(&mut hash_storage); // delete table afterwards
    /// ```
    pub fn shash(&self, table: Table, row: Row) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let a: Vec<String> = table.read(row);
        a[self.pos].hash(&mut hasher);
        hasher.finish()
    }
    /// # shash_debug()
    ///
    /// The debug version of shash(). It tests the hashing for a given string.
    ///
    /// ## Examples
    /// ```
    /// use jadb;
    ///
    /// let table = jadb::Table {
    ///   path: "mytable",
    ///   id: 0,
    /// };
    ///
    /// let row = jadb::Row {
    ///   pos: 0,
    /// };
    ///
    /// let field = jadb::Field {
    ///   pos: 0,
    /// };
    ///
    /// let mut hash_storage: Vec<Vec<std::collections::HashMap<String, usize>>> = vec![vec![std::collections::HashMap::new()]];
    ///
    /// table.create();
    ///
    /// jadb::init(table, &mut hash_storage); // Initialize the hash storage
    ///
    /// table.write("hey", row, &mut hash_storage); // write 'hi' and 'you' in seperate fields
    ///
    /// let hash: u64 = field.shash_debug(table, row, "hey");
    ///
    /// table.delete(&mut hash_storage); // delete table afterwards
    /// ```
    pub fn shash_debug(&self, table: Table, row: Row, test_con: &str) -> u64 { // debug version with content to compare against
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let a: Vec<String> = table.read(row);

        let mut test_hasher = std::collections::hash_map::DefaultHasher::new(); // make 'b' hash with same content
        let mut b: Vec<String> = Vec::with_capacity(1);
        b.push(String::from(test_con));

        println!("actual Field: {:?}, test Field: {:?}", a[self.pos], b[0]); // print unhashed contents

        b[0].hash(&mut test_hasher); // finish 'b' hash
        let res_b = test_hasher.finish();

         a[self.pos].hash(&mut hasher); // finish 'a' hash
        let res_a = hasher.finish();

        assert_eq!(res_a, res_b); // check if are the same
        hasher.finish()
    }
    /// # delete()
    ///
    /// This deletes a field from a row and the hash storage.
    ///
    /// ## Panic
    ///
    /// Returns 1 if the field can't be found, if the deletion was successful, 0 is returned.
    ///
    /// ## Examples
    /// ```
    /// use jadb;
    ///
    /// let table = jadb::Table {
    ///   path: "mytable",
    ///   id: 0,
    /// };
    ///
    /// let row = jadb::Row {
    ///   pos: 0,
    /// };
    ///
    /// let field = jadb::Field {
    ///   pos: 0,
    /// };
    ///
    /// let mut hash_storage: Vec<Vec<std::collections::HashMap<String, usize>>> = vec![vec![std::collections::HashMap::new()]];
    ///
    /// table.create();
    ///
    /// jadb::init(table, &mut hash_storage); // Initialize the hash storage
    ///
    /// table.write("hi\nyou", row, &mut hash_storage); // write 'hi' and 'you' in seperate fields
    ///
    /// field.delete(table, row, &mut hash_storage);
    ///
    /// table.delete(&mut hash_storage); // delete table afterwards
    /// ```
    pub fn delete(&self, table: Table, row: Row, hash_var: &mut Vec<Vec<std::collections::HashMap<String, usize>>>) -> i8 {
        let mut wo_field = table.read(row); // read contents with field
        let to_delete = wo_field[self.pos].clone(); // save content to be deleted
        wo_field.remove(self.pos); // remove it from the string
        hash_var[table.id][row.pos].remove(&*to_delete); // and the HashMap
        let wo_field_str: &str = &wo_field.join("\n"); // make it into one string
        table.write(wo_field_str, row, hash_var) // rewrite row without field
    }
}

/// # init()
///
/// This functions initializes a table. The tables contents hashes are put into the hash storage.
///
/// ## Examples
/// ```
/// use jadb;
///
/// let table = jadb::Table {
///   path: "mytable",
///   id: 0,
/// };
///
/// let mut hash_storage: Vec<Vec<std::collections::HashMap<String, usize>>> = vec![vec![std::collections::HashMap::new()]];
///
/// table.create();
///
/// jadb::init(table, &mut hash_storage);
///
/// table.delete(&mut hash_storage); // delete table afterwards
/// ```
pub fn init(table: Table, hash_var: &mut Vec<Vec<std::collections::HashMap<String, usize>>>) -> i8 {
    if hash_var.len() < table.id { // if table hash var is too small
        hash_var.resize(table.id, vec![std::collections::HashMap::new()]); // resize
    }
    let paths = std::fs::read_dir(table.path).expect("Couldn't read table directory"); // read dir contents
    let mut strpaths: Vec<String> = Vec::with_capacity(100); // assuming there are 100 rows. will be only reallocated if number is higher
    for path in paths {
        strpaths.push(path.unwrap().path().display().to_string()); // put them into a string vector
    }
    strpaths.shrink_to_fit(); // free up unused memory
    if hash_var[table.id].len() < strpaths.len() { // if row hash var is too small
        hash_var[table.id].resize(strpaths.len(), std::collections::HashMap::new()); // resize
    }
    for i in 0..strpaths.len() {
        if strpaths[i] != "info.jadb" { // if is a row file
            let con: Vec<String> = std::fs::read_to_string(&strpaths[i]).expect("Couldn't read Row").split("\n").map(String::from).collect(); // get Row contents
            for j in 0..con.len() {
                hash_var[table.id][i].insert(con[j].clone(), j); // add them to hash table
            }
        }
    }
    0
}

/// # search()
///
/// Using this you can search all tables for a string.
///
/// The hash storage of all tables is searched for the hash of the String that aims to be found.
///
/// ## Panic
///
/// If the term isn't found, a empty vector will be returned. Else a vector consisting of the table id, the row position and the field position will be returned.
///
/// ## Examples
/// ```
/// use jadb;
///
/// let table = jadb::Table {
///   path: "mytable",
///   id: 0,
/// };
///
/// let row = jadb::Row {
///   pos: 0,
/// };
///
/// let mut hash_storage: Vec<Vec<std::collections::HashMap<String, usize>>> = vec![vec![std::collections::HashMap::new()]];
///
/// table.create();
///
/// jadb::init(table, &mut hash_storage); // Initialize the hash storage
///
/// table.write("hi\nyou", row, &mut hash_storage); // write 'hi' and 'you' in seperate fields
///
/// let location: Vec<usize> = jadb::search(String::from("hi"), &mut hash_storage);
///
/// table.delete(&mut hash_storage); // delete table afterwards
/// ```
pub fn search(term: String, hash_var: &Vec<Vec<std::collections::HashMap<String, usize>>>) -> Vec<usize> {
    for i in 0..hash_var.len() { // iterate through whole hash array
        for j in 0..hash_var[i].len() { // iterate through every table
            return match hash_var[i][j].get(&term) {
                Some(result) => vec![i, j, *result],
                None => vec![]
            }
        }
    }
    vec![]
}
