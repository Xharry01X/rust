fn main() {
    let age = 18;

   if age >=18 && age<=70 {
    println!("You're allowed to use cars");
   } else if age == 18 || (age > 18 && age < 21){
    println!("You're allowed to use car but some certain restrictions");
   }else {
    println!("You're either too young or not eligible")
   }
}
