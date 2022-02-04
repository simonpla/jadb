#[cfg(test)]
mod tests {
    use jadb;
    use std::fs;
    use std::path::Path;

    fn test_create(test_table: jadb::Table) {
        let info_path = format!("{}/{}", test_table.path, "info.jadb");
        test_table.create();
        assert_eq!(Path::new(&info_path).exists(), true);
    }
    fn test_write(test_table: jadb::Table, test_row: jadb::Row) {
        test_table.write("hi", test_row);
        assert_eq!(fs::read_to_string(format!("{}/{}", test_table.path, 0)).expect("Couldn't read test"), "hi");
    }
    fn test_read(test_table: jadb::Table, test_row: jadb::Row) {
        let con = test_table.read(test_row);
        let v_con = vec![String::from("hi")];
        assert_eq!(con, v_con);
    }
    fn delete(test_table: jadb::Table) {
        let info_path = format!("{}/{}", test_table.path, "info.jadb");
        if Path::new(&info_path).exists() {
            fs::remove_dir_all(test_table.path).expect("Couldn't delete test files.");
        }
        assert_eq!(Path::new(&info_path).exists(), false);
    }
    #[test]
    fn test_all() {
        let test_table = jadb::Table {
            path: "tests/test_dir/test_db",
        };
        let test_row = jadb::Row {
            pos: 0,
        };
        delete(test_table);
        println!("Deleted database.");
        test_create(test_table);
        println!("Created database.");
        test_write(test_table, test_row);
        println!("Wrote to database.");
        test_read(test_table, test_row);
        println!("Read from database.");
    }
}
