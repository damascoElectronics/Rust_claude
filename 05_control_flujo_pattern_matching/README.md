# Capitulo 5: Control de Flujo y Pattern Matching

> Referencia: *The Rust Programming Language* - Cap. 3 (Control Flow) y Cap. 6 (Pattern Matching)

---

## if / else if / else

```rust
let numero = 7;

if numero > 10 {
    println!("Mayor que 10");
} else if numero > 5 {
    println!("Mayor que 5");
} else {
    println!("5 o menor");
}

// if como expression (similar al ternario en otros lenguajes)
let estado = if numero > 0 { "positivo" } else { "no positivo" };
```

---

## Loops

### `loop` - Loop Infinito

```rust
let mut contador = 0;
let resultado = loop {
    contador += 1;
    if contador == 10 {
        break contador * 2;  // break puede retornar un valor
    }
};
// resultado == 20
```

### Loop Labels

```rust
'exterior: loop {
    loop {
        break 'exterior;  // rompe el loop exterior
    }
}
```

### `while`

```rust
let mut n = 5;
while n > 0 {
    println!("{n}");
    n -= 1;
}
```

### `for` - El mas Usado

```rust
// Iterar sobre un range
for i in 0..5 {
    println!("{i}");  // 0, 1, 2, 3, 4
}

// Range inclusivo
for i in 1..=5 {
    println!("{i}");  // 1, 2, 3, 4, 5
}

// Iterar sobre una coleccion
let frutas = ["manzana", "banana", "cereza"];
for fruta in &frutas {
    println!("{fruta}");
}

// Con indice usando enumerate
for (i, fruta) in frutas.iter().enumerate() {
    println!("{i}: {fruta}");
}

// Iterar en reversa
for i in (1..=5).rev() {
    println!("{i}");  // 5, 4, 3, 2, 1
}
```

---

## match - Pattern Matching

`match` es una de las herramientas mas poderosas de Rust. Debe cubrir TODOS los casos posibles (**exhaustive**).

```rust
let numero = 3;

match numero {
    1 => println!("Uno"),
    2 => println!("Dos"),
    3 => println!("Tres"),
    4..=10 => println!("Entre 4 y 10"),
    _ => println!("Otro numero"),  // _ es el catch-all
}
```

### Match con Enums

```rust
enum Moneda {
    Centavo,
    Cinco,
    Diez,
    Veinticinco,
}

fn valor(moneda: &Moneda) -> u32 {
    match moneda {
        Moneda::Centavo => 1,
        Moneda::Cinco => 5,
        Moneda::Diez => 10,
        Moneda::Veinticinco => 25,
    }
}
```

### Match con Binding

```rust
match algun_valor {
    Some(n) if n > 0 => println!("Positivo: {n}"),  // match guard
    Some(n) => println!("No positivo: {n}"),
    None => println!("Nada"),
}
```

### Match con Destructuring

```rust
struct Punto { x: i32, y: i32 }

let p = Punto { x: 0, y: 7 };
match p {
    Punto { x: 0, y } => println!("En el eje Y: y={y}"),
    Punto { x, y: 0 } => println!("En el eje X: x={x}"),
    Punto { x, y } => println!("Punto: ({x}, {y})"),
}
```

---

## if let y while let

Formas concisas cuando solo te interesa un patron:

```rust
// if let - en vez de match con un solo caso
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("Max configurado: {max}");
}

// while let
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("Sacamos: {top}");
}
```

---

## Ejecutar Este Ejemplo

```bash
cd 05_control_flujo_pattern_matching
cargo run
```

---

## Ejercicios

1. Crea un programa que imprima "FizzBuzz": para numeros del 1 al 100, imprime "Fizz" si es multiplo de 3, "Buzz" si es multiplo de 5, "FizzBuzz" si es ambos
2. Usa `match` para clasificar una nota (0-100) en "Reprobado", "Suficiente", "Bueno", "Excelente"
3. Crea un enum `Operacion` con variantes `Suma(f64,f64)`, `Resta(f64,f64)`, etc. y usa `match` para calcular el resultado
