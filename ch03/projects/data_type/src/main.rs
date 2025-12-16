fn main() {
    let guess: u32 = "123".parse().expect("not a number!");
    println!("El número es: {}", guess);

    
    // Uso de tupla y desestructuración
    let tup = (500, 6.4, 1);
    let (_, y, _) = tup;
    println!("The value of y is: {y}");
    println!("The value of z is: {}", tup.2);
    println!("The value of x is: {}", tup.0);

    // Declaranto tupla con tipos explícitos
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("
    five_hundred: {five_hundred}, 
    six_point_four: {six_point_four}, 
    one: {one}");
}