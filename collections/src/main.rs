use std::collections::HashMap;
fn main() {
    vector_collection();
    string_collection();
    hashmap_collection();
}

fn hashmap_collection(){

    let mut map: HashMap<String, &str> = HashMap::new();
    map.insert("url1".to_string(), "https://www.google.com");
    map.insert("url2".to_string(), "https://www.facebook.com");
    map.insert("url3".to_string(), "https://www.linkedin.com");
    println!("Map: {:?}", map);

    match map.get(&"url1".to_string()){
        Some(url) => println!("Url: {}", url),
        None => println!("Url não encontrada")
    };

}

fn string_collection(){
    let mut text = String::from("Hello,");
    text.push_str(" World!");

    println!("Text: {}", text);
}

fn vector_collection(){
    let lista: [u8; 5] = [1, 2, 3, 4, 5];
    println!("Lista: {:?}", lista);
    println!("Valor na posição 0: {}", lista[0]);

    let numeros = create_number_vector();
    println!("Numeros: {:?}", numeros);

    print_numbers(&numeros);
}

fn create_number_vector() -> Vec<u8> {
    let mut numeros: Vec<u8> = Vec::new();
    numeros.push(1);
    numeros.push(2);
    numeros.push(3);
    numeros.push(4);
    numeros.push(5);
    numeros
}

fn print_numbers(numbers: &Vec<u8>) {
    for n in numbers {
        println!("{}", n);
    }
}
