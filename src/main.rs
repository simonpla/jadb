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
    println!(/*" {} {:?} {} {} {}*/"{}",
    // modify::Table::create(path),
    /*jadb::Table::write(rlylongstr, my_table, 1),
    jadb::Table::read(my_table, 1),
    jadb::Row::length(jadb::LenType::Fields),
    jadb::Row::length(jadb::LenType::Characters),
    jadb::Field::length(),*/
    my_row.shash(my_table)
    );
}
