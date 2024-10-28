fn main() {
    let traffic_light = "green";

    match traffic_light {
        "green" => println!("Go"),
        "Yellow" => println!("Slow down"),
        "red" => println!("Stop"),
        _=> println!("Invalid color"), // if you're wondering why to use _ it means it's a default
    }
}
