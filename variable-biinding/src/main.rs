// let x=5; 
// // it's a immutable you can't change

// // mutable vs immutable

// // by default it's immutable which is good i beleive
// let mut y = 10;
// y=15


fn main(){
    let mut mutable_variable = 10;
    println!("Before: {}", mutable_variable);
    mutable_variable += 5;
    println!("After: {}",mutable_variable);
}

