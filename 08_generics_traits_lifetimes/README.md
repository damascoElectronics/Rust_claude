# Capitulo 8: Generics, Traits y Lifetimes

> Referencia: *The Rust Programming Language* - Cap. 10: Generic Types, Traits, and Lifetimes

---

## Generics

Los **generics** permiten escribir codigo que funciona con multiples types, sin duplicar logica.

### Functions Genericas

```rust
// Sin generics: necesitarias una funcion por cada type
fn mayor_i32(lista: &[i32]) -> &i32 { ... }
fn mayor_f64(lista: &[f64]) -> &f64 { ... }

// Con generics: una sola funcion para cualquier type comparable
fn mayor<T: PartialOrd>(lista: &[T]) -> &T {
    let mut max = &lista[0];
    for item in &lista[1..] {
        if item > max {
            max = item;
        }
    }
    max
}
```

### Structs Genericos

```rust
struct Punto<T> {
    x: T,
    y: T,
}

// Con dos tipos diferentes
struct Par<T, U> {
    primero: T,
    segundo: U,
}

let entero = Punto { x: 5, y: 10 };
let flotante = Punto { x: 1.0, y: 4.0 };
let mixto = Par { primero: "hola", segundo: 42 };
```

### Methods Genericos

```rust
impl<T> Punto<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Method solo para Punto<f64>
impl Punto<f64> {
    fn distancia_al_origen(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

---

## Traits

Un **trait** define comportamiento compartido. Es similar a las **interfaces** en otros lenguajes.

### Definir un Trait

```rust
trait Resumen {
    // Method que debe implementar cada type
    fn resumir(&self) -> String;

    // Method con implementacion por defecto
    fn vista_previa(&self) -> String {
        format!("{}...", &self.resumir()[..20])
    }
}
```

### Implementar un Trait

```rust
struct Articulo {
    titulo: String,
    autor: String,
    contenido: String,
}

impl Resumen for Articulo {
    fn resumir(&self) -> String {
        format!("{} por {}", self.titulo, self.autor)
    }
}
```

### Traits como Parameters

```rust
// Syntax corta (impl Trait)
fn notificar(item: &impl Resumen) {
    println!("Nuevo: {}", item.resumir());
}

// Syntax completa (trait bound)
fn notificar<T: Resumen>(item: &T) {
    println!("Nuevo: {}", item.resumir());
}

// Multiples trait bounds
fn mostrar<T: Resumen + std::fmt::Display>(item: &T) { ... }

// Con where clause (mas legible)
fn procesar<T>(item: &T) -> String
where
    T: Resumen + Clone,
{
    item.resumir()
}
```

### Traits de la Standard Library

| Trait | Proposito | Ejemplo |
|-------|-----------|---------|
| `Display` | Formatear para el usuario | `println!("{}", x)` |
| `Debug` | Formatear para debugging | `println!("{:?}", x)` |
| `Clone` | Copia profunda explicita | `x.clone()` |
| `Copy` | Copia implicita (stack) | `let y = x;` |
| `PartialEq` | Comparacion de igualdad | `x == y` |
| `PartialOrd` | Comparacion de orden | `x > y` |
| `Default` | Valor por defecto | `T::default()` |
| `Iterator` | Iterar sobre elementos | `for x in iter` |

---

## Lifetimes

Los **lifetimes** aseguran que las references sean validas mientras se necesiten. El compilador los verifica en compile time.

### El Problema

```rust
// NO COMPILA: el compilador no sabe cual reference retornar
fn mas_largo(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

### La Solucion: Lifetime Annotations

```rust
// 'a es un lifetime parameter
fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

Esto le dice al compilador: "el return value vive al menos tanto como el mas corto de x e y".

### Lifetimes en Structs

```rust
// El struct no puede vivir mas que la reference que contiene
struct Extracto<'a> {
    texto: &'a str,
}
```

### Lifetime Elision Rules

El compilador puede inferir lifetimes automaticamente en muchos casos:

1. Cada reference parameter recibe su propio lifetime
2. Si hay exactamente un input lifetime, se asigna a todos los output
3. Si uno de los parameters es `&self`, su lifetime se asigna al output

```rust
// El compilador infiere los lifetimes automaticamente
fn primera_palabra(s: &str) -> &str { ... }
// Es equivalente a:
fn primera_palabra<'a>(s: &'a str) -> &'a str { ... }
```

### 'static Lifetime

`'static` significa que la reference vive durante toda la ejecucion del programa:

```rust
let s: &'static str = "Vivo para siempre"; // string literal
```

---

## Ejecutar Este Ejemplo

```bash
cd 08_generics_traits_lifetimes
cargo run
```

---

## Ejercicios

1. Crea un trait `Area` con un method `area(&self) -> f64` e implementalo para `Circulo` y `Rectangulo`
2. Escribe una funcion generica `imprimir_si_grande<T: Display + PartialOrd>` que imprima el valor solo si es mayor que un umbral
3. Crea un struct `Cache<'a>` que almacene una referencia a un string y un valor computado
