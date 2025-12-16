## Notas sobre los Tipos de Datos en Rust

- **Rust es un lenguaje de tipado estático**: Siempre debe conocer el tipo de cada variable en tiempo de compilación.
- **Inferencia de tipos**: El compilador suele deducir el tipo automáticamente, pero a veces necesitas especificarlo explícitamente con anotaciones (`let x: u32 = ...`).
- **Error común**: Si Rust no puede inferir el tipo, mostrará un error como `type annotations needed` y te sugerirá agregar una anotación.
- **Tipos escalares**: Representan un solo valor. Los principales son:
    - **Enteros**: Números sin parte decimal. Pueden ser *firmados* (`i8`, `i16`, `i32`, `i64`, `i128`, `isize`) o *sin signo* (`u8`, `u16`, `u32`, `u64`, `u128`, `usize`).
        - *Firmados* pueden ser negativos y positivos.
        - *Sin signo* solo positivos.
        - `isize` y `usize` dependen de la arquitectura (32 o 64 bits).
    - **Literales numéricos**: Puedes escribir números en decimal, hexadecimal (`0xff`), octal (`0o77`), binario (`0b1111_0000`) y como byte (`b'A'`).
    - **Separadores visuales**: Usa `_` para mejorar la legibilidad (`1_000` es igual a `1000`).

- **¿Qué tipo elegir?**
    - Por defecto, Rust usa `i32` para enteros.
    - Usa `isize` o `usize` para índices de colecciones.
    - Si necesitas un rango específico o ahorrar memoria, elige el tipo adecuado.

- **Representación interna**:
    - Los números firmados usan *complemento a dos*.
    - Los rangos dependen del número de bits (ejemplo: `i8` va de -128 a 127, `u8` de 0 a 255).
