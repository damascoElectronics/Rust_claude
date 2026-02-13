# Capitulo 10: Proyecto Final - minigrep (CLI App)

> Referencia: *The Rust Programming Language* - Cap. 12: An I/O Project: Building a Command Line Program

---

## Que Vamos a Construir

Un programa de linea de comandos llamado **minigrep** que busca texto en archivos, similar al comando `grep` de Unix. Este proyecto combina todos los conceptos aprendidos:

- **Variables y types** (Cap. 2)
- **Ownership y borrowing** (Cap. 3)
- **Structs** (Cap. 4)
- **Pattern matching** (Cap. 5)
- **Error handling** con `Result` y `?` (Cap. 6)
- **Collections**: `Vec`, `String` (Cap. 7)
- **Traits** y **lifetimes** (Cap. 8)
- **Modules** y organizacion de codigo (Cap. 9)

---

## Uso

```bash
# Buscar "patron" en un archivo
cargo run -- <patron> <archivo>

# Ejemplos:
cargo run -- rust poema.txt
cargo run -- "to" poema.txt

# Busqueda case-insensitive (variable de entorno)
CASE_INSENSITIVE=1 cargo run -- "rust" poema.txt
```

---

## Estructura del Proyecto

```
10_proyecto_final/
├── Cargo.toml
├── README.md
├── poema.txt          # Archivo de prueba
└── src/
    ├── main.rs        # Entry point: parsea args y maneja errores
    └── lib.rs         # Logica principal: Config, buscar, ejecutar
```

---

## Conceptos Aplicados

### 1. Parseo de Argumentos
Leemos los arguments de la linea de comandos con `std::env::args()`.

### 2. Lectura de Archivos
Usamos `std::fs::read_to_string()` para leer el contenido del archivo.

### 3. Separacion de Responsabilidades
- `main.rs` - Solo maneja la ejecucion: parsea args, llama a `run()`, maneja errores
- `lib.rs` - Contiene toda la logica: struct `Config`, funciones `buscar()` y `run()`

### 4. Error Handling
Usamos `Result<T, Box<dyn Error>>` para manejar diferentes tipos de errores de forma elegante.

### 5. Tests
Tests unitarios para verificar que la busqueda funciona correctamente.

### 6. Variables de Entorno
Usamos `std::env::var()` para leer la variable `CASE_INSENSITIVE`.

---

## Ejecutar

```bash
cd 10_proyecto_final
cargo run -- rust poema.txt
```

## Ejecutar Tests

```bash
cargo test
```

---

## Ejercicios de Extension

1. Agrega soporte para buscar en multiples archivos
2. Agrega un flag `--count` que solo muestre el numero de coincidencias
3. Agrega coloreado del texto encontrado usando ANSI escape codes
4. Agrega soporte para expresiones regulares basicas
