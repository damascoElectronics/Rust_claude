# Aprende Rust desde Cero

> Basado en *"The Rust Programming Language"* de No Starch Press (conocido como "The Book").
> Contenido en **español** con palabras clave de programacion en **English**.

---

## Que es Rust?

Rust es un lenguaje de programacion de sistemas que se enfoca en tres pilares:
- **Safety** (Seguridad de memoria sin garbage collector)
- **Speed** (Rendimiento comparable a C/C++)
- **Concurrency** (Concurrencia segura)

---

## Estructura del Repositorio

| Capitulo | Tema | Referencia del Libro |
|----------|------|---------------------|
| [01](./01_hola_mundo/) | Hola Mundo y Cargo | Cap. 1: Getting Started |
| [02](./02_variables_tipos_funciones/) | Variables, Types y Functions | Cap. 3: Common Programming Concepts |
| [03](./03_ownership_borrowing/) | Ownership y Borrowing | Cap. 4: Understanding Ownership |
| [04](./04_structs_enums/) | Structs y Enums | Cap. 5-6: Structs & Enums |
| [05](./05_control_flujo_pattern_matching/) | Control de Flujo y Pattern Matching | Cap. 6: Enums and Pattern Matching |
| [06](./06_manejo_errores/) | Manejo de Errors | Cap. 9: Error Handling |
| [07](./07_collections/) | Collections (Vec, String, HashMap) | Cap. 8: Common Collections |
| [08](./08_generics_traits_lifetimes/) | Generics, Traits y Lifetimes | Cap. 10: Generic Types, Traits, Lifetimes |
| [09](./09_modulos_cargo/) | Modules y Cargo | Cap. 7 & 14: Packages, Crates, Modules |
| [10](./10_proyecto_final/) | Proyecto Final: CLI App | Cap. 12: An I/O Project |

---

## Requisitos Previos

1. **Instalar Rust** usando [rustup](https://rustup.rs/):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Verificar la instalacion:
   ```bash
   rustc --version
   cargo --version
   ```

## Como Usar Este Repositorio

1. Lee el `README.md` de cada capitulo en orden
2. Revisa los archivos `.rs` con los ejemplos
3. Ejecuta los ejemplos:
   ```bash
   cd 01_hola_mundo
   cargo run
   ```
4. Modifica el codigo y experimenta por tu cuenta
5. Intenta los **ejercicios** al final de cada capitulo

---

## Recursos Adicionales

- [The Rust Programming Language (The Book)](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Playground](https://play.rust-lang.org/) - Prueba codigo en el navegador
- [Rustlings](https://github.com/rust-lang/rustlings) - Ejercicios interactivos

---

> **Nota:** Este repositorio es un recurso educativo. Para profundizar, consulta el libro original completo.
