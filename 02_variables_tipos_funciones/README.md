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

un tema a tener en concideracion es que en general se pueden crear multiples espacios en memoria casa vez se cera una nueva variable cuando se aplica **shadowing**, aunque Rust libera los anteriores cuando salen del scope.

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

Aqui esta lo importante, si se crean dos espacios distintos en el heap, pero el primero se libera inmediatamente cuando el nuevo `x` lo "sombrea", porque el `String` anterior pierde su owner. 

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

### ## Stack vs Heap

Son las dos zonas de memoria que usa tu programa:

### Stack (La Pila)

- Funciona como una **pila de platos**: el ultimo que pones es el primero que sacas (LIFO)
- **Super rapido** porque solo agrega/quita del tope
- El tamanio de cada dato **debe conocerse en compile time**
- Se libera automaticamente cuando la funcion termina

```
Stack (crece hacia abajo)
┌─────────────┐
│  z = true   │  <- tope (ultimo en entrar, primero en salir)
│  y = 3.14   │
│  x = 42     │
└─────────────┘
```

```rust
fn ejemplo() {
    let x: i32 = 42;      // 4 bytes en stack
    let y: f64 = 3.14;    // 8 bytes en stack
    let z: bool = true;   // 1 byte en stack
}   // x, y, z se liberan automaticamente aqui
```

### Heap (El Monton)

- Es un bloque grande de memoria **sin orden fijo**
- Cuando pides memoria, el sistema busca un espacio libre y te da un **pointer** (direccion)
- **Mas lento** porque tiene que buscar espacio y seguir pointers
- El tamanio puede ser **dinamico** (crecer/decrecer en runtime)
- En Rust se libera automaticamente via **ownership** (en C/C++ lo haces manual con malloc)

```
Stack                    Heap
┌──────────────┐        ┌─────────────────────┐
│ s ──────────────────> │ "Hola Mundo"        │
│ ptr, len: 10 │        │ (10 bytes, dinamico)│
│ capacity: 10 │        └─────────────────────┘
└──────────────┘
```

```rust
fn ejemplo() {
    let s = String::from("Hola Mundo");
    // s (en stack) tiene:
    //   - pointer -> apunta a los bytes en el heap
    //   - len: 10 (longitud actual)
    //   - capacity: 10 (espacio reservado)
    //
    // Los bytes "Hola Mundo" estan en el heap

}   // Rust llama drop(s) -> libera el heap automaticamente
```

### Por que importa en Rust?

```rust
// STACK: tipos de tamanio fijo -> Copy (se copian)
let a = 5;
let b = a;   // copia los 4 bytes, rapido y barato
println!("{a} {b}");  // ambos validos

// HEAP: tamanio variable -> Move (se mueven)
let s1 = String::from("hola");
let s2 = s1;  // solo copia el pointer (no duplica el heap)
              // s1 se invalida para evitar double free
// println!("{s1}"); // ERROR
```

Si Rust copiara el heap cada vez, seria lento. Si dejara dos owners apuntando al mismo heap, tendrias **double free** (liberar la misma memoria dos veces = crash). Por eso existe el sistema de **ownership**: un solo owner, una sola liberacion.

| | Stack | Heap |
|---|---|---|
| Velocidad | Muy rapido | Mas lento |
| Tamanio | Fijo (compile time) | Dinamico (runtime) |
| Tipos | `i32`, `f64`, `bool`, `char`, arrays | `String`, `Vec`, `Box`, `HashMap` |
| En Rust | Copy trait | Move semantics |
| Liberacion | Automatica al salir del scope | Automatica via `drop()` (ownership) |

Esto es lo que hace a Rust unico: **seguridad de memoria sin garbage collector**, gracias a que el compilador verifica ownership en compile time.

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
