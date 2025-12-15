## Notas del Capitulo

En Rust, las variables son inmutables por defecto. Usa `let mut` para declarar variables mutables y `let` para inmutables. Ejemplo:
```rust
let x = 5; // inmutable
let mut y = 10; // mutable
```
1. **Uso de `use` para importar módulos**: En Rust, se utiliza `use` para traer módulos o funciones específicas al alcance. En este caso, `std::io` y `rand::Rng` son importados para manejar la entrada del usuario y generar números aleatorios, respectivamente.

2. **Interpolación de cadenas**: Rust utiliza llaves `{}` para insertar valores en cadenas. Sin embargo, en el ejemplo, `println!("The secret number is: {secret_number}")` 

3. **Rangos inclusivos**: El método `gen_range(1..=100)` genera un número aleatorio en el rango inclusivo de 1 a 100. El operador `..=` incluye el límite superior.

4. **Manejo de errores**: La función `expect` se usa para manejar errores en caso de que `read_line` falle. Esto es útil para depuración y manejo seguro de errores.

5. **Cadenas mutables**: La variable `guess` se declara como mutable (`mut`) porque su valor se modifica al leer la entrada del usuario.

6. **Conversión de entrada**: La entrada del usuario se almacena como una cadena (`String`). Para usarla como número, sería necesario convertirla explícitamente, lo cual no se realiza en este código.

7. **Propiedad y préstamo**: La función `read_line` toma una referencia mutable de `guess` (`&mut guess`), lo que permite modificar su contenido sin transferir la propiedad.

