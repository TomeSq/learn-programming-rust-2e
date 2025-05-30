use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

fn main() {
    let mut table: Table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec!["many madrigals".to_string(), "tenebrae".to_string()],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Mattehew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Preseus with the head of Medusa".to_string(),
            "a salt celler".to_string(),
        ],
    );

    show(&table);

    assert_eq!(table["Gesualdo"][0], "many madrigals");
}
