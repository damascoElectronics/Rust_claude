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

| Stack | Heap |
|-------|------|
| Datos de tamanio fijo | Datos de tamanio variable |
| Rapido (LIFO) | Mas lento (necesita buscar espacio) |
| `i32`, `f64`, `bool`, `char` | `String`, `Vec`, `Box` |
| Se copian automaticamente (Copy trait) | Se mueven (move) por defecto |

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
