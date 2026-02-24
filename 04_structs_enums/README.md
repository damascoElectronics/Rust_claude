# Capitulo 4: Structs y Enums

> Referencia: *The Rust Programming Language* - Cap. 5-6: Using Structs & Enums

---

## Structs

Un **struct** es un tipo de dato personalizado que agrupa campos relacionados bajo un solo nombre. Si vienes de lenguajes orientados a objetos, puedes pensarlo como una "clase" pero sin herencia: tiene datos y puede tener metodos, pero no existe el concepto de que un struct "extienda" a otro.

### Que problema resuelven

Imagina que necesitas representar un usuario con nombre, email, edad y estado activo. Sin structs, tendrias que manejar cuatro variables sueltas y pasarlas por separado a cada funcion. Con un struct, agrupas esos datos en una sola unidad con campos nombrados, lo que hace tu codigo mas legible, mas seguro y mas facil de mantener.

### Cuando usar struct vs tupla

Rust tiene tuplas, que tambien agrupan valores. La diferencia es que los campos de un struct tienen **nombre**, y los de una tupla solo tienen **posicion**.

- Usa un **struct** cuando los campos tienen significado claro y los vas a usar en varias partes del programa. Por ejemplo, un `Usuario` con `nombre`, `email`, `edad` es mucho mas legible que una tupla `(String, String, u32)` donde tienes que recordar que el primer `String` es el nombre y el segundo es el email.
- Usa una **tupla** cuando necesitas agrupar valores temporalmente, como devolver dos resultados de una funcion, y el significado es obvio por el contexto.

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

Al crear un struct, debes proporcionar un valor para **cada campo**. No existe el concepto de campos con valores por defecto en la sintaxis basica (aunque puedes implementar patrones que lo simulen).

### Acceso a campos

Accedes a los campos de un struct con la notacion de punto:

```rust
println!("Nombre: {}", usuario.nombre);
println!("Edad: {}", usuario.edad);
```

Para **modificar** un campo, la variable que contiene el struct debe haber sido declarada con `let mut`. Rust no permite marcar campos individuales como mutables: o toda la instancia es mutable, o ninguna lo es.

```rust
let mut usuario = Usuario {
    nombre: String::from("Ana"),
    email: String::from("ana@mail.com"),
    edad: 28,
    activo: true,
};

usuario.edad = 29;  // funciona porque usuario es mut
usuario.email = String::from("nueva@mail.com");  // tambien funciona
```

Si hubieras declarado `let usuario` (sin `mut`), el compilador rechazaria cualquier intento de modificacion.

### Field Init Shorthand

Cuando tienes una variable con el **mismo nombre** que un campo del struct, puedes usar la forma abreviada para evitar repeticion. Esto existe por conveniencia: es muy comun que los parametros de una funcion constructora tengan los mismos nombres que los campos del struct, y escribir `nombre: nombre` se vuelve tedioso y ruidoso.

```rust
let nombre = String::from("Ana");
let email = String::from("ana@mail.com");

// Sin shorthand (repetitivo):
let usuario = Usuario {
    nombre: nombre,
    email: email,
    edad: 28,
    activo: true,
};

// Con shorthand (menos repeticion):
let usuario = Usuario {
    nombre,    // equivale a nombre: nombre
    email,     // equivale a email: email
    edad: 28,
    activo: true,
};
```

Ambas formas producen exactamente el mismo resultado. El shorthand solo aplica cuando el nombre de la variable coincide con el nombre del campo.

### Struct Update Syntax

Puedes crear un nuevo struct basado en otro existente, cambiando solo los campos que necesitas. Los campos que no especifiques se copian (o mueven) del struct original:

```rust
let usuario2 = Usuario {
    email: String::from("otro@mail.com"),
    ..usuario  // el resto de campos viene de usuario
};
```

**Advertencia importante sobre ownership:** La sintaxis `..usuario` no siempre copia los campos; en algunos casos los **mueve**. La diferencia depende del tipo de cada campo:

- Los campos que implementan el trait `Copy` (como numeros `u32`, `f64`, booleanos `bool`, etc.) se **copian**. El struct original conserva esos valores intactos.
- Los campos que NO implementan `Copy` (como `String`, `Vec`, y la mayoria de tipos que viven en el heap) se **mueven**. Despues del move, el struct original pierde ownership de esos campos.

En el ejemplo de arriba, `email` se proporciona explicitamente para `usuario2`, pero `nombre` (un `String`) se mueve desde `usuario`. Despues de esta linea, `usuario.nombre` ya no es valido. Sin embargo, `usuario.edad` y `usuario.activo` (que son `u32` y `bool`, tipos `Copy`) siguen siendo accesibles.

```rust
// Despues de crear usuario2:
println!("{}", usuario.edad);    // OK: u32 implementa Copy
println!("{}", usuario.activo);  // OK: bool implementa Copy
// println!("{}", usuario.nombre);  // ERROR: nombre fue movido a usuario2
// println!("{}", usuario);         // ERROR: no puedes usar usuario como un todo
```

Este es un punto de confusion frecuente. Si solo se "heredan" campos de tipos `Copy`, el struct original sigue completamente valido. Si se mueve al menos un campo no-`Copy`, el struct original queda parcialmente invalidado.

### Tuple Structs

Los tuple structs son structs cuyos campos no tienen nombre, solo posicion, como una tupla. Se definen con un nombre seguido de los tipos entre parentesis:

```rust
struct Color(i32, i32, i32);
struct Punto(f64, f64, f64);

let negro = Color(0, 0, 0);
let origen = Punto(0.0, 0.0, 0.0);
```

**Cuando usar tuple structs vs structs normales:** El caso de uso principal es crear **tipos distintos** que internamente tienen la misma estructura de datos. En el ejemplo anterior, `Color` y `Punto` tienen exactamente tres numeros, pero representan cosas completamente diferentes. Gracias a que son tipos distintos, el compilador impide que los confundas:

```rust
fn pintar(color: Color) { /* ... */ }

pintar(negro);   // OK: negro es un Color
// pintar(origen);  // ERROR: origen es un Punto, no un Color
```

Sin tuple structs, si usaras tuplas simples `(i32, i32, i32)` para ambos, el compilador no podria distinguirlas y aceptaria pasar un punto donde se espera un color. Los tuple structs te dan **seguridad de tipos** con una sintaxis minima.

Accedes a los campos por indice, igual que en una tupla:

```rust
let rojo = negro.0;   // primer campo
let verde = negro.1;  // segundo campo
let azul = negro.2;   // tercer campo
```

### Unit-Like Structs

Los unit-like structs son structs que no tienen ningun campo. Puede parecer inutil a primera vista, pero existen por una razon: puedes **implementar traits** en ellos. Esto es comun cuando necesitas un tipo que represente un comportamiento o una configuracion, pero no necesita almacenar datos.

```rust
struct SiempreIgual;

impl PartialEq for SiempreIgual {
    fn eq(&self, _other: &Self) -> bool {
        true  // cualquier instancia es igual a cualquier otra
    }
}

let a = SiempreIgual;
let b = SiempreIgual;
assert!(a == b);  // siempre true
```

Los veras mas adelante cuando trabajes con traits y programacion generica. Por ahora, solo necesitas saber que existen y que su utilidad radica en ser "portadores de comportamiento" sin datos.

---

## Methods con `impl`

Un bloque `impl` (abreviatura de *implementation*) es donde defines el **comportamiento** de un struct. Mientras que la definicion del struct declara *que datos* contiene, el bloque `impl` declara *que puede hacer* con esos datos.

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

Hay dos categorias de funciones dentro de un bloque `impl`:

- **Methods**: su primer parametro es alguna forma de `self` (`&self`, `&mut self`, o `self`). Se llaman sobre una instancia con la sintaxis de punto: `rect.area()`. El punto clave es que un method opera sobre una instancia existente.
- **Funciones asociadas**: NO tienen `self` como primer parametro. Se llaman sobre el tipo con la sintaxis `::`, como `Rectangulo::cuadrado(5.0)`. No necesitan una instancia existente. Son analogas a los "metodos estaticos" de otros lenguajes y se usan frecuentemente como constructores.

Puedes tener **multiples bloques `impl`** para el mismo struct. El compilador los fusiona internamente, asi que no hay diferencia funcional. Esto es util para organizar el codigo o para separar implementaciones de distintos traits:

```rust
impl Rectangulo {
    fn area(&self) -> f64 {
        self.ancho * self.alto
    }
}

impl Rectangulo {
    fn cuadrado(lado: f64) -> Self {
        Self { ancho: lado, alto: lado }
    }
}
// Ambos bloques se combinan. Rectangulo tiene tanto area() como cuadrado().
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

Un **enum** (abreviatura de *enumeration*) define un tipo que puede ser **una de varias variantes posibles**. Mientras que un struct dice "este valor tiene todos estos campos", un enum dice "este valor es una de estas opciones".

### Que problema resuelven

En muchos lenguajes, cuando necesitas representar un valor que puede ser una de varias opciones, usas constantes numericas o strings:

```text
// En otros lenguajes (pseudocodigo):
const DIRECCION_NORTE = 0;
const DIRECCION_SUR = 1;
// ... nada impide que alguien pase un 42 donde se espera una direccion
```

Esto no es type-safe: nada impide que pases un valor invalido. En Rust, un enum es un tipo cerrado. Si una funcion recibe `Direccion`, el compilador **garantiza** que solo puede ser `Norte`, `Sur`, `Este` u `Oeste`. No hay forma de pasar un valor invalido.

### Enums simples

En su forma mas basica, un enum lista las variantes posibles sin datos asociados:

```rust
enum Direccion {
    Norte,
    Sur,
    Este,
    Oeste,
}

let dir = Direccion::Norte;
```

Cada variante es un valor posible del tipo `Direccion`. El enum es un tipo "cerrado": no puedes agregar nuevas variantes sin modificar la definicion del enum. Esto es una ventaja, porque significa que cualquier codigo que maneje todas las variantes es exhaustivo y correcto.

### Enums con Datos

Aqui es donde los enums de Rust se vuelven realmente poderosos. Cada variante puede tener **datos asociados de tipos diferentes**. Un solo tipo enum puede contener formas de datos completamente distintas:

```rust
enum Mensaje {
    Salir,                        // sin datos
    Mover { x: i32, y: i32 },    // campos nombrados (struct-like)
    Texto(String),                // un String
    Color(i32, i32, i32),         // tres enteros (tuple-like)
}

let m1 = Mensaje::Salir;
let m2 = Mensaje::Mover { x: 10, y: 20 };
let m3 = Mensaje::Texto(String::from("hola"));
let m4 = Mensaje::Color(255, 0, 0);
```

Piensa en lo que necesitarias en otros lenguajes para lograr esto: una interfaz `Mensaje` con cuatro clases que la implementen, cada una con sus propios campos. En Rust, un solo enum reemplaza toda esa jerarquia. Cada variante funciona como un "mini-struct" con su propia forma de datos, pero todas comparten el mismo tipo `Mensaje`.

Esto es especialmente util para modelar protocolos, eventos de interfaz, resultados de operaciones, o cualquier situacion donde un valor puede tomar formas diferentes.

### Pattern matching con `match`

La forma principal de trabajar con enums es `match`, que te permite ejecutar codigo diferente segun la variante. Lo que hace a `match` especial es que es **exhaustivo**: el compilador te obliga a cubrir **todas** las variantes del enum. Si agregas una nueva variante al enum, el compilador te senala exactamente en que lugares del codigo falta manejarla.

```rust
fn describir(dir: Direccion) {
    match dir {
        Direccion::Norte => println!("Vamos al norte"),
        Direccion::Sur => println!("Vamos al sur"),
        Direccion::Este => println!("Vamos al este"),
        Direccion::Oeste => println!("Vamos al oeste"),
    }
}
```

Si comentaras una de las ramas, el compilador te daria un error indicando que no cubriste todas las variantes. Esto es una red de seguridad enorme: cuando tu programa evoluciona y agregas nuevas opciones, el compilador te guia a todos los lugares que necesitan actualizarse.

Con enums que tienen datos, `match` ademas te permite **extraer** esos datos:

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

Cada rama del `match` desestructura la variante y te da acceso a los datos internos con nombres que tu eliges.

---

## Option\<T\>: El Enum mas Importante

### Por que Rust no tiene null

En la mayoria de lenguajes de programacion, cualquier variable de tipo referencia puede ser `null`. Esto significa que cada vez que usas una variable, existe la posibilidad de que sea null, y si no la verificas, tu programa crashea en tiempo de ejecucion. Tony Hoare, el inventor de null, lo llamo su "billion dollar mistake" (error de mil millones de dolares) por la cantidad de bugs, crashes y vulnerabilidades de seguridad que ha causado a lo largo de las decadas.

El problema fundamental no es el concepto de "ausencia de valor" (eso es legitimo y necesario), sino que null es **invisible en el sistema de tipos**. Un `String` en Java podria ser null, y el compilador no te obliga a verificarlo. Puedes escribir `usuario.nombre.length()` y el compilador lo acepta sin quejarse, aunque `nombre` podria ser null y tu programa explotaria en produccion.

Rust toma un enfoque diferente: **no existe null**. Si declaras una variable de tipo `String`, esa variable **siempre** contiene un `String` valido. Punto.

### Option: la alternativa segura

Cuando un valor podria no existir, Rust usa el enum `Option<T>`:

```rust
enum Option<T> {
    Some(T),  // Hay un valor de tipo T
    None,     // No hay valor
}
```

La `T` es un tipo generico: `Option<i32>` puede contener un entero o nada, `Option<String>` puede contener un string o nada, etc.

```rust
let numero: Option<i32> = Some(42);
let vacio: Option<i32> = None;
```

La diferencia clave con null es que `Option<i32>` e `i32` son **tipos diferentes**. No puedes usar un `Option<i32>` donde se espera un `i32` sin antes extraer el valor. El compilador te **obliga** a manejar el caso `None` antes de poder usar el valor. Esto convierte lo que en otros lenguajes seria un crash en tiempo de ejecucion en un error de compilacion.

### Como usar Option

La forma mas completa es `match`, que maneja ambos casos explicitamente:

```rust
let numero: Option<i32> = Some(42);

match numero {
    Some(n) => println!("El numero es: {n}"),
    None => println!("No hay numero"),
}
```

Cuando solo te importa **un caso** y quieres ignorar el otro, `if let` es mas conciso:

```rust
let nombre: Option<String> = Some(String::from("Ana"));

if let Some(n) = nombre {
    println!("Hola, {n}!");
}
// Si nombre fuera None, simplemente no hace nada
```

Para extraer el valor directamente, tienes varios metodos:

```rust
let x: Option<i32> = Some(10);
let y: Option<i32> = None;

// .unwrap() extrae el valor, pero hace panic! si es None
// Usalo solo cuando estas SEGURO de que hay un valor, o en prototipos rapidos
let valor = x.unwrap();  // 10
// let boom = y.unwrap();  // panic! en tiempo de ejecucion

// .unwrap_or(default) extrae el valor o devuelve un valor por defecto
let a = x.unwrap_or(0);  // 10 (tiene valor, usa ese)
let b = y.unwrap_or(0);  // 0  (no tiene valor, usa el default)
```

### Cuando usar Option

Usa `Option<T>` siempre que un valor podria legitimamente no existir:

- Buscar un elemento en una coleccion (podria no encontrarse)
- Un campo opcional en un struct (segundo nombre, telefono alternativo)
- El resultado de una operacion que podria no tener sentido (dividir por cero)
- El primer o ultimo elemento de una lista que podria estar vacia

```rust
struct Persona {
    nombre: String,
    segundo_nombre: Option<String>,  // no todos tienen segundo nombre
    edad: u32,
}

let ana = Persona {
    nombre: String::from("Ana"),
    segundo_nombre: None,  // sin segundo nombre
    edad: 28,
};

let juan = Persona {
    nombre: String::from("Juan"),
    segundo_nombre: Some(String::from("Carlos")),
    edad: 35,
};
```

La regla es simple: si algo siempre tiene un valor, usa el tipo directamente (`String`, `i32`, etc.). Si algo podria no tener un valor, envuelvelo en `Option`. El compilador se encarga del resto.

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
