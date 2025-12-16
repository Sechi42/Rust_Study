fn main() {
    let months = [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December",
    ];
    println!("El primer mes es: {}", months[0]);
    println!("El último mes es: {}", months[11]);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // Imprimiendo todo el array
    println!("Array completo: {:?}", a);

    let a = [3; 5]; // Esto crea un array con 5 elementos, todos inicializados a 3
    println!("Array con valores repetidos: {:?}", a);

    let first = a[0];
    let second = a[1];
    println!("
    Primer elemento: {}, 
    Segundo elemento: {}",
     first, second);


}
