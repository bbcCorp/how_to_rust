fn main() {
    
    let i = 16;
    println!("i = {}", i);

    let j = 20;
    println!("j = {}", j);

    let mut sum = i +j;
    println!("sum = {}", sum);



    // calculate area of circle
    let radius = 5.0;   
    println!("area = {}", getArea(radius));
}

fn getArea(radius: f64) -> f64 {
    // floating point
    const PI :f64 = 3.14159;

    let area = PI * radius * radius;
    return area;
}
