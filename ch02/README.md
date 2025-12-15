## Notas del Capitulo

En Rust, las variables son inmutables por defecto. Usa `let mut` para declarar variables mutables y `let` para inmutables. Ejemplo:
```rust
let x = 5; // inmutable
let mut y = 10; // mutable
```
1. **Uso de `use` para importar módulos**: En Rust, se utiliza `use` para traer módulos o funciones específicas al alcance. En este caso, `std::io`, `std::cmp::Ordering` y `rand::Rng` son importados para manejar la entrada del usuario, comparar valores y generar números aleatorios, respectivamente.

2. **Interpolación de cadenas**: Rust utiliza llaves `{}` para insertar valores en cadenas. Por ejemplo, `println!("You guessed: {guess}")` inserta el valor de `guess` en la cadena.

3. **Rangos inclusivos**: El método `gen_range(1..=100)` genera un número aleatorio en el rango inclusivo de 1 a 100. El operador `..=` incluye el límite superior.

4. **Manejo de errores**: La función `expect` se usa para manejar errores en caso de que `read_line` falle. Esto es útil para depuración y manejo seguro de errores.

5. **Cadenas mutables**: La variable `guess` se declara como mutable (`mut`) porque su valor se modifica al leer la entrada del usuario.

6. **Conversión de entrada**: La entrada del usuario se almacena como una cadena (`String`). Para usarla como número, se convierte explícitamente con `trim` y `parse`. Si la conversión falla, el programa continúa el bucle sin interrumpirse.

7. **Propiedad y préstamo**: La función `read_line` toma una referencia mutable de `guess` (`&mut guess`), lo que permite modificar su contenido sin transferir la propiedad.

8. **Comparación de valores**: El uso de `cmp` permite comparar el valor ingresado (`guess`) con el número secreto (`secret_number`). Esto devuelve un `Ordering` que puede ser `Less`, `Greater` o `Equal`.

9. **Bucle infinito con `loop`**: El programa utiliza un bucle infinito para permitir múltiples intentos hasta que el usuario adivine correctamente. El bucle se interrumpe con `break` cuando el usuario gana.

10. **Manejo de entradas inválidas**: Si el usuario ingresa un valor no numérico, el programa ignora la entrada y solicita un nuevo intento, gracias al uso de `match` y el patrón `Err(_)`.

11. **Mensajes de retroalimentación**: El programa proporciona retroalimentación inmediata al usuario indicando si el número ingresado es "Too small", "Too big" o si ha ganado con "You win!".

12. **Uso de `std::cmp::Ordering`**: Este enum se utiliza para manejar las comparaciones de manera clara y legible, mejorando la estructura del código.

13. **Generación de números aleatorios**: La función `rand::thread_rng().gen_range(1..=100)` asegura que el número secreto sea generado de manera eficiente y dentro del rango especificado.

14. **Buenas prácticas**: El código sigue patrones idiomáticos de Rust, como el manejo explícito de errores y el uso de referencias mutables para modificar variables.

15. **Extensibilidad**: Este programa puede ampliarse fácilmente para incluir características adicionales, como un contador de intentos o límites de tiempo.

