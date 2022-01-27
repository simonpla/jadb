mod jadb;

fn main() {
    let rlylongstr = "|o\nlabore et dolore magna aliquyam erat\n|o\nStet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.";
    let path: &str = "../../../test/123";
    let my_table = jadb::Table {
        path: path,
    };
    let my_row = jadb::Row {
        pos: 1,
    };
    let my_field = jadb::Field {
        pos: 0,
    };
    if cfg!(debug_assertions) {
        println!(
            "{} {}",
            my_row.shash_debug(my_table, "a"),
            my_field.shash_debug(my_table, my_row, "a")
        );
    } else {
        println!(
            "{} {}",
            my_row.shash(my_table),
            my_field.shash(my_table, my_row)
        );
    }
}
