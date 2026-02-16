# Capitulo 3: Ownership y Borrowing

> Referencia: *The Rust Programming Language* - Cap. 4: Understanding Ownership

---

## El Concepto mas Importante de Rust

**Ownership** es lo que hace unico a Rust. Es un sistema de reglas que el compilador verifica en **compile time** para gestionar la memoria sin garbage collector.

---

## Las 3 Reglas del Ownership

1. Cada valor en Rust tiene un **owner** (dueño)
2. Solo puede haber **un owner** a la vez
3. Cuando el owner sale del **scope**, el valor se elimina (**drop**)

```rust
{
    let s = String::from("hola");  // s es el owner del String
    // s es valido aqui
}   // s sale del scope -> Rust llama a drop() -> memoria liberada
```

---

## Stack vs Heap

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
- En Rust se libera automáticamente via **ownership** (en C/C++ lo haces manual con malloc/free o new/delete)

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

---

## Move vs Copy

### Copy (tipos en el Stack)
```rust
let x = 5;
let y = x;  // Se COPIA el valor. x sigue siendo valido
println!("x={x}, y={y}"); // OK
```

### Move (tipos en el Heap)
```rust
let s1 = String::from("hola");
let s2 = s1;  // s1 se MUEVE a s2. s1 ya NO es valido
// println!("{s1}");  // ERROR: value used after move
println!("{s2}");     // OK
```

### Clone (copia profunda explícita)
```rust
let s1 = String::from("hola");
let s2 = s1.clone();  // Copia profunda (deep copy)
println!("s1={s1}, s2={s2}"); // OK, ambos son validos
```

---

## Ownership y Functions

Pasar un valor a una funcion es como asignarlo a otra variable: se **mueve** o se **copia**.

```rust
fn main() {
    let s = String::from("hola");
    tomar_ownership(s);       // s se mueve a la funcion
    // println!("{s}");        // ERROR: s ya no es valido

    let n = 42;
    hacer_copia(n);           // n se copia
    println!("{n}");          // OK: n sigue siendo valido
}

fn tomar_ownership(texto: String) {
    println!("{texto}");
}   // texto sale del scope y se libera

fn hacer_copia(numero: i32) {
    println!("{numero}");
}
```

---

## References y Borrowing

Para usar un valor sin tomar ownership, usamos **references** (&). Esto se llama **borrowing** (prestamo).

### Immutable Reference (&T)
```rust
fn main() {
    let s = String::from("hola");
    let longitud = calcular_longitud(&s);  // prestamos s
    println!("'{s}' tiene {longitud} caracteres"); // s sigue valido
}

fn calcular_longitud(s: &String) -> usize {
    s.len()
}   // s es solo una referencia, no se libera nada
```

### Mutable Reference (&mut T)
```rust
fn main() {
    let mut s = String::from("hola");
    agregar_texto(&mut s);
    println!("{s}"); // "hola, mundo"
}

fn agregar_texto(s: &mut String) {
    s.push_str(", mundo");
}
```

### Reglas de References

1. Puedes tener **multiples** immutable references (`&T`) al mismo tiempo
2. Solo puedes tener **una** mutable reference (`&mut T`) a la vez
3. No puedes mezclar immutable y mutable references al mismo tiempo

```rust
let mut s = String::from("hola");

let r1 = &s;     // OK
let r2 = &s;     // OK - multiples immutable refs
// let r3 = &mut s; // ERROR: no puedes tener mutable e immutable a la vez

println!("{r1}, {r2}");
// r1 y r2 ya no se usan despues de aqui

let r3 = &mut s;  // OK ahora - r1 y r2 ya no estan activos
println!("{r3}");
```

---

## Slices

Un **slice** es una referencia a una porcion contigua de una coleccion.

```rust
let s = String::from("hola mundo");

let hola = &s[0..4];    // "hola"
let mundo = &s[5..10];  // "mundo"
let todo = &s[..];      // "hola mundo" (slice completo)

// String slices tienen tipo &str
fn primera_palabra(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}
```

---

## Ejecutar Este Ejemplo

```bash
cd 03_ownership_borrowing
cargo run
```

---

## Ejercicios

1. Crea una funcion que reciba un `&String` y retorne `true` si contiene la palabra "rust"
2. Escribe una funcion que tome ownership de un `String`, lo modifique, y lo retorne
3. Experimenta: intenta crear dos `&mut` references al mismo valor y observa el error
