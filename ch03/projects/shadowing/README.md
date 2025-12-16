## Notas

- **Shadowing permite reutilizar el nombre de una variable, creando una nueva cada vez que usamos `let` de nuevo.**
- **A diferencia de `mut`, shadowing permite cambiar el tipo de la variable.**
- **Con shadowing, la variable original deja de existir y solo la nueva está disponible.**
- **Usar `mut` solo permite cambiar el valor, pero no el tipo; intentar cambiar el tipo da error de compilación.**
- **Shadowing es útil para transformar valores paso a paso y luego hacer la variable inmutable.**
- **Evita la necesidad de inventar nombres diferentes para cada transformación (por ejemplo, `spaces_str` y `spaces_num`).**
- **El compilador ayuda a detectar errores de tipo cuando se usa `mut` incorrectamente.**