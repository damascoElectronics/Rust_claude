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

## `self` en Detalle

`self` es la forma en que un method se refiere **a la instancia sobre la que fue llamado** — equivalente al `this` de otros lenguajes, pero con una diferencia clave: en Rust debes declarar *como* quieres acceder a esa instancia, y esa eleccion tiene consecuencias de ownership.

Hay tres formas, y cada una tiene un significado distinto:

### `&self` — leer sin modificar

```rust
impl Rectangulo {
    fn area(&self) -> f64 {
        self.ancho * self.alto  // solo lee los campos
    }
}
```

- `&self` es un **borrow inmutable** de la instancia
- Puedes leer los campos pero **no puedes modificarlos**
- La instancia sigue siendo valida despues del llamado
- Es la forma mas comun — usala siempre que solo necesites leer

```rust
let rect = Rectangulo { ancho: 10.0, alto: 5.0 };
let a = rect.area();  // rect sigue existiendo, solo fue prestado
let b = rect.area();  // puedes llamarlo de nuevo
```

### `&mut self` — leer y modificar

```rust
impl Rectangulo {
    fn escalar(&mut self, factor: f64) {
        self.ancho *= factor;  // modifica el campo
        self.alto  *= factor;
    }
}
```

- `&mut self` es un **borrow mutable** de la instancia
- Puedes leer **y modificar** los campos
- La instancia debe haber sido declarada con `let mut`
- Solo puede haber un borrow mutable a la vez (regla de Rust)

```rust
let mut rect = Rectangulo { ancho: 10.0, alto: 5.0 };
rect.escalar(2.0);  // rect ahora mide 20x10
// rect sigue siendo valido y usable
```

### `self` — consumir la instancia

```rust
impl Rectangulo {
    fn convertir_en_cuadrado(self) -> Rectangulo {
        let lado = (self.ancho + self.alto) / 2.0;
        Rectangulo { ancho: lado, alto: lado }
        // self es consumido aqui, ya no existe
    }
}
```

- `self` toma **ownership** completo de la instancia
- Despues de llamar el method, **la instancia original ya no existe**
- Se usa cuando quieres transformar o destruir el valor
- Es poco comun, pero aparece en patrones tipo "builder"

```rust
let rect = Rectangulo { ancho: 10.0, alto: 5.0 };
let cuadrado = rect.convertir_en_cuadrado();
// rect ya no existe aqui — fue movido dentro del method
// println!("{}", rect.ancho); // ERROR: valor movido
```

### `Self` (mayuscula) — el tipo del struct

Dentro de `impl`, `Self` (con S mayuscula) es un alias para el tipo que estas implementando. Es azucar sintactico para no repetir el nombre del struct:

```rust
impl Rectangulo {
    fn cuadrado(lado: f64) -> Self {  // Self == Rectangulo
        Self {                         // Self == Rectangulo
            ancho: lado,
            alto: lado,
        }
    }
}
```

Es equivalente a escribir `Rectangulo` directamente, pero si renombras el struct, no tienes que cambiar nada dentro del `impl`.

### Funciones asociadas — sin `self`

Cuando una funcion dentro de `impl` **no tiene `self`** como primer parametro, no es un method: es una **funcion asociada**. No opera sobre una instancia existente, sino que normalmente crea una nueva:

```rust
impl Rectangulo {
    fn nuevo(ancho: f64, alto: f64) -> Self {
        Self { ancho, alto }
    }
}

// Se llama con :: en lugar de .
let rect = Rectangulo::nuevo(10.0, 5.0);
//                     ^^ no hay instancia antes del ::
```

La convencion en Rust es llamar `new` a la funcion constructora principal, aunque el lenguaje no lo obliga.

### Resumen visual

| Forma | Ownership | Puede modificar | Instancia sigue viva |
|-------|-----------|-----------------|----------------------|
| `&self` | borrow inmutable | No | Si |
| `&mut self` | borrow mutable | Si | Si |
| `self` | toma ownership | Si | No |
| _(sin self)_ | ninguno | — | No aplica |

### Por que importa esta distincion

En otros lenguajes `this` siempre tiene acceso total. En Rust la distincion existe porque el compilador necesita saber si un method puede modificar datos o transferir ownership — eso le permite garantizar que no hay data races ni uso de memoria invalida. Si intentas modificar un campo en un `&self`, el compilador te detiene en tiempo de compilacion, antes de que el bug exista.

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
