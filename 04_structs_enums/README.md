# Capitulo 4: Structs y Enums

> Referencia: *The Rust Programming Language* - Cap. 5-6: Using Structs & Enums

---

## Structs

Un **struct** es un tipo de dato personalizado que agrupa campos relacionados.

### Definir y Crear Structs

```rust
struct Usuario {
    nombre: String,
    email: String,
    edad: u32,
    activo: bool,
}

let usuario = Usuario {
    nombre: String::from("Ana"),
    email: String::from("ana@mail.com"),
    edad: 28,
    activo: true,
};
```

### Field Init Shorthand

Si la variable tiene el mismo nombre que el campo:
```rust
let nombre = String::from("Ana");
let email = String::from("ana@mail.com");

let usuario = Usuario {
    nombre,    // shorthand: nombre: nombre
    email,     // shorthand: email: email
    edad: 28,
    activo: true,
};
```

### Struct Update Syntax

Crear un struct basado en otro:
```rust
let usuario2 = Usuario {
    email: String::from("otro@mail.com"),
    ..usuario  // el resto de campos viene de usuario
};
```

### Tuple Structs

Structs sin nombres de campo:
```rust
struct Color(i32, i32, i32);
struct Punto(f64, f64, f64);

let negro = Color(0, 0, 0);
let origen = Punto(0.0, 0.0, 0.0);
```

### Unit-Like Structs

Structs sin campos (utiles para traits):
```rust
struct SiempreIgual;
```

---

## Methods con `impl`

Los **methods** se definen dentro de un bloque `impl`:

```rust
struct Rectangulo {
    ancho: f64,
    alto: f64,
}

impl Rectangulo {
    // Method: primer parametro es &self
    fn area(&self) -> f64 {
        self.ancho * self.alto
    }

    fn es_cuadrado(&self) -> bool {
        self.ancho == self.alto
    }

    // Associated function (sin self) - como un constructor
    fn cuadrado(lado: f64) -> Self {
        Self {
            ancho: lado,
            alto: lado,
        }
    }
}

let rect = Rectangulo { ancho: 10.0, alto: 5.0 };
let area = rect.area();          // method call
let cuad = Rectangulo::cuadrado(5.0);  // associated function call
```

---

## Enums

Un **enum** define un tipo que puede ser una de varias variantes:

```rust
enum Direccion {
    Norte,
    Sur,
    Este,
    Oeste,
}

let dir = Direccion::Norte;
```

### Enums con Datos

Cada variante puede tener datos asociados de diferentes tipos:

```rust
enum Mensaje {
    Salir,                        // sin datos
    Mover { x: i32, y: i32 },    // struct-like
    Texto(String),                // un String
    Color(i32, i32, i32),         // tuple-like
}

let m1 = Mensaje::Salir;
let m2 = Mensaje::Mover { x: 10, y: 20 };
let m3 = Mensaje::Texto(String::from("hola"));
let m4 = Mensaje::Color(255, 0, 0);
```

### Methods en Enums

```rust
impl Mensaje {
    fn describir(&self) {
        match self {
            Mensaje::Salir => println!("Salir del programa"),
            Mensaje::Mover { x, y } => println!("Mover a ({x}, {y})"),
            Mensaje::Texto(t) => println!("Texto: {t}"),
            Mensaje::Color(r, g, b) => println!("Color: ({r}, {g}, {b})"),
        }
    }
}
```

---

## Option\<T\>: El Enum mas Importante

Rust no tiene `null`. En su lugar usa `Option<T>`:

```rust
enum Option<T> {
    Some(T),  // Hay un valor
    None,     // No hay valor
}

let numero: Option<i32> = Some(42);
let vacio: Option<i32> = None;

// Para obtener el valor, debes manejar ambos casos
match numero {
    Some(n) => println!("El numero es: {n}"),
    None => println!("No hay numero"),
}
```

---

## Ejecutar Este Ejemplo

```bash
cd 04_structs_enums
cargo run
```

---

## Ejercicios

1. Crea un struct `Circulo` con radio y un method `area()` y `perimetro()`
2. Crea un enum `Figura` con variantes `Circulo(f64)`, `Rectangulo(f64, f64)`, `Triangulo(f64, f64, f64)` y un method que calcule el area
3. Usa `Option<String>` para representar un campo "segundo nombre" opcional en un struct `Persona`
