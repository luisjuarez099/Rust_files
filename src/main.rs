fn main() {

    println!("Introduce tu Año de nacimeinto: ");
    let mut anio:String=String::new();
    std::io::stdin().read_line(&mut anio).unwrap();
    let _eddad:i32=anio.trim().parse().unwrap();

    let years:i32 = 2022 - _eddad;
    
    let mut name:String=String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    
    println!("Hola {name} Su edad es de {} años",years);


}
