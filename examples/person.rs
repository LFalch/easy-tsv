use easy_tsv::impl_tsv;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    livret: String,
}

impl_tsv! {
    Person {
        name: String,
        age: u32,
        livret: String,
    }
}

fn main() {
    let examples = vec![
        Person {
            name: "Henning".to_owned(),
            age: 44,
            livret: "Ål".to_owned(),
        },
        Person {
            name: "Søren".to_owned(),
            age: 23,
            livret: "Ristet løg".to_owned(),
        },
        Person {
            name: "Bjarki".to_owned(),
            age: 11,
            livret: "Lasagne".to_owned(),
        },
    ];
    let s = easy_tsv::to_tsv_string(&examples);
    println!("{}", s);

    let examples: Vec<Person> = easy_tsv::from_tsv_string(s);
    println!("{:?}", examples);
}
