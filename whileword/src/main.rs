fn main() {
    let mut num = 1;

    while num <= 5 || num == 10 {
        if num > 2 && num < 5 {
            println!("Number is between 3 and 4: {}", num);
        } else {
            println!("Number is: {}", num);
        }

        num += 1;
    }
}
