# Capitulo 2: Variables, Types y Functions

> Referencia: *The Rust Programming Language* - Cap. 3: Common Programming Concepts

---

## Variables y Mutabilidad

En Rust, las variables son **immutable** (inmutables) por defecto. Esto no es un capricho del lenguaje, sino una decision de diseno deliberada con varias razones de fondo:

1. **Prevencion de bugs**: Si una variable no puede cambiar, eliminas una categoria entera de errores. No puedes modificar accidentalmente un valor que deberia mantenerse constante. En programas grandes, es dificil rastrear donde y cuando cambia una variable; si es inmutable, simplemente no cambia.

2. **Concurrencia segura**: Cuando multiples hilos de ejecucion acceden a la misma variable, los bugs mas dificiles de detectar ocurren cuando uno la modifica mientras otro la lee. Si la variable es inmutable, cualquier hilo puede leerla sin riesgo. Esta es una de las bases del modelo de seguridad de Rust.

3. **Intencion explicita**: Cuando ves `let mut x = 5;`, sabes inmediatamente que esa variable va a cambiar en algun momento. Es una senal para quien lee el codigo. Si no tiene `mut`, sabes que puedes confiar en que su valor no cambiara.

```rust
let x = 5;       // immutable - NO se puede cambiar
let mut y = 10;   // mutable - SI se puede cambiar
y = 20;           // OK
// x = 10;        // ERROR: cannot assign twice to immutable variable
```

### Que pasa si intentas mutar una variable inmutable?

El compilador de Rust te detiene con un mensaje de error claro. Si escribes:

```rust
let x = 5;
x = 10;
```

Obtendras un error como este:

```
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:3:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     x = 10;
  |     ^^^^^^ cannot assign twice to immutable variable
```

Observa que el compilador no solo te dice que esta mal, sino que te sugiere la solucion: agregar `mut`. Los mensajes de error de Rust son excepcionalmente utiles y pedagogicos.

### Cuando usar `mut` y cuando no

Como regla general: **no uses `mut` a menos que lo necesites**. Empieza siempre con `let` inmutable. Si el compilador te dice que necesitas mutar la variable, entonces agrega `mut`.

Casos tipicos donde SI necesitas `mut`:

```rust
// Acumuladores y contadores
let mut suma = 0;
for i in 1..=10 {
    suma += i;
}

// Variables que cambian segun condiciones
let mut mensaje = String::from("Procesando");
if hubo_error {
    mensaje = String::from("Error encontrado");
}

// Buffers y colecciones que crecen
let mut nombres = Vec::new();
nombres.push("Ana");
nombres.push("Luis");
```

Casos donde NO necesitas `mut` (y no deberias usarlo):

```rust
// Valores calculados una sola vez
let area = base * altura;
let nombre_completo = format!("{} {}", nombre, apellido);

// Valores de configuracion que no cambian
let puerto = 8080;
let max_intentos = 3;

// Transformaciones con shadowing (en vez de mutar)
let texto = "  hola mundo  ";
let texto = texto.trim();      // shadow, no mutacion
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

Un tema a tener en consideracion es que en general se pueden crear multiples espacios en memoria cada vez que se crea una nueva variable cuando se aplica **shadowing**, aunque Rust libera los anteriores cuando salen del scope.

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

Aqui esta lo importante: si se crean dos espacios distintos en el heap, pero el primero se libera inmediatamente cuando el nuevo `x` lo "sombrea", porque el `String` anterior pierde su owner.

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

### Inferencia de Tipos (Type Inference)

Sin embargo, eso no significa que siempre tengas que escribir el tipo explicitamente. Rust tiene un sistema de **inferencia de tipos** muy potente: el compilador puede deducir el tipo de una variable a partir del valor que le asignas y de como la usas despues.

```rust
// El compilador infiere los tipos automaticamente
let x = 5;           // infiere i32 (el tipo entero por defecto)
let y = 3.14;        // infiere f64 (el tipo flotante por defecto)
let activo = true;   // infiere bool
let letra = 'A';     // infiere char
let nombre = "Rust"; // infiere &str
```

Puedes tambien ser explicito con el tipo cuando quieras mayor claridad:

```rust
// Anotaciones de tipo explicitas (mismo resultado, mas legible en algunos contextos)
let x: i32 = 5;
let y: f64 = 3.14;
let activo: bool = true;
```

Pero hay situaciones donde **debes** anotar el tipo porque el compilador no tiene suficiente informacion para inferirlo:

```rust
// OBLIGATORIO: parse() puede devolver muchos tipos, el compilador no sabe cual quieres
let numero: i32 = "42".parse().expect("No es un numero");

// Sin la anotacion, obtienes este error:
// let numero = "42".parse().expect("No es un numero");
// error[E0284]: type annotations needed
//   --> src/main.rs:2:9
//   |
// 2 |     let numero = "42".parse().expect("No es un numero");
//   |         ^^^^^^ consider giving `numero` a type

// Alternativa: usar la sintaxis "turbofish" en vez de anotar la variable
let numero = "42".parse::<i32>().expect("No es un numero");
```

La regla practica: si el compilador puede deducirlo, no hace falta anotarlo. Si no puede, te pedira que lo hagas.

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

#### Enteros con signo vs sin signo (i32 vs u32)

La diferencia fundamental es si el tipo puede representar numeros negativos:

- **Con signo** (`i8`, `i16`, `i32`, etc.): Almacenan numeros positivos Y negativos. La "i" viene de "integer" (con signo). Un `i32` almacena valores desde -2,147,483,648 hasta 2,147,483,647.
- **Sin signo** (`u8`, `u16`, `u32`, etc.): Solo almacenan numeros positivos (y cero). La "u" viene de "unsigned". Un `u32` almacena valores desde 0 hasta 4,294,967,295.

Cuando usar cada uno:

```rust
let temperatura: i32 = -15;    // i32: las temperaturas pueden ser negativas
let edad: u32 = 25;            // u32: una edad nunca es negativa
let indice: usize = 0;         // usize: indices de arrays siempre son positivos
                                // (usize tiene el tamano del puntero: 64 bits en sistemas de 64 bits)
let byte: u8 = 255;            // u8: valores de bytes van de 0 a 255
```

El tipo por defecto para enteros es `i32`, que es una buena opcion general. Usa `u32` o `usize` cuando sepas que el valor nunca sera negativo.

#### Integer Overflow (Desbordamiento de enteros)

Que pasa si intentas almacenar un valor mas grande de lo que un tipo puede contener?

```rust
let x: u8 = 255;   // u8 puede almacenar hasta 255
// let y: u8 = 256; // ERROR en compile time: literal out of range for `u8`
```

Si el overflow ocurre en tiempo de ejecucion (por ejemplo, sumando dos variables), el comportamiento depende del modo de compilacion:

- **En modo debug** (`cargo build`): El programa hace **panic** (se detiene con un error). Esto te ayuda a detectar bugs durante el desarrollo.
- **En modo release** (`cargo build --release`): El valor hace **wrapping** (da la vuelta). Un `u8` con valor 255 + 1 se convierte en 0. Esto es eficiente pero puede causar bugs silenciosos.

```rust
// En modo debug, esto causa panic en runtime:
let mut x: u8 = 255;
// x += 1;  // panic: 'attempt to add with overflow'

// Si necesitas wrapping intencional, Rust ofrece metodos explicitos:
let resultado = x.wrapping_add(1);   // resultado = 0, sin panic
let resultado = x.saturating_add(1); // resultado = 255 (se queda en el maximo)
let (resultado, overflow) = x.overflowing_add(1); // resultado = 0, overflow = true
```

#### Casting con `as`

Rust no convierte tipos automaticamente. Si necesitas convertir un valor de un tipo a otro, debes hacerlo explicitamente con la keyword `as`:

```rust
let entero: i32 = 42;
let flotante: f64 = entero as f64;       // i32 -> f64: 42.0
let pequeno: i8 = entero as i8;          // i32 -> i8: 42 (cabe, no hay problema)

let pi: f64 = 3.14159;
let truncado: i32 = pi as i32;           // f64 -> i32: 3 (se trunca, no se redondea)

let literal: f64 = 5i32 as f64;          // tambien funciona con literales con sufijo de tipo

// Esto es comun en los ejercicios de este capitulo:
let celsius: f64 = 100.0;
let fahrenheit: f64 = (celsius * 9.0 / 5.0) + 32.0;

// O si tienes un entero y necesitas hacer division con decimales:
let a: i32 = 9;
let b: i32 = 5;
let resultado: f64 = a as f64 / b as f64;  // 1.8, no 1
```

Sin `as`, intentar mezclar tipos produce un error de compilacion:

```rust
let x: i32 = 5;
let y: f64 = 3.0;
// let z = x + y;  // ERROR: cannot add `f64` to `i32`
let z = x as f64 + y;  // OK: 8.0
```

Esto es una decision de diseno de Rust: las conversiones implicitas son una fuente comun de bugs en otros lenguajes (especialmente cuando se pierde precision), asi que Rust te obliga a ser explicito.

#### El tipo `char` y Unicode

En Rust, `char` ocupa **4 bytes** (32 bits), no 1 byte como en C. Esto es porque `char` representa un **Unicode Scalar Value**, lo que significa que puede almacenar cualquier caracter Unicode: letras acentuadas, caracteres chinos, emojis, etc.

```rust
let letra: char = 'a';        // ASCII, pero almacenado como Unicode
let acento: char = 'n';       // caracter con tilde
let emoji: char = '🦀';       // emoji del cangrejo de Rust
let chino: char = '中';        // caracter chino

// Todos ocupan 4 bytes en memoria
println!("Tamano de char: {} bytes", std::mem::size_of::<char>()); // 4
```

Nota: `char` (con comillas simples) es un solo caracter Unicode. Una cadena de texto `"hola"` (con comillas dobles) es un `&str`, que es una cosa completamente distinta. Los strings en Rust se codifican en UTF-8, donde cada caracter puede ocupar entre 1 y 4 bytes. El tipo `char` siempre ocupa 4 bytes para poder almacenar cualquier caracter posible.

### Compound Types (Tipos Compuestos)

#### Tuple (Tupla)

Una tupla agrupa valores de **diferentes tipos** en una estructura de longitud fija. Una vez declarada, su tamano no puede cambiar.

```rust
let tupla: (i32, f64, char) = (500, 6.4, 'R');
let (x, y, z) = tupla;         // destructuring
let primero = tupla.0;          // acceso por indice
```

**Destructuring** es una forma muy comoda de extraer todos los valores de una tupla en variables separadas de una sola vez:

```rust
let punto = (10.0, 20.0);
let (x, y) = punto;            // x = 10.0, y = 20.0

// Puedes ignorar valores con _
let datos = ("Ana", 25, true);
let (nombre, _, activo) = datos;  // ignoramos la edad
```

**Acceso por indice**: Tambien puedes acceder a elementos individuales con la notacion de punto seguido del indice (empezando en 0):

```rust
let tupla = (100, "hola", 3.14);
let primero = tupla.0;   // 100
let segundo = tupla.1;   // "hola"
let tercero = tupla.2;   // 3.14
```

**Cuando usar tuplas vs structs**: Las tuplas son utiles para agrupaciones rapidas y temporales, como retornar multiples valores desde una funcion. Pero cuando la agrupacion tiene un significado semantico claro (por ejemplo, un "punto" con x e y), es mejor usar un struct (que veras en capitulos posteriores), porque los campos tienen nombres y el codigo es mas legible.

```rust
// Tupla: funciona, pero que significa cada campo?
let persona: (&str, u32) = ("Ana", 25);
let edad = persona.1;  // no es obvio que .1 es la edad

// Struct (veras esto despues): mucho mas claro
// struct Persona { nombre: &str, edad: u32 }
// let persona = Persona { nombre: "Ana", edad: 25 };
// let edad = persona.edad;  // obvio
```

**La tupla vacia `()` - Unit Type**: Existe una tupla especial sin ningun valor: `()`. Se llama el **unit type** y representa "sin valor" o "nada util que retornar". Es el equivalente a `void` en C o Java, pero en Rust es un tipo real con un unico valor posible (tambien escrito `()`).

```rust
// Una funcion que no retorna nada, en realidad retorna ()
fn saludar() {
    println!("Hola!");
}
// Es equivalente a:
fn saludar() -> () {
    println!("Hola!");
}

// Un statement (instruccion con ;) produce el valor ()
let x = {
    let y = 5;
    y + 1;   // el ; convierte esto en statement, el bloque retorna ()
};
// x es de tipo ()
```

#### Array

Un array es una coleccion de elementos del **mismo tipo** con **longitud fija** conocida en tiempo de compilacion. Se almacena en el **stack**, lo que lo hace muy eficiente.

```rust
let numeros: [i32; 5] = [1, 2, 3, 4, 5];
let primero = numeros[0];       // acceso por indice
let ceros = [0; 10];            // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
```

**Tamano fijo en tiempo de compilacion**: El tamano del array es parte de su tipo. Un `[i32; 5]` y un `[i32; 3]` son tipos DIFERENTES. No puedes asignar uno al otro, ni cambiar el tamano despues de crearlo.

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
// No puedes hacer esto:
// let b: [i32; 3] = a;  // ERROR: tipos incompatibles
// a.push(6);            // ERROR: los arrays no tienen push, su tamano es fijo
```

**Acceso fuera de rango (bounds checking)**: A diferencia de C (donde acceder fuera de un array puede leer memoria basura o corromper datos), Rust verifica los limites en tiempo de ejecucion. Si intentas acceder a un indice que no existe, el programa hace **panic** (se detiene con un error claro) en lugar de continuar con datos corruptos:

```rust
let numeros = [1, 2, 3, 4, 5];
// let fuera = numeros[10];  // panic en runtime:
                              // index out of bounds: the len is 5 but the index is 10
```

Esto es una garantia de seguridad de memoria: Rust nunca te deja leer o escribir memoria que no te pertenece.

**Array vs Vec**: Si necesitas una coleccion que pueda crecer o reducirse en tiempo de ejecucion, necesitas un `Vec<T>` (vector), que se almacena en el heap. Los vectores se cubren en capitulos posteriores. La regla simple:

- Usas `[T; N]` (array) cuando sabes el tamano exacto en tiempo de compilacion y no va a cambiar.
- Usas `Vec<T>` cuando el tamano es dinamico o no lo conoces de antemano.

---

## Loops Basicos (Adelanto)

Para iterar sobre colecciones o repetir codigo, Rust tiene loops. Aqui una introduccion basica del `for` loop que necesitaras para los ejercicios. Los loops se cubriran en detalle en el Capitulo 5.

### For Loop

```rust
// Iterar sobre un rango de numeros
for i in 0..5 {
    println!("{}", i);  // imprime 0, 1, 2, 3, 4
}

// Iterar sobre un array con .iter()
let numeros = [10, 20, 30, 40, 50];
for numero in numeros.iter() {
    println!("{}", numero);
}

// Iterar y modificar: suma de array
let mut suma = 0;
for &n in numeros.iter() {  // &n extrae el valor
    suma += n;
}
println!("Suma: {}", suma);  // 150
```

**Nota:** Los loops se explicaran completamente en el Capitulo 5 (Control de Flujo). Por ahora, solo necesitas saber que `for elemento in coleccion.iter()` te permite recorrer cada elemento.

---

## Constants

Las **constants** son siempre inmutables y deben tener tipo explicito:

```rust
const MAX_PUNTOS: u32 = 100_000;
const PI: f64 = 3.14159265;
```

### Diferencias entre `const` y `let`

Aunque tanto `const` como `let` (sin `mut`) producen valores que no puedes cambiar, son mecanismos fundamentalmente diferentes:

| Caracteristica | `const` | `let` (inmutable) |
|---|---|---|
| Requiere type annotation | Siempre | Solo cuando el compilador no puede inferir |
| Cuando se evalua | Compile time | Runtime |
| Puede ser resultado de una funcion | No | Si |
| Scope global | Si | No (solo dentro de funciones/bloques) |
| Shadowing | No se puede hacer shadow | Si se puede hacer shadow |
| Naming convention | `SCREAMING_SNAKE_CASE` | `snake_case` |

**`const` debe ser un valor conocido en compile time**. No puedes asignarle el resultado de una funcion ni de ninguna operacion que se ejecute en runtime:

```rust
const MAX: u32 = 100;                    // OK: valor literal
const DOBLE: u32 = MAX * 2;              // OK: operacion entre constantes, evaluable en compile time
const AREA: f64 = 3.14159 * 10.0 * 10.0; // OK: aritmetica con literales

// const AHORA: String = obtener_hora(); // ERROR: no se puede evaluar en compile time
// const RANDOM: u32 = generar_random(); // ERROR: depende del runtime
```

**Naming convention**: Las constantes se escriben en `SCREAMING_SNAKE_CASE` (todo en mayusculas, palabras separadas por guion bajo). Esto es una convencion fuerte en Rust y el compilador te dara un warning si no la sigues:

```rust
const MAX_CONEXIONES: u32 = 100;
const VELOCIDAD_LUZ_M_S: f64 = 299_792_458.0;
const SEGUNDOS_POR_HORA: u32 = 3_600;
```

**Cuando usar `const` vs `let` inmutable?**

- Usa `const` para valores que son verdaderamente constantes universales y conocidos antes de ejecutar el programa: limites, configuraciones fijas, constantes matematicas, factores de conversion.
- Usa `let` para valores que se calculan durante la ejecucion, aunque no vayan a cambiar despues de asignados: resultados de funciones, datos leidos del usuario, calculos basados en input.

```rust
const GRAVEDAD: f64 = 9.81;          // constante universal -> const

fn calcular_caida(segundos: f64) -> f64 {
    let distancia = 0.5 * GRAVEDAD * segundos * segundos;  // calculado en runtime -> let
    distancia
}
```

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

### Los parametros siempre necesitan type annotation

A diferencia de las variables locales (donde el compilador puede inferir el tipo), los parametros de una funcion **siempre** deben tener su tipo anotado explicitamente. El compilador no infiere tipos en las firmas de funciones. Esta es una decision de diseno intencional: las firmas de funciones son contratos entre quien escribe la funcion y quien la usa, y deben ser explicitas.

```rust
// CORRECTO: todos los parametros con tipo
fn calcular_area(base: f64, altura: f64) -> f64 {
    base * altura / 2.0
}

// ERROR: falta el tipo del parametro
// fn calcular_area(base, altura) -> f64 {
//     base * altura / 2.0
// }
// error: expected one of `:`, `@`, or `|`, found `,`
```

### Scope de las variables

Las variables declaradas dentro de una funcion solo existen dentro de esa funcion. No son accesibles desde fuera. Esto se llama **scope** (alcance):

```rust
fn crear_mensaje() -> String {
    let mensaje = String::from("Hola desde la funcion");
    mensaje  // retornamos el valor, no la variable
}

fn main() {
    let resultado = crear_mensaje();
    println!("{}", resultado);     // OK: resultado tiene el valor retornado
    // println!("{}", mensaje);    // ERROR: `mensaje` no existe aqui
                                   // `mensaje` solo existe dentro de crear_mensaje()
}
```

Lo mismo aplica a los bloques `{}` dentro de una funcion:

```rust
fn main() {
    let x = 5;
    {
        let y = 10;
        println!("{} {}", x, y);  // OK: x y y son accesibles aqui
    }
    // println!("{}", y);  // ERROR: y ya no existe fuera del bloque
    println!("{}", x);     // OK: x sigue viva
}
```

### Funciones con multiples parametros y tipos de retorno

Veamos varios ejemplos de funciones para familiarizarnos con las combinaciones mas comunes:

```rust
// Multiples parametros de diferentes tipos
fn describir_persona(nombre: &str, edad: u32, altura_cm: f64) -> String {
    format!("{} tiene {} anios y mide {:.1} cm", nombre, edad, altura_cm)
}

// Retornando una tupla (multiples valores)
fn dividir(dividendo: i32, divisor: i32) -> (i32, i32) {
    let cociente = dividendo / divisor;
    let resto = dividendo % divisor;
    (cociente, resto)  // retorna una tupla
}

// Uso:
// let (cociente, resto) = dividir(17, 5);  // cociente = 3, resto = 2

// Retornando bool
fn es_par(numero: i32) -> bool {
    numero % 2 == 0
}

// Funcion que no retorna nada util (retorna () implicitamente)
fn imprimir_linea(caracter: char, longitud: usize) {
    for _ in 0..longitud {
        print!("{}", caracter);
    }
    println!();
}
// Uso: imprimir_linea('-', 40);  // imprime: ----------------------------------------
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
