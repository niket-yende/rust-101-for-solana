use std::collections::HashMap;

fn main() {
    let unsigned: u32 = 10;
    let signed: i32 = -15;
    let float: f64 = 0.32;
    println!("Different numbers => {}, {}, {}", unsigned, signed, float);

    let character: char = 'a';
    println!("Character => {}", character);

    let boolean: bool = true;
    println!("Boolean => {}", boolean);

    let tuple: (i32, i32, f64, i32, bool) = (1, -2, 3.0, 4, true);
    println!("Tuple => {:?}", tuple);

    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array => {:?}", array);

    let string: &str = "Hello World!";
    println!("String => {}", string);

    let mut vector: Vec<i32> = vec![1, 2, 3, 4, 5];
    vector.push(6);
    println!("Vector => {:?}", vector);

    let mut hash_map: HashMap<&str, i32> = HashMap::new();
    hash_map.insert("Solana", 100);
    hash_map.insert("age", 2);
    println!("Hash Map => {:?}", hash_map);

    enum Color {
        Red,
        Green,
        Blue,
    }

    // hash set is used for grouping same data types

    let mut hash_set = std::collections::HashSet::new();

    hash_set.insert("John Doe");
    hash_set.insert("Jane Doe");

    // println!("Hash Set => {:?}", hash_set);

    // Looping

    for i in 0..10 {
        println!("Looping => {}", i);
    }

    let mut i = 0;
    while i < 10 {
        println!("Looping => {}", i);
        i += 1;
    }

    let mut counter = 0;
    loop {
        println!("Looping...");

        counter += 1;
        if counter >= 5 {
            break;
        }
    }

    // Looping over an array
    let array = [10, 20, 30, 40, 50];
    for element in array.iter() {
        println!("Array element: {}", element);
    }

    // Looping over an iterator
    let array = [100, 200, 300, 400, 500];
    for (index, value) in array.iter().enumerate() {
        println!("Value at index {}: {}", index, value);
    }
}
