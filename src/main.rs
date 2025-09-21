fn get_rand_name() -> usize {
    rand::random_range(0..4)
}


fn main() {

    let names:Vec<&str> = vec!["Ryan", "Peter", "John", "Justin"];
    println!("Name: {}", names[get_rand_name()]);
}



