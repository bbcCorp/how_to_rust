fn main() {
    demo_if_else();

    demo_unconditional_loop();

    demo_labelled_loop();

    demo_while_loop();
    demo_alternate_do_while_loop();

    demo_for_loop();

    demo_pattern_matching();
}

fn demo_if_else() {
    println!("\n\n If else demo");

    let number = 3;
    let mut msg = "";

    if number == 5 {
        println!("Condition matched");
    } else if number < 5 {
        msg = "condition was true";
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    println!("Control flow msg: {}", msg);

    // alternate syntax
    msg = if number < 5 {
        "condition was true"
    } else {
        "condition was false"
    };
    println!("Alternate syntax: {}", msg);
}

fn demo_unconditional_loop() {
    println!("\n\n Unconditional loop demo");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn demo_while_loop() {
    println!("\n\n While loop demo");

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn demo_alternate_do_while_loop() {
    // rust does not have a do while loop
    // but you can use a while loop with a let statement

    let mut number = 3;
    loop {
        println!("{}!", number);

        number -= 1;

        if number == 0 {
            break;
        }
    }
}

fn demo_labelled_loop() {
    println!("\n\nDemo of labelled nested loop");

    // Nested loops with loop labels
    'one: loop {
        'two: loop {
            println!("Breaking out of loop 2");
            break 'two;
        }

        println!("In loop 1");
        break 'one;
    }
}

fn demo_for_loop() {
    println!("\n\n For loop demo");

    for num in [7, 8, 9].iter() {
        println!("num: {}", num);
    }

    let array = [(1, 2), (3, 4), (5, 6)];
    println!("array: {:?}", array);
    let mut index = 0;
    for (a, b) in array.iter() {
        println!("index:{} :: a: {}, b: {}", index, a, b);
        index += 1;
    }

    // iterate using range from 0 to 9
    for i in 0..10 {
        println!("i: {}", i);
    }

    // iterate and update a vector collection
    let mut my_vec = vec![1, 2, 3, 4, 5];
    println!("my_vec before looping: {:?}", my_vec);
    for num in &mut my_vec {
        *num += 10
    }
    println!("my_vec after looping: {:?}", my_vec);
}

// ---------- //
enum MessageState {
    Pending,
    Sending,
    Received,
}

fn check_msg_state(msg_state: &MessageState) {
    match msg_state {
        MessageState::Pending => println!("Msg is Pending"),

        // catch all condition
        _ => println!("Msg is no longer pending"),
    }
}

fn demo_pattern_matching() {
    println!("\n\nPattern Matching demo");

    let mut msg = MessageState::Pending;
    check_msg_state(&msg);

    msg = MessageState::Received;
    check_msg_state(&msg);
}
