fn main() {
    let mut count = 0; // Start from 0

    loop {
        count += 1;

        if count == 50 {
            println!("Breaking the loop at count: {}", count);
            break;
        }
    }
}
