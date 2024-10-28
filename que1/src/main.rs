use std::io; // for standard inp[ut output


fn main(){
    println!("Please nenter a number");

    // REad the user input 
   // Read input from the user
   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("Failed to read line");

   // Parse the input directly to an integer
   let num: i32 = input.trim().parse().expect("Please enter a valid number");

   if num % 2 == 0 {
    println!("Your number is even");
} else {
    println!("Your number is odd");
}
}
