fn main() {
    // call associated function
    let p1 = Point::new(5.0, 10.0);
    let p2 = Point::new(10.0, 5.0);

    // call method
    p1.print();
    p2.print();

    let distance_between_points =
        |p1: Point, p2: Point| ((p2.x - p1.x).powf(2.0) + (p2.y - p1.y).powf(2.0)).sqrt();

    println!(
        "Distance between p1 and p2: {}",
        distance_between_points(p1, p2)
    );

    // call named function
    demo_closure();
}

// named function
fn demo_closure() {
    println!("Demo of functions and closures!");

    // Closure: Function that do not require a name
    let diff_closure = |a: u8, b: u8| if a > b { a - b } else { b - a };

    println!("Diff:{}", diff_closure(5, 8));
    println!("Diff:{}", diff_closure(8, 5));

    // Closures can capture the variable from outside its scope
    let PI = 3.14159;

    // The following annon function uses the value of PI from outside the body of the function
    let area_of_a_circle = |r: f32| PI * r * r;
    println!(
        "Area of circle with radius 5 units: {}",
        area_of_a_circle(5.0)
    );
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    // associated functions: functions usable on a type (similar to Static functions)
    fn new(x_coord: f32, y_coord: f32) -> Point {
        Point {
            x: x_coord,
            y: y_coord,
        }
    }

    // methods: functions usable on an instance of a type
    // for a method, the first param has to be a reference of self
    fn print(&self) {
        println!("\nX:{} | Y:{}", self.x, self.y);
    }
}
