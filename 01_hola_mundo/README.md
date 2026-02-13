# Capitulo 1: Hola Mundo y Cargo

> Referencia: *The Rust Programming Language* - Cap. 1: Getting Started

---

## Tu Primer Programa en Rust

El clasico "Hello, World!" en Rust:

```rust
fn main() {
    println!("Hola, mundo!");
}
```

### Conceptos Clave

- `fn` - Keyword para declarar una **function**
- `main()` - Es la **entry point** (punto de entrada) de todo programa Rust
- `println!` - Es un **macro** (nota el `!`), no una funcion comun
- Las llaves `{}` delimitan el **body** (cuerpo) de la funcion
- Cada sentencia termina con `;` (semicolon)

---

## Cargo: El Build System y Package Manager

Cargo es la herramienta principal de Rust. Combina:
- **Build system** - Compila tu codigo
- **Package manager** - Maneja dependencias (llamadas **crates**)
- **Test runner** - Ejecuta pruebas

### Comandos Esenciales de Cargo

```bash
# Crear un nuevo proyecto
cargo new nombre_proyecto

# Compilar sin ejecutar
cargo build

# Compilar y ejecutar
cargo run

# Verificar que compila (mas rapido que build)
cargo check

# Compilar en modo release (optimizado)
cargo build --release
```

### Estructura de un Proyecto Cargo

```
mi_proyecto/
├── Cargo.toml    # Archivo de configuracion (manifest)
├── Cargo.lock    # Versiones exactas de dependencias (auto-generado)
└── src/
    └── main.rs   # Codigo fuente principal
```

### El Archivo `Cargo.toml`

```toml
[package]
name = "hola_mundo"       # Nombre del proyecto
version = "0.1.0"         # Version del proyecto
edition = "2021"          # Edicion de Rust a usar

[dependencies]            # Aqui van las dependencias externas (crates)
```

---

## Compilacion sin Cargo

Tambien puedes compilar directamente con `rustc`:

```bash
rustc main.rs    # Compila el archivo
./main           # Ejecuta el binario resultante
```

Pero para proyectos reales, **siempre usa Cargo**.

---

## Ejecutar Este Ejemplo

```bash
cd 01_hola_mundo
cargo run
```

---

## Ejercicios

1. Modifica el mensaje para que imprima tu nombre
2. Crea un nuevo proyecto con `cargo new mi_proyecto` y explora la estructura
3. Usa `cargo check` vs `cargo build` y observa la diferencia en velocidad
