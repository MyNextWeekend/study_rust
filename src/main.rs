use study_rust::test_struct::Student;

fn main() {
    let s = Student::new("name".to_string(), 18, 1);
    println!("hello {:?}", s);
    s.get_name();
    println!("hello {:?}", s);
}
