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

#### Shadowing en Memoria

Un tema a tener en consideración es que en general se pueden crear múltiples espacios en memoria cada vez que se crea una nueva variable cuando se aplica **shadowing**, aunque Rust libera los anteriores cuando salen del scope.

Veamos los dos casos:

1. Caso 1: Tipos en el Stack (i32, f64, bool, etc.)

```rust
let x = 5;       // [Stack: x = 5]
let x = x + 1;   // [Stack: x = 6]  <- el compilador REUTILIZA o libera el anterior
let x = x * 2;   // [Stack: x = 12] <- optimizado por el compilador
```

Para tipos en el stack, el compilador de Rust (via LLVM) optimiza agresivamente. En la practica, el espacio anterior se reutiliza o se elimina porque el compilador detecta que ya no es accesible. En el binario final, probablemente solo exista un espacio con el valor `12`.

2. Caso 2: Tipos en el Heap (String, Vec, etc.)

```rust
let x = String::from("hola");    // Heap: asigna memoria para "hola"
let x = String::from("mundo");   // Heap: asigna NUEVA memoria para "mundo"
                                  // "hola" se LIBERA (drop) porque ya no tiene owner

```

Aquí está lo importante: sí se crean dos espacios distintos en el heap, pero el primero se libera inmediatamente cuando el nuevo `x` lo "sombrea", porque el `String` anterior pierde su owner. 

Cada vez que haces shadow, el valor anterior se libera (`drop`). Al final solo queda 1 espacio en uso.

**Excepcion**: si capturas una referencia antes del shadow

```rust
let x = String::from("hola");
let referencia = &x;           // referencia apunta a "hola"
// let x = String::from("mundo"); // ERROR: no puedes hacer shadow mientras
                                   // haya una referencia activa a x

```
El compilador te protege de esta situacion.

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

### Return explicito vs implicito

En Rust hay dos formas de retornar un valor desde una funcion:

**1. Con `return` (explicito)** - usas la keyword `return` y terminas con `;`
```rust
fn es_mayor_de_edad(edad: u32) -> bool {
    if edad >= 18 {
        return true;   // return explicito: sale de la funcion inmediatamente
    }
    return false;      // return explicito
}
```

**2. Sin `return` (implicito)** - la ultima expression (sin `;`) es el valor que retorna
```rust
fn es_mayor_de_edad(edad: u32) -> bool {
    if edad >= 18 {
        true           // sin ; = expression que se retorna
    } else {
        false          // sin ; = expression que se retorna
    }
    // ^ todo el bloque if/else es una expression que produce un valor
}
```

El segundo caso es el **estilo idiomatico** en Rust. La clave es entender que en Rust casi todo es una expression que produce un valor: bloques `{}`, `if/else`, `match`, etc. La ultima expression sin `;` dentro de un bloque es su valor de retorno.

**Cuidado con el `;`** - si agregas `;` al final, se convierte en statement y retorna `()` (unit type):
```rust
fn sumar(a: i32, b: i32) -> i32 {
    a + b;  // ERROR: esto retorna () en lugar de i32
            // el ; convierte la expression en statement
}

fn sumar(a: i32, b: i32) -> i32 {
    a + b   // OK: sin ; retorna el resultado de a + b
}
```

**Cuando usar `return` explicito?** Solo cuando necesitas salir antes de llegar al final de la funcion (early return):
```rust
fn buscar_par(numeros: &[i32]) -> Option<i32> {
    for &n in numeros {
        if n % 2 == 0 {
            return Some(n);  // early return: encontramos un par, salimos ya
        }
    }
    None  // si llegamos aqui, no habia ningun par (implicit return)
}
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
