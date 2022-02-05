#[cfg(test)]
mod tests {
    use jadb;
    use std::fs;
    use std::path::Path;

    #[test]
    fn a_delete() { // prefix with letter so tests are run as intended in alphabetical order
        let test_table = jadb::Table {
            path: "tests/test_dir/test_db",
        };
        let info_path = format!("{}/{}", test_table.path, "info.jadb");
        if Path::new(&info_path).exists() {
            fs::remove_dir_all(test_table.path).expect("Couldn't delete test files.");
        }
        assert_eq!(Path::new(&info_path).exists(), false);
    }
    #[test]
    fn b_test_create() {
        let test_table = jadb::Table {
            path: "tests/test_dir/test_db",
        };
        let info_path = format!("{}/{}", test_table.path, "info.jadb");
        test_table.create();
        assert_eq!(Path::new(&info_path).exists(), true);
    }
    #[test]
    fn c_test_write() {
        let test_table = jadb::Table {
            path: "tests/test_dir/test_db",
        };
        let test_row = jadb::Row {
            pos: 0,
        };
        test_table.write("hi", test_row);
        assert_eq!(fs::read_to_string(format!("{}/{}", test_table.path, 0)).expect("Couldn't read test"), "hi");
    }
    #[test]
    fn d_test_read() {
        let test_table = jadb::Table {
            path: "tests/test_dir/test_db",
        };
        let test_row = jadb::Row {
            pos: 0,
        };
        let con = test_table.read(test_row);
        let v_con = vec![String::from("hi")];
        assert_eq!(con, v_con);
    }
    #[test]
    fn e_test_len() {
        let test_table = jadb::Table {
            path: "tests/test_dir/test_db",
        };
        let test_row = jadb::Row {
            pos: 0,
        };
        let test_field = jadb::Field {
            pos: 0,
        };
        assert_eq!(test_row.length(test_table, jadb::LenType::Characters), 2);
        assert_eq!(test_row.length(test_table, jadb::LenType::Fields), 1);
        assert_eq!(test_field.length(test_table, test_row), 2);
    }
    #[test]
    fn f_test_hash() {
        let test_table = jadb::Table {
            path: "tests/test_dir/test_db",
        };
        let test_row = jadb::Row {
            pos: 0,
        };
        let test_field = jadb::Field {
            pos: 0,
        };
        assert_eq!(test_row.shash_debug(test_table, "hi"), 17259954866336786813);
        assert_eq!(test_row.shash(test_table), 17259954866336786813);
        assert_eq!(test_field.shash_debug(test_table, test_row, "hi"), 14565685931123352409);
        assert_eq!(test_field.shash(test_table, test_row), 14565685931123352409);
    }
}
