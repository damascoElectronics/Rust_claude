# Capitulo 9: Modules y Cargo

> Referencia: *The Rust Programming Language* - Cap. 7 & 14: Packages, Crates, Modules

---

## El Sistema de Modules de Rust

Rust tiene un sistema para organizar codigo en piezas reutilizables:

- **Package** - Un proyecto de Cargo con `Cargo.toml`
- **Crate** - Un arbol de modules que produce un library o binary
- **Module** - Organiza codigo dentro de un crate y controla visibilidad
- **Path** - Forma de referirse a un item (struct, function, module, etc.)

---

## Crate Types

| Tipo | Descripcion | Entry Point |
|------|-------------|-------------|
| **Binary crate** | Programa ejecutable | `src/main.rs` |
| **Library crate** | Codigo reutilizable | `src/lib.rs` |

Un package puede tener **multiples** binary crates (en `src/bin/`) y como maximo **un** library crate.

---

## Definir Modules

```rust
// src/lib.rs

// Declarar un module
mod restaurante {
    // Submodule publico
    pub mod cocina {
        pub fn preparar_plato() {
            println!("Preparando...");
        }

        fn lavar_platos() {  // privado por defecto
            println!("Lavando...");
        }
    }

    pub mod servicio {
        pub fn tomar_orden() {
            // Llamar a otro module con super (path relativo)
            super::cocina::preparar_plato();
        }
    }
}
```

### Reglas de Visibilidad (Privacy)

- Todo es **private** (privado) por defecto
- `pub` hace un item publico
- Los hijos pueden ver todo del padre
- El padre NO puede ver items privados de los hijos
- `pub(crate)` hace visible solo dentro del crate
- `pub(super)` hace visible solo al module padre

---

## Paths: Referirse a Items

```rust
// Path absoluto (desde crate root)
crate::restaurante::cocina::preparar_plato();

// Path relativo (desde modulo actual)
restaurante::cocina::preparar_plato();

// super: ir al modulo padre
super::otra_funcion();
```

### `use` - Traer Items al Scope

```rust
use crate::restaurante::cocina;

// Ahora puedes usar directamente:
cocina::preparar_plato();

// Para functions, es idiomatico traer el modulo padre
use crate::restaurante::cocina;         // BIEN
use crate::restaurante::cocina::preparar_plato;  // funciona pero menos claro

// Para structs y enums, es idiomatico traer el item directamente
use std::collections::HashMap;          // BIEN

// Re-exportar con pub use
pub use crate::restaurante::cocina;
```

### Aliases con `as`

```rust
use std::fmt::Result;
use std::io::Result as IoResult;
```

### Nested Paths

```rust
// En vez de:
use std::io;
use std::io::Write;

// Puedes escribir:
use std::io::{self, Write};

// Traer todo de un module (glob operator)
use std::collections::*;
```

---

## Modules en Archivos Separados

Para proyectos grandes, cada module puede estar en su propio archivo:

```
src/
├── main.rs          // o lib.rs
├── restaurante.rs   // mod restaurante
└── restaurante/
    ├── cocina.rs    // mod cocina
    └── servicio.rs  // mod servicio
```

```rust
// src/main.rs
mod restaurante;  // busca en src/restaurante.rs o src/restaurante/mod.rs

// src/restaurante.rs
pub mod cocina;   // busca en src/restaurante/cocina.rs
pub mod servicio;
```

---

## Cargo en Profundidad

### Dependencias

```toml
[dependencies]
serde = "1.0"                     # desde crates.io
serde_json = { version = "1.0" }  # formato alternativo
mi_lib = { path = "../mi_lib" }   # crate local
```

### Profiles de Compilacion

```toml
# En Cargo.toml
[profile.dev]
opt-level = 0    # rapido de compilar

[profile.release]
opt-level = 3    # maximamente optimizado
```

### Comandos Utiles

```bash
cargo doc --open       # Generar y abrir documentacion
cargo test             # Ejecutar tests
cargo bench            # Ejecutar benchmarks
cargo publish          # Publicar a crates.io
cargo install nombre   # Instalar un binary crate
cargo fmt              # Formatear codigo
cargo clippy           # Linter con sugerencias
```

---

## Ejecutar Los Ejemplos

```bash
# Ejemplo de library crate
cd 09_modulos_cargo/mi_libreria
cargo test

# Ejemplo de uso
cd 09_modulos_cargo/uso_libreria
cargo run
```

---

## Ejercicios

1. Crea un library crate con modules para `matematicas::basica` y `matematicas::avanzada`
2. Agrega tests unitarios con `#[cfg(test)]`
3. Usa tu library desde otro binary crate con `path` dependency
