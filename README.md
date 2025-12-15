# 🦀 Rust Study - Repositorio de Aprendizaje

Este repositorio documenta mi proceso de aprendizaje del lenguaje de programación Rust, siguiendo una metodología estructurada y práctica.

## 📚 Descripción

Rust es un lenguaje de programación de sistemas que se enfoca en seguridad, velocidad y concurrencia. Este repositorio contiene ejercicios, proyectos y ejemplos organizados por capítulos, siguiendo una progresión lógica de aprendizaje.

## 🗂️ Estructura del Repositorio

### Capítulos Principales

#### **ch01** - Introducción a Rust
- Configuración del entorno de desarrollo
- Primer programa "Hello, World!"
- Compilación y ejecución básica
- Introducción a Cargo (gestor de paquetes de Rust)

#### **ch02** - Proyecto Guessing Game
- Manejo de entrada/salida con `std::io`
- Variables mutables e inmutables
- Uso de bibliotecas externas (`rand`)
- Manejo de errores con `Result` y `expect`
- Control de flujo con `loop` y `match`
- Comparación de valores con `Ordering`
- Conversión de tipos con `parse()`

## 🎯 Objetivos de Aprendizaje

- Dominar los conceptos fundamentales de Rust
- Entender el sistema de ownership y borrowing
- Aplicar patrones de programación funcional y procedural
- Desarrollar habilidades en manejo de errores y seguridad de memoria
- Construir proyectos prácticos incrementalmente

## 🛠️ Tecnologías y Herramientas

- **Lenguaje**: Rust (última versión estable)
- **Gestor de paquetes**: Cargo

## 🚀 Cómo Usar Este Repositorio

### Prerrequisitos

1. Instalar Rust desde [rustup.rs](https://rustup.rs/)
2. Verificar la instalación:
   ```bash
   rustc --version
   cargo --version
   ```

### Ejecutar los Ejemplos

Cada capítulo contiene proyectos independientes. Para ejecutarlos:

```bash
# Navegar al directorio del proyecto
cd ch02/projects/guessinf_game

# Compilar y ejecutar
cargo run

# Solo compilar
cargo build

# Compilar en modo release (optimizado)
cargo build --release
```

## 📝 Conceptos Clave Aprendidos

### Capítulo 1
- Compilación con `rustc`
- Gestión de proyectos con Cargo
- Macros básicas: `println!`

### Capítulo 2
- **Variables mutables**: `let mut variable`
- **Inmutabilidad por defecto**: seguridad en tiempo de compilación
- **Tipos de datos**: `String`, `u32`, inferencia de tipos
- **Manejo de errores**: `Result<T, E>`, `Ok`, `Err`
- **Pattern matching**: `match` para control de flujo exhaustivo
- **Referencias mutables**: `&mut` para pasar datos sin transferir ownership
- **Loops**: `loop` para iteraciones infinitas con `break`

## 🎓 Metodología de Aprendizaje

Este repositorio sigue principios de aprendizaje efectivo:

1. **Práctica activa**: Cada concepto se implementa en código
2. **Progresión gradual**: De lo simple a lo complejo
3. **Proyectos aplicados**: Casos de uso reales
4. **Documentación reflexiva**: Comentarios y notas sobre el proceso

## 📖 Recursos Adicionales

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) - Ejercicios interactivos
- [Rust Playground](https://play.rust-lang.org/) - Editor online

## 🔄 Progreso Actual

- ✅ Capítulo 1: Introducción completada
- ✅ Capítulo 2: Guessing Game completado
- ⏳ Capítulo 3: En progreso...

## 💡 Notas Importantes

- **Rust es un lenguaje fuertemente tipado**: Todos los tipos deben ser conocidos en tiempo de compilación
- **Ownership es único**: Cada valor tiene un único dueño
- **Inmutabilidad por defecto**: Ayuda a prevenir errores comunes
- **El compilador es tu amigo**: Los mensajes de error de Rust son muy descriptivos

## 🤝 Contribuciones

Este es un repositorio personal de aprendizaje, pero cualquier sugerencia o mejora es bienvenida mediante issues o pull requests.

## 📄 Licencia

Este proyecto es de uso educativo y personal.

---

**Última actualización**: Diciembre 2025  
**Autor**: Aprendiz de Rust 🦀
