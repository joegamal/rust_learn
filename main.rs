mod functions;


fn main() {
    let strng: String = String::from("hello world");
    
    let mut vec = vec![0, 1, 2, 3, 4];
    
    let _ = functions::add_value(&mut vec);

    println!("{}", vec.last().unwrap());

    println!("{}",strng);
}