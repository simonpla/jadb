#[cfg(test)]
mod tests {
    use jadb;
    use std::fs;
    use std::path::Path;

    fn a_delete(test_table: jadb::Table, info_path: String) { // prefix with letter so tests are run as intended in alphabetical order
        if Path::new(&info_path).exists() {
            fs::remove_dir_all(test_table.path).expect("Couldn't delete test files.");
        }
        assert_eq!(Path::new(&info_path).exists(), false);
    }
    #[test]
    fn a_test_create() {
        let test_table = jadb::Table {
            path: "tests/test_dir/test_db",
            id: 0,
        };
        let info_path = format!("{}/{}", test_table.path, "info.jadb");
        a_delete(test_table, info_path.clone());
        let c_res = test_table.create();
        assert_eq!(c_res, 0);
        assert_eq!(Path::new(&info_path).exists(), true);
    }
    #[test]
    fn b_test_write() {
        let test_table = jadb::Table {
            path: "tests/test_dir/test_db",
            id: 0,
        };
        let test_row = jadb::Row {
            pos: 0,
        };
        let w_res = test_table.write("hi", test_row);
        assert_eq!(w_res, 0);
        assert_eq!(fs::read_to_string(format!("{}/{}", test_table.path, 0)).expect("Couldn't read test"), "hi");
    }
    #[test]
    fn c_test_read() {
        let test_table = jadb::Table {
            path: "tests/test_dir/test_db",
            id: 0,
        };
        let test_row = jadb::Row {
            pos: 0,
        };
        let con = test_table.read(test_row);
        let v_con = vec![String::from("hi")];
        assert_eq!(con, v_con);
    }
    #[test]
    fn d_test_len() {
        let test_table = jadb::Table {
            path: "tests/test_dir/test_db",
            id: 0,
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
    fn e_test_hash() {
        let test_table = jadb::Table {
            path: "tests/test_dir/test_db",
            id: 0,
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

        assert_eq!(fs::read_to_string(format!("{}/{}_hash", test_table.path, test_row.pos)).expect("Couldn't read hash file."), "14565685931123352409");
    }
    #[test]
    fn f_search_test() {
        let test_table = jadb::Table {
            path: "tests/test_dir/test_db",
            id: 0,
        };
        let mut hasher: Vec<Vec<String>> = vec![];
        assert_eq!(jadb::init(test_table, &mut hasher), 0);
        assert_eq!(jadb::search(String::from("hi"), test_table, jadb::SearchType::Table, &hasher), vec![0, 0]);
    }
    #[test]
    fn g_test_delete() {
        let test_table = jadb::Table {
            path: "tests/test_dir/test_db",
            id: 0,
        };
        let test_row = jadb::Row {
            pos: 0,
        };
        let test_field = jadb::Field {
            pos: 1,
        };
        let row_path = format!("{}/{}", test_table.path, test_row.pos);
        let w_res = test_table.write("|o\na", test_row);
        assert_eq!(w_res, 0);
        let del_f = test_field.delete(test_table, test_row);
        assert_eq!(del_f, 0);
        assert_eq!(test_table.read(test_row), vec![String::from("hi")]);
        let del_r = test_row.delete(test_table);
        assert_eq!(del_r, 0);
        assert_eq!(Path::new(&row_path).exists(), false);
        let del_t = test_table.delete();
        assert_eq!(del_t, 0);
        assert_eq!(Path::new(&test_table.path).exists(), false);
    }
}
