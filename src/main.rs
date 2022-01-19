mod modify;

fn main() {
    let my_table = modify::Table {
        path: "../mydb",
    };
    println!("{} {} {} {} {} {} {}",
    modify::Table::create("456"),
    modify::Table::write("hi", my_table, 1),
    modify::Row::length(modify::LenType::Fields),
    modify::Row::length(modify::LenType::Characters),
    modify::Row::hash(),
    modify::Field::length(),
    modify::Field::hash());
}
