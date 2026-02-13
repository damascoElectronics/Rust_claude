# Capitulo 7: Collections (Vec, String, HashMap)

> Referencia: *The Rust Programming Language* - Cap. 8: Common Collections

---

## Collections en Rust

A diferencia de los arrays y tuples (almacenados en el stack), las **collections** almacenan datos en el **heap** y pueden crecer/decrecer en runtime.

Las tres collections mas usadas:
- `Vec<T>` - Lista de elementos del mismo tipo
- `String` - Coleccion de caracteres UTF-8
- `HashMap<K, V>` - Pares clave-valor

---

## Vec\<T\> (Vector)

Un **vector** es un array dinamico. Todos los elementos deben ser del mismo type.

### Crear y Modificar

```rust
// Crear vacio
let mut v: Vec<i32> = Vec::new();

// Crear con macro vec!
let v2 = vec![1, 2, 3, 4, 5];

// Agregar elementos
v.push(10);
v.push(20);
v.push(30);

// Eliminar el ultimo
let ultimo = v.pop(); // Some(30)
```

### Acceder a Elementos

```rust
let v = vec![10, 20, 30, 40, 50];

// Con indice (panic si no existe)
let tercero = v[2];   // 30

// Con .get() (retorna Option, mas seguro)
match v.get(2) {
    Some(valor) => println!("Tercero: {valor}"),
    None => println!("No existe"),
}
```

### Iterar

```rust
// Immutable
for elemento in &v {
    println!("{elemento}");
}

// Mutable
let mut v = vec![1, 2, 3];
for elem in &mut v {
    *elem *= 2;  // dereference con *
}
```

---

## String

`String` en Rust es una coleccion de bytes UTF-8. Hay dos tipos principales:
- `String` - Owned, mutable, en el heap
- `&str` - String slice, referencia inmutable

```rust
// Crear Strings
let mut s = String::new();
let s2 = String::from("Hola");
let s3 = "Mundo".to_string();

// Concatenar
s.push_str("Hola ");    // agregar &str
s.push('!');              // agregar un char
let completo = format!("{s2} {s3}");  // sin mover ownership

// Slicing (cuidado con UTF-8!)
let hola = &s2[0..4];   // "Hola"

// Iterar por caracteres
for c in "Hola 🦀".chars() {
    println!("{c}");
}
```

**Importante:** No puedes indexar un String con `s[0]` porque UTF-8 usa bytes de tamanio variable.

---

## HashMap\<K, V\>

Almacena pares **key-value** (clave-valor):

```rust
use std::collections::HashMap;

let mut puntuaciones = HashMap::new();

// Insertar
puntuaciones.insert(String::from("Azul"), 10);
puntuaciones.insert(String::from("Rojo"), 50);

// Acceder
let azul = puntuaciones.get("Azul"); // Option<&i32>

// Iterar
for (equipo, puntos) in &puntuaciones {
    println!("{equipo}: {puntos}");
}

// Insertar solo si la key no existe
puntuaciones.entry(String::from("Verde")).or_insert(25);

// Actualizar basado en valor anterior
let texto = "hola mundo hola rust hola";
let mut conteo = HashMap::new();
for palabra in texto.split_whitespace() {
    let cuenta = conteo.entry(palabra).or_insert(0);
    *cuenta += 1;
}
```

---

## Ejecutar Este Ejemplo

```bash
cd 07_collections
cargo run
```

---

## Ejercicios

1. Dado un `Vec<i32>`, calcula la media, mediana y moda
2. Convierte un string a "Pig Latin" (mueve la primera consonante al final y agrega "ay")
3. Crea un directorio de empleados por departamento usando `HashMap<String, Vec<String>>`
