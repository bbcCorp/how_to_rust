fn main() {
    println!("\n\nDemo of Collections in Rust");

    demo_vectors();

    demo_maps();

    demo_sets();
}

fn demo_vectors() {
    // Vectors are generic and growable sequence of data
    // data is stored on the heap
    let mut v1: Vec<i32> = Vec::new();
    println!("Vector v1:{:?}", v1);
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
    // HashMap : A key value store like a Python dict

    // BTreeMap: A map optimized for search
}

fn demo_sets() {

    // HashSet : A set containing no duplicates implemented as a HashMap

    // BTreeMap: An ordered set implemented as a BTreeMap that is optimized for search
}
