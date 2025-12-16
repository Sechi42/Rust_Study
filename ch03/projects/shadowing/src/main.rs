fn main() {
    let x = 5; // Declaramos x con valor 5

    let x = x + 1; // "Shadowing": redeclaramos x, ahora vale 6

    {
        // Nuevo bloque (alcance interno)
        let x = x * 2; // Shadowing otra vez: x ahora vale 12 solo dentro de este bloque
        println!("El valor de x en el alcance interno es {x}"); // Imprime 12
    }

    // Fuera del bloque interno, x vuelve a ser 6
    println!("El valor de x es {x}"); // Imprime 6
}
