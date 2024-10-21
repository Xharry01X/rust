fn main(){
    let my_age: i32 = 21;
    println!("My age is {}", my_age);

    let my_height: f32 = 5.5;
    println!("My height is {}", my_height);

    let is_student = true;
    println!("Am I student {}", is_student);

    let current_year: i32 = 2024;

    let birth_year = current_year - my_age;

    println!("My birthyear is {}", birth_year)
}