use std::fs::File;
use std::io::Read;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Category {
    name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Product {
    name: String,
    price: i32,
    category: Category,
    tags: Vec<String>,
}

fn get_product() -> Product {
    let mut file: File = File::open("example.json").expect("Json file can not be opened");
    let mut json_str: String = String::new();

    file.read_to_string(&mut json_str)
        .expect("Can not load the file into string");

    let product: Product = serde_json::from_str(&json_str.as_str()).unwrap();

    return product;
}

fn main() {
    let product: Product = get_product();

    product.tags.iter().for_each(|tag| {
        println!("{}", tag);
    });


    print!("{} {} {:?} {}", product.name, product.price, product.tags, product.price);
}
