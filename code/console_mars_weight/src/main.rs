use std::io;

fn main() {
    println!("Enter your weight (in kgs): ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();

    println!("Weight in mars:{}", weight_in_mars(weight));
}

fn weight_in_mars(earth: f32) -> f32 {
    (earth / 9.81) * 3.711
}
