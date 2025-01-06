use std::collections::HashMap;

fn main() {
    println!("\n\nDemo of Collections in Rust");

    demo_vectors();

    demo_maps();

    demo_sets();
}

fn demo_vectors() {
    println!("\n\n === Demo of Vector ===");

    // Vectors are generic and growable sequence of data
    // data is stored on the heap
    let mut v1: Vec<i32> = Vec::new();
    println!("Vector v1:{:?}", v1);
    // push and pop will add elements at the end of the vector
    v1.push(10);
    v1.push(20);
    v1.push(30);
    let _ = v1.pop();
    println!("Vector:{:?}", v1);

    let v2: Vec<i32> = Vec::from([1, 2, 3, 4, 5]);
    println!("Vector v2:{:?}", v2);
    println!("v2[0]={}", v2[0]);

    let v3 = vec![1, 2, 3, 4, 5];
    println!("Vector v3:{:?}", v3);

    // Safely reading from vector
    let third: Option<&i32> = v3.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // VecDeque : A double ended queue

    // LinkedList : A doubly-linked list containing a series of nodes
    // each node points to the node before and after in the series
}

fn demo_maps() {
    println!("\n\n === Demo of Maps === ");

    // HashMap : A key value store like a Python dict
    // Reference: https://doc.rust-lang.org/book/ch08-03-hash-maps.html?highlight=hashmap#creating-a-new-hash-map

    let mut dict: HashMap<i32, &str> = HashMap::new();
    dict.insert(1, "Hello");
    dict.insert(2, "World!");
    println!(
        "HashMap - Size of dict:{}, e1:{}, e2:{}",
        dict.len(),
        dict[&1],
        dict[&2]
    );

    // Iterating over a hashmap
    println!("\n Printing all elements in the hashmap");
    for (key, value) in &dict {
        println!("key:{} value:{}", key, value);
    }

    // Now let's try removal of element
    let maybe_remove: Option<&str> = dict.remove(&8);
    match maybe_remove {
        Some(removed) => print!("Removed value:{} from dict", removed),
        None => println!("Key:8 does not exist"),
    }

    // BTreeMap: A map optimized for search
}

fn demo_sets() {

    // HashSet : A set containing no duplicates implemented as a HashMap

    // BTreeMap: An ordered set implemented as a BTreeMap that is optimized for search
}
