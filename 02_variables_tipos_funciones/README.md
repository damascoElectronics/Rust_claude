# Capitulo 2: Variables, Types y Functions

> Referencia: *The Rust Programming Language* - Cap. 3: Common Programming Concepts

---

## Variables y Mutabilidad

En Rust, las variables son **immutable** (inmutables) por defecto. Esto es una decision de diseno para favorecer la seguridad.

```rust
let x = 5;       // immutable - NO se puede cambiar
let mut y = 10;   // mutable - SI se puede cambiar
y = 20;           // OK
// x = 10;        // ERROR: cannot assign twice to immutable variable
```

### Shadowing

Puedes declarar una nueva variable con el mismo nombre. Esto se llama **shadowing**:

```rust
let x = 5;
let x = x + 1;     // x ahora es 6 (nueva variable, no mutacion)
let x = x * 2;     // x ahora es 12

// Shadowing permite cambiar el type
let espacios = "   ";          // &str
let espacios = espacios.len(); // usize (numero)
```

**Diferencia con `mut`:** Shadowing crea una nueva variable; `mut` modifica la existente.

---

## Tipos de Datos (Data Types)

Rust es **statically typed** - el compilador debe saber el tipo de cada variable en tiempo de compilacion.

### Scalar Types (Tipos Escalares)

| Tipo | Descripcion | Ejemplos |
|------|-------------|----------|
| `i8, i16, i32, i64, i128` | Enteros con signo | `-5, 42, 1_000` |
| `u8, u16, u32, u64, u128` | Enteros sin signo | `0, 255, 1_000` |
| `f32, f64` | Punto flotante | `3.14, 2.0` |
| `bool` | Booleano | `true, false` |
| `char` | Caracter Unicode | `'a', 'Z', '🦀'` |

```rust
let entero: i32 = 42;
let flotante: f64 = 3.14;
let booleano: bool = true;
let caracter: char = '🦀';
```

### Compound Types (Tipos Compuestos)

**Tuple** - Agrupa valores de diferentes tipos con longitud fija:
```rust
let tupla: (i32, f64, char) = (500, 6.4, 'R');
let (x, y, z) = tupla;         // destructuring
let primero = tupla.0;          // acceso por indice
```

**Array** - Coleccion de elementos del mismo tipo con longitud fija:
```rust
let numeros: [i32; 5] = [1, 2, 3, 4, 5];
let primero = numeros[0];       // acceso por indice
let ceros = [0; 10];            // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
```

---

## Constants

Las **constants** son siempre inmutables y deben tener tipo explicito:

```rust
const MAX_PUNTOS: u32 = 100_000;
const PI: f64 = 3.14159265;
```

Diferencias con `let`:
- `const` debe tener type annotation siempre
- `const` se evalua en compile time
- `const` se puede declarar en cualquier scope, incluyendo global

---

## Functions

```rust
fn main() {
    saludar("Rust");
    let resultado = sumar(5, 3);
    println!("5 + 3 = {resultado}");
}

fn saludar(nombre: &str) {
    println!("Hola, {nombre}!");
}

// Los parameters DEBEN tener type annotation
// El return type se indica con ->
fn sumar(a: i32, b: i32) -> i32 {
    a + b  // Sin ; = expression que retorna el valor (implicit return)
}
```

### Statements vs Expressions

- **Statement**: Instruccion que NO retorna valor (termina con `;`)
- **Expression**: Produce un valor (NO termina con `;`)

```rust
let y = {
    let x = 3;
    x + 1       // expression - retorna 4 (sin ;)
};
// y == 4
```

---

## Ejecutar Este Ejemplo

```bash
cd 02_variables_tipos_funciones
cargo run
```

---

## Ejercicios

1. Crea una funcion que convierta grados Celsius a Fahrenheit: `F = (C * 9/5) + 32`
2. Crea una funcion que calcule el n-esimo numero de Fibonacci
3. Experimenta con shadowing: declara una variable `&str` y luego haz shadow con su `.len()`
