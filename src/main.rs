fn main() {
    println!("Introduce tu edad \t");
    let mut _edad:String=String::new();
    std::io::stdin().read_line(&mut _edad).unwrap();
    let _age:i32=_edad.trim().parse().unwrap();

    println!("Su edad es de: {} años ",_age);
}
