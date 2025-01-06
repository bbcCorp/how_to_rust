fn main() {
    {
        // References as safe pointers
        // References are always valid (no dangling references)
        // There can be only 1 mutable reference
        // There can be any number of immutable references

        let mut s1 = String::from("hello");
        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);

        create_greeting(&mut s1);
        println!("updated string: {}", s1);

        // Array reversal
        let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
        println!("\n\nArray: {:?}, len={}", arr, arr.len());
        inplace_reverse(&mut arr);
        println!("Reversed Array: {:?}, len={} \n\n", arr, arr.len());

        // Slice example
        // Slice is a reference to a contigious sequence of elements in a collection
        let hello = &s1[0..5];
        let world = &s1[7..12];

        println!("{} {}", hello, world);
    }

    {
        let mut arr = [1, 2, 3, 4, 5];
        println!("\n\nArray: {:?}, len={}", arr, arr.len());

        // create a immutable slice
        let slice = &arr[..1];
        println!("starting_element = {}, len={}", slice[0], slice.len());

        // create mutable slice
        let s1 = &mut arr[0..2];
        s1[0] = 0;
        println!("Slice 1: {:?}, len={}", s1, s1.len());
        println!("Array: {:?}, len={}", arr, arr.len());

        let s2 = &mut arr[3..];
        s2[0] = 10;
        println!("Slice 2: {:?}, len={}", s2, s2.len());
        println!("Array: {:?}, len={}", arr, arr.len());
    }
}

/// Function to do an inplace reversal of array
fn inplace_reverse(arr: &mut [i32]) {
    for i in 0..arr.len() / 2 {
        let temp = arr[i];
        arr[i] = arr[arr.len() - 1 - i];
        arr[arr.len() - 1 - i] = temp;
    }
}

/// Get an immutable reference as input and return size of the string
fn calculate_length(s: &String) -> usize {
    s.len()
}

// Get a mutable reference as input and update the string
fn create_greeting(s: &mut String) {
    s.push_str(", World!");
}
