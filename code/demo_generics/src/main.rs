fn main() {
    println!("\n\n === Using Generics === \n");

    let students: Vec<Student> = Vec::from([
        Student::new(1, 98.5),
        Student::new(2, 87.3),
        Student::new(3, 79.5),
        Student::new(4, 99.89),
        Student::new(5, 57.9),
    ]);

    let best_student = best(&students);
    print!("\n Best Student:{}", best_student.print());
}

pub trait Comparable {
    fn compare_value(&self) -> i32;
}

fn best<T: Comparable>(items: &Vec<T>) -> &T {
    let mut best = &items[0];

    for item in items {
        if item.compare_value() >= best.compare_value() {
            best = &item;
        }
    }

    best
}

#[derive(Debug)]
struct Student {
    id: u8,
    score: f32,
}

impl Student {
    fn new(id: u8, score: f32) -> Student {
        Student {
            id: id,
            score: score,
        }
    }

    fn print(&self) -> String {
        format!("Student ID:{} - Score:{}", self.id, self.score)
    }
}

impl Comparable for Student {
    fn compare_value(&self) -> i32 {
        self.score.round() as i32
    }
}
