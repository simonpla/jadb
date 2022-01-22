mod modify;

fn main() {
    let rlylongstr = "|o\nlabore et dolore magna aliquyam erat\n|o\nStet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.";
    let path: &str = "../../../test/123";
    let my_table = modify::Table {
        path: path,
    };
    println!(" {} {} {} {} {} {}",
    // modify::Table::create(path),
    modify::Table::write(rlylongstr, my_table, 1),
    modify::Row::length(modify::LenType::Fields),
    modify::Row::length(modify::LenType::Characters),
    modify::Row::hash(),
    modify::Field::length(),
    modify::Field::hash());
}
