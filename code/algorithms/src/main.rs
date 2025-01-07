use algorithms::linked_list::LinkedListAsVec;
use std::collections::LinkedList;
use std::time::Instant;

fn main() {
    // make a list of 10k random ints
    let mut data: Vec<i32> = Vec::new();

    let test_data_size: i32 = 10;

    for i in 0..test_data_size {
        data.push(i + 1);
    }

    // Now we will measure the time it takes to insert this data
    // into a LinkedList vs LinkedListAsVec
    // We will keep inserting at the beginning of the list to bring out the worst case

    let mut linked_list: LinkedList<i32> = LinkedList::new();

    let mut start = Instant::now();
    for value in data.iter() {
        linked_list.push_front(*value);
    }
    let mut end = Instant::now();
    let mut duration = end - start;

    println!(
        "\n\nLinkedList - Insertion time for {} items:{:?}",
        linked_list.len(),
        duration
    );
    println!("{:?}", linked_list);

    let mut vec_list: LinkedListAsVec<i32> = LinkedListAsVec::new();

    start = Instant::now();
    for value in data.iter() {
        // We could use any of these methods
        vec_list.push_front(*value);
        //vec_list.insert_at_index(0, *value);
    }
    end = Instant::now();
    duration = end - start;

    println!(
        "\n\nLinkedListAsVec - Insertion time for {} items:{:?}",
        vec_list.len(),
        duration
    );
    println!("{:?}", vec_list);

    // Now we will remove all elements from the lists
    // Validate that they come out in the same order
    for _i in 0..test_data_size {
        assert_eq!(
            linked_list.pop_back().unwrap(),
            vec_list.pop_back().unwrap()
        );
    }

    // Validate that now they are both empty
    assert_eq!(linked_list.is_empty(), vec_list.is_empty());
}
