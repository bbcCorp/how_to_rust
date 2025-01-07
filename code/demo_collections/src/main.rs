use std::collections::HashMap;
use std::collections::LinkedList;

fn main() {
    println!("\n\nDemo of Collections in Rust");

    demo_vectors();
    demo_vec_deque();
    demo_linkedlist();

    demo_hash_map();
    demo_btree_map();

    demo_hash_set();
    demo_btree_set();
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

    // get an iterator
    let v1_iter = v1.iter();
    println!("Vector iterator: {:?}\n", v1_iter);

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
}

fn demo_vec_deque() {
    // VecDeque : A double ended queue
}

fn demo_linkedlist() {
    // LinkedList : A doubly-linked list containing a series of nodes
    // each node points to the node before and after in the series
    // Reference: https://doc.rust-lang.org/std/collections/struct.LinkedList.html

    println!("\n\n === Linked List ===\n");

    let mut ll1 = LinkedList::from([1, 2, 3]);
    println!("ll1: {:?}", ll1);

    // Searching in the list
    assert_eq!(ll1.contains(&1), true);
    assert_eq!(ll1.contains(&10), false);

    // iterating over a linked list
    for element in ll1.iter_mut() {
        *element += 10;
    }
    println!("ll1: {:?}", ll1);

    // remove all elements from linked list
    ll1.clear();

    // assert list is empty
    assert!(ll1.is_empty());

    // Start with an empty list and insert elements from both ends
    let mut llist = LinkedList::new();

    // assert that the reference to front and back nodes are None
    assert_eq!(llist.front(), None);
    assert_eq!(llist.back(), None);

    // Insert elements at both end
    llist.push_back('b');
    llist.push_front('a');
    llist.push_back('c');
    llist.push_back('d');

    println!("llist after insertion: {:?}", llist);

    // Now lets remove a few elements using pop_front() and pop_back() methods
    assert_eq!(llist.pop_front(), Some('a'));
    llist.pop_front();
    llist.pop_back();

    println!("llist after deletion: {:?}", llist);
}

fn demo_hash_map() {
    println!("\n\n === Demo of Maps === ");

    // HashMap : A key value store like a Python dict
    // Reference: https://doc.rust-lang.org/book/ch08-03-hash-maps.html?highlight=hashmap#creating-a-new-hash-map

    let coffee_map: HashMap<&str, f64> = HashMap::from([("Drip", 2.99), ("Espresso", 4.50)]);
    println!("Coffee hashmap:{:?}", coffee_map);

    // get an iterator
    let coffee_iter = coffee_map.iter();
    println!("HashMap iterator: {:?}\n", coffee_iter);

    // using iterator adapters - some consume some don't
    let total: f64 = coffee_iter.map(|(_coffee, value)| value).sum();
    println!("Total cost of all coffees:{}", total);

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
}

fn demo_btree_map() {
    // BTreeMap: A map optimized for search
}

fn demo_hash_set() {

    // HashSet : A set containing no duplicates implemented as a HashMap
}

fn demo_btree_set() {
    // BTreeMap: An ordered set implemented as a BTreeMap that is optimized for search
}
