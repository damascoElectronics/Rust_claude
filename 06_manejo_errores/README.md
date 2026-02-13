# Capitulo 6: Manejo de Errors

> Referencia: *The Rust Programming Language* - Cap. 9: Error Handling

---

## Filosofia de Rust sobre Errors

Rust divide los errores en dos categorias:
- **Recoverable errors** - Errores que se pueden manejar (`Result<T, E>`)
- **Unrecoverable errors** - Errores fatales que detienen el programa (`panic!`)

No hay **exceptions** como en otros lenguajes. Rust usa **types** para manejar errores.

---

## panic! - Errores Irrecuperables

`panic!` detiene el programa inmediatamente:

```rust
panic!("Algo salio muy mal!");

// Tambien ocurre al acceder a un indice invalido
let v = vec![1, 2, 3];
v[99]; // panic: index out of bounds
```

Usa `panic!` solo cuando no hay forma razonable de recuperarse.

---

## Result\<T, E\> - Errores Recuperables

```rust
enum Result<T, E> {
    Ok(T),   // Operacion exitosa con valor T
    Err(E),  // Error con informacion E
}
```

### Ejemplo: Leer un Archivo

```rust
use std::fs::File;

let resultado = File::open("datos.txt");

let archivo = match resultado {
    Ok(file) => file,
    Err(error) => {
        println!("Error al abrir archivo: {error}");
        return;
    }
};
```

### Manejar Diferentes Tipos de Error

```rust
use std::io::ErrorKind;

let archivo = match File::open("datos.txt") {
    Ok(file) => file,
    Err(error) => match error.kind() {
        ErrorKind::NotFound => {
            // Crear el archivo si no existe
            File::create("datos.txt").expect("No se pudo crear")
        }
        _ => panic!("Error inesperado: {error}"),
    },
};
```

---

## Atajos: unwrap() y expect()

```rust
// unwrap: retorna el valor o hace panic
let f = File::open("datos.txt").unwrap();

// expect: como unwrap pero con mensaje personalizado
let f = File::open("datos.txt").expect("No se pudo abrir datos.txt");
```

**Importante:** `unwrap()` y `expect()` son utiles para prototipos y pruebas, pero en produccion es mejor manejar el error explicitamente.

---

## El Operador ? (Propagation)

El operador `?` propaga el error automaticamente al caller:

```rust
use std::fs;
use std::io;

fn leer_archivo(ruta: &str) -> Result<String, io::Error> {
    let contenido = fs::read_to_string(ruta)?; // si hay error, retorna Err
    Ok(contenido)
}
```

`?` es equivalente a:
```rust
let contenido = match fs::read_to_string(ruta) {
    Ok(c) => c,
    Err(e) => return Err(e),
};
```

### Encadenar con ?

```rust
fn leer_primera_linea(ruta: &str) -> Result<String, io::Error> {
    let contenido = fs::read_to_string(ruta)?;
    let primera = contenido.lines().next().unwrap_or("").to_string();
    Ok(primera)
}
```

---

## Custom Error Types

```rust
#[derive(Debug)]
enum AppError {
    ArchivoNoEncontrado(String),
    DatosInvalidos(String),
    ConexionFallida,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::ArchivoNoEncontrado(ruta) => write!(f, "Archivo no encontrado: {ruta}"),
            AppError::DatosInvalidos(msg) => write!(f, "Datos invalidos: {msg}"),
            AppError::ConexionFallida => write!(f, "Conexion fallida"),
        }
    }
}
```

---

## Cuando usar panic! vs Result

| Situacion | Usar |
|-----------|------|
| Prototipos y ejemplos rapidos | `unwrap()` / `expect()` |
| Error del programador (bug) | `panic!` |
| Errores esperados (archivo, red, input) | `Result<T, E>` |
| El programa no puede continuar | `panic!` |
| Puedes recuperarte del error | `Result<T, E>` |

---

## Ejecutar Este Ejemplo

```bash
cd 06_manejo_errores
cargo run
```

---

## Ejercicios

1. Crea una funcion `dividir(a: f64, b: f64) -> Result<f64, String>` que retorne error si `b` es 0
2. Crea una funcion que lea un numero de un string usando `str::parse()` y maneje el error
3. Encadena multiples operaciones con `?` en una sola funcion
