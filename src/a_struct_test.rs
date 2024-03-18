struct Student {
    name: String,
    age: i32,
    six: i32,
}

impl Student {
    fn new(name: String, age: i32, six: i32) -> Student {
        Student {
            name: name,
            age: age,
            six: six,
        }
    }
    // 不可变借用
    fn get_name(&self) -> String {
        self.name.clone()
    }
    // 可变借用
    fn set_name(&mut self, name: String) {
        self.name = name
    }

    fn avg_age(students: &[Student]) -> f32 {
        let mut sum = 0.0;
        for student in students.iter() {
            sum += student.age as f32
        }
        sum / (students.len() as f32)
    }
}

#[test]
fn test_01() {
    let mut s1 = Student::new(String::from("张三"), 18, 1);
    let s2 = Student::new(String::from("莉丝"), 20, 2);
    let s3 = Student::new(String::from("王五"), 22, 1);
    s1.set_name(String::from("新章三"));

    println!("s1_name:{},s2_name:{},s3_name:{}",s1.get_name(),s2.get_name(),s3.get_name());
    let stu = [s1,s2,s3];
    println!("平均年龄是：{}",Student::avg_age(&stu));
}
