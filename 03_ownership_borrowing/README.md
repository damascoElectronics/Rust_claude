# Capitulo 3: Ownership y Borrowing

> Referencia: *The Rust Programming Language* - Cap. 4: Understanding Ownership

---

## El Concepto mas Importante de Rust

**Ownership** es, sin exageracion, el concepto central de Rust. Es el sistema que hace que todo lo demas funcione: la seguridad de memoria, la ausencia de garbage collector, la concurrencia sin data races. Si entiendes ownership, entiendes Rust.

### El problema que resuelve

Para entender por que existe ownership, primero hay que entender el problema historico de la gestion de memoria:

**En C y C++**, el programador gestiona la memoria manualmente. Usas `malloc()` para reservar memoria y `free()` para liberarla (o `new`/`delete` en C++). Esto te da control total y rendimiento maximo, pero abre la puerta a bugs devastadores:

- **Use-after-free**: acceder a memoria que ya fue liberada. El programa lee basura o se cuelga.
- **Double free**: llamar a `free()` dos veces sobre la misma direccion de memoria. Corrompe el heap y puede causar crashes aleatorios o vulnerabilidades de seguridad.
- **Memory leaks**: olvidarte de llamar a `free()`. El programa consume cada vez mas memoria hasta que se agota.
- **Dangling pointers**: un puntero que apunta a memoria que ya no es valida.

Estos bugs son especialmente peligrosos porque muchas veces no aparecen de inmediato. Un programa puede funcionar "bien" durante meses y de repente fallar en produccion. Los use-after-free son ademas una de las principales fuentes de vulnerabilidades de seguridad.

**Los lenguajes con Garbage Collector** (Java, Python, Go, C#, JavaScript) resuelven estos problemas con un recolector de basura: un proceso que se ejecuta en segundo plano, detecta que memoria ya no se usa y la libera automaticamente. Esto elimina toda una categoria de bugs, pero tiene un costo:

- **Pausas impredecibles**: el garbage collector puede pausar tu programa en cualquier momento para hacer limpieza.
- **Mayor consumo de memoria**: necesita memoria extra para rastrear las referencias.
- **Menor rendimiento**: el rastreo y la recoleccion consumen ciclos de CPU.
- **Menor control**: no puedes decidir exactamente cuando se libera un recurso.

**Rust propone una tercera via**: en lugar de que el programador gestione la memoria (C/C++) o que un proceso en runtime lo haga (garbage collector), el **compilador** verifica un conjunto de reglas de ownership en **compile time**. Si tu codigo cumple las reglas, compila y se garantiza que no habra errores de memoria. Si no las cumple, el compilador te muestra un error claro.

El resultado: **seguridad de memoria con rendimiento de C/C++**. Sin garbage collector, sin pausas, sin costo en runtime. Las verificaciones suceden antes de que tu programa se ejecute.

---

## Las 3 Reglas del Ownership

Todo el sistema se construye sobre tres reglas simples. Son faciles de enunciar pero tienen implicaciones profundas.

### Regla 1: Cada valor tiene un owner

Cada dato en Rust tiene exactamente una variable que es su "duenio" (owner). No existe dato sin duenio, y no existe dato con dos duenios.

```rust
let s = String::from("hola");  // s es el owner de este String
let x = 42;                     // x es el owner de este i32
let v = vec![1, 2, 3];          // v es el owner de este Vec
```

Esto puede parecer obvio, pero es una restriccion fuerte. En C/C++, multiples punteros pueden apuntar al mismo dato sin que nadie sea "el duenio". En Rust, siempre hay exactamente un responsable.

### Regla 2: Solo puede haber un owner a la vez

Si transfieres un valor a otra variable, la variable original deja de ser el owner. No pueden existir dos owners simultaneos del mismo dato. Esta restriccion es la que permite a Rust saber siempre, de forma inequivoca, quien es responsable de liberar la memoria.

```rust
let s1 = String::from("hola");
let s2 = s1;  // el ownership se transfiere a s2
// s1 ya no es owner de nada. El compilador no te deja usarlo.
```

### Regla 3: Cuando el owner sale del scope, el valor se elimina (drop)

Aqui es donde Rust libera la memoria automaticamente. Cuando una variable deja de existir (sale de su scope), Rust llama automaticamente a una funcion especial llamada `drop()` que libera los recursos asociados.

**Que es el scope?** El scope (ambito) de una variable es la region del codigo donde esa variable es valida. En Rust, los scopes se definen con llaves `{}`. Una variable es valida desde el punto donde se declara hasta la llave de cierre del bloque donde fue declarada.

```rust
fn main() {                         // empieza el scope de main
    let x = 42;                     // x es valido desde aqui

    {                               // empieza un scope interno
        let y = String::from("hola");  // y es valido desde aqui
        println!("{x} {y}");           // ambos son validos
    }                               // y sale del scope -> drop(y) -> memoria liberada

    // println!("{y}");             // ERROR: y ya no existe
    println!("{x}");                // OK: x todavia esta en su scope

}                                   // x sale del scope -> drop(x)
```

**"Salir del scope"** significa que la ejecucion llega a la llave de cierre `}` del bloque donde la variable fue declarada. En ese preciso momento, Rust inserta automaticamente la llamada a `drop()`. No tienes que escribirlo tu; es un mecanismo invisible e infalible.

Un ejemplo mas detallado con scopes anidados:

```rust
fn main() {
    let a = String::from("alfa");       // a: valida

    {   // bloque 1
        let b = String::from("beta");   // a: valida, b: valida

        {   // bloque 2
            let c = String::from("gamma");  // a, b, c: validas
            println!("{a} {b} {c}");
        }   // c se destruye aqui (drop)

        println!("{a} {b}");
        // println!("{c}");  // ERROR: c ya no existe

    }   // b se destruye aqui (drop)

    println!("{a}");
    // println!("{b}");  // ERROR: b ya no existe

}   // a se destruye aqui (drop)
```

Gracias a esta regla, Rust sabe exactamente cuando liberar cada recurso. No necesita rastrear nada en runtime: el compilador puede ver los scopes y sabe en que punto exacto insertar cada `drop()`.

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
- En Rust se libera automaticamente via **ownership** (en C/C++ lo haces manual con malloc/free o new/delete)

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

Cuando asignas un valor a otra variable o lo pasas a una funcion, sucede una de dos cosas: un **move** o un **copy**. Entender la diferencia es fundamental.

### Move: transferir ownership

Cuando trabajas con tipos que almacenan datos en el heap (como `String`, `Vec<T>`, `Box<T>`), la asignacion transfiere el ownership. La variable original queda invalidada.

**Por que existe el Move?** Piensa en lo que pasaria sin el:

```
Sin Move (hipotetico, Rust NO permite esto):

Stack                    Heap
┌──────────────┐
│ s1 ──────────────────> ┌──────────┐
│ ptr, len: 4  │    ┌──> │ "hola"   │
└──────────────┘    │    └──────────┘
                    │
┌──────────────┐    │
│ s2 ───────────────┘
│ ptr, len: 4  │
└──────────────┘

Problema: cuando s1 y s2 salgan del scope, ambos
llamarian drop() sobre la misma memoria del heap.
Eso es un DOUBLE FREE -> crash o corrupcion.
```

El move resuelve esto de forma elegante: al hacer `let s2 = s1`, Rust copia los datos del stack (el puntero, la longitud, la capacidad) pero **invalida s1**. Asi, solo s2 puede llamar a `drop()`.

```rust
let s1 = String::from("hola");
let s2 = s1;  // s1 se MUEVE a s2. s1 queda invalidado.

// println!("{s1}");  // ERROR: value used after move
println!("{s2}");     // OK: s2 es el unico owner
```

```
Despues del Move:

Stack                    Heap
┌──────────────┐
│ s1 (invalid) │         ┌──────────┐
│ ------------ │    ┌──> │ "hola"   │
└──────────────┘    │    └──────────┘
                    │
┌──────────────┐    │
│ s2 ───────────────┘
│ ptr, len: 4  │
└──────────────┘

Solo s2 llamara drop(). No hay double free.
```

El move es una operacion barata: solo copia unos pocos bytes en el stack (el puntero y los metadatos). Los datos del heap no se tocan. Es tan rapido como una copia superficial, pero seguro.

### Copy: duplicar el valor

Los tipos que viven completamente en el stack y tienen un tamanio fijo y conocido implementan el **trait Copy**. Cuando asignas uno de estos valores a otra variable, se copia bit a bit automaticamente. Ambas variables son independientes y validas.

```rust
let x = 5;
let y = x;  // Se COPIA el valor. x sigue siendo valido
println!("x={x}, y={y}"); // OK: ambos tienen su propia copia
```

**Que es un trait?** Por ahora, piensa en un trait como una "capacidad" que un tipo puede tener. Si un tipo implementa el trait `Copy`, Rust sabe que puede duplicarlo de forma segura copiando sus bytes. No todos los tipos pueden ser `Copy`; solo aquellos cuya copia es barata y no requiere logica adicional.

**Tipos que implementan Copy** (todos son de tamanio fijo y viven en el stack):

- Enteros: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- Flotantes: `f32`, `f64`
- Booleanos: `bool`
- Caracteres: `char`
- Tuplas de tipos Copy: `(i32, f64)` es Copy, pero `(i32, String)` NO (porque String no es Copy)
- Arrays de tipos Copy: `[i32; 5]` es Copy
- Referencias: `&T` es Copy (copiar una referencia es copiar la direccion, no el dato)

**Tipos que NO implementan Copy** (porque manejan datos en el heap):

- `String` (datos en el heap)
- `Vec<T>` (datos en el heap)
- `Box<T>` (dato en el heap)
- `HashMap<K, V>` (datos en el heap)
- Cualquier tipo que implementa `Drop` (liberacion de recursos personalizada)

La regla general es simple: si copiar el tipo es tan barato como copiar unos pocos bytes en el stack, entonces puede ser `Copy`. Si requiere duplicar datos en el heap o liberar recursos, no puede.

### Clone: copia profunda explicita

Que pasa si necesitas una copia real e independiente de un dato que vive en el heap? Para eso existe el trait **Clone**. A diferencia de `Copy`, que es automatico e implicito, `Clone` es explicito: tienes que llamar al metodo `.clone()`.

```rust
let s1 = String::from("hola");
let s2 = s1.clone();  // Copia profunda: duplica los datos del heap
println!("s1={s1}, s2={s2}"); // OK: son dos Strings independientes
```

```
Despues del Clone:

Stack                    Heap
┌──────────────┐
│ s1 ───────────────────> ┌──────────┐
│ ptr, len: 4  │          │ "hola"   │  (copia 1)
└──────────────┘          └──────────┘

┌──────────────┐
│ s2 ───────────────────> ┌──────────┐
│ ptr, len: 4  │          │ "hola"   │  (copia 2)
└──────────────┘          └──────────┘

Cada uno tiene su propia copia en el heap.
Cada uno llama drop() sobre su propia copia. Sin problemas.
```

**Clone tiene un costo.** Al llamar `.clone()` estas duplicando todos los datos del heap. Para un String corto no importa mucho, pero si clonas un `Vec` con un millon de elementos, estas copiando un millon de elementos. Usalo cuando realmente necesites dos copias independientes.

**Resumen de Copy vs Clone:**

| | Copy | Clone |
|---|---|---|
| Invocacion | Automatica e implicita | Explicita (`.clone()`) |
| Costo | Barato (pocos bytes en stack) | Potencialmente caro (duplica el heap) |
| Tipos | Solo tipos de tamanio fijo en stack | Casi cualquier tipo puede implementarlo |
| Resultado | Ambas variables independientes | Ambas variables independientes |
| Ejemplo | `let y = x;` (con i32) | `let s2 = s1.clone();` (con String) |

---

## Ownership y Functions

Pasar un valor a una funcion sigue las mismas reglas que la asignacion: se **mueve** o se **copia**.

```rust
fn main() {
    let s = String::from("hola");
    tomar_ownership(s);       // s se mueve a la funcion
    // println!("{s}");        // ERROR: s ya no es valido

    let n = 42;
    hacer_copia(n);           // n se copia (i32 implementa Copy)
    println!("{n}");          // OK: n sigue siendo valido
}

fn tomar_ownership(texto: String) {
    println!("{texto}");
}   // texto sale del scope y se libera (drop)

fn hacer_copia(numero: i32) {
    println!("{numero}");
}   // numero sale del scope, pero es i32 (Copy), no hay nada que liberar en heap
```

### El patron de devolver ownership

Que pasa si quieres pasar un String a una funcion pero seguir usandolo despues? Una opcion es que la funcion devuelva el ownership:

```rust
fn main() {
    let s1 = String::from("hola");
    let s2 = agregar_exclamacion(s1);  // s1 se mueve, s2 recibe el resultado
    // s1 ya no es valido, pero s2 si
    println!("{s2}");  // "hola!"
}

fn agregar_exclamacion(mut texto: String) -> String {
    texto.push('!');
    texto   // devolvemos el ownership al llamador
}
```

Funciona, pero imagina que necesitas pasar un dato a multiples funciones:

```rust
fn main() {
    let s = String::from("hola mundo");

    let (s, longitud) = calcular_longitud(s);  // mover y devolver
    let (s, mayusculas) = a_mayusculas(s);      // mover y devolver otra vez
    let (s, palabras) = contar_palabras(s);     // y otra vez...

    println!("{s}: len={longitud}, upper={mayusculas}, words={palabras}");
}

fn calcular_longitud(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)  // devolver el String junto con el resultado
}

fn a_mayusculas(s: String) -> (String, String) {
    let upper = s.to_uppercase();
    (s, upper)
}

fn contar_palabras(s: String) -> (String, usize) {
    let count = s.split_whitespace().count();
    (s, count)
}
```

Esto es tedioso, verboso y propenso a errores. Cada funcion tiene que recibir el String, hacer su trabajo, y devolver el String junto con su resultado real. Nadie quiere programar asi.

**Por eso existen las references.** En lugar de mover el ownership de ida y vuelta, puedes *prestar* el valor a la funcion. La funcion lo usa temporalmente y el owner original lo conserva. Esto es lo que veremos en la siguiente seccion.

---

## References y Borrowing

### Que es una referencia

Una referencia es, a nivel tecnico, un puntero: una direccion de memoria que apunta a un dato. Pero a diferencia de los punteros en C/C++, las referencias en Rust tienen una garantia fundamental: **siempre son validas**. El compilador se asegura, en compile time, de que una referencia nunca apunte a memoria liberada, no inicializada o invalida.

Creas una referencia con el operador `&`:

```rust
let s = String::from("hola");
let r = &s;  // r es una referencia a s

println!("{r}");  // usa r para acceder al valor de s
println!("{s}");  // s sigue siendo valido (sigue siendo el owner)
```

```
Stack                    Heap
┌──────────────┐
│ s ──────────────────> ┌──────────┐
│ ptr, len: 4  │        │ "hola"   │
└──────────────┘        └──────────┘
       ^
       │
┌──────────────┐
│ r ───────────┘
│ (referencia) │
└──────────────┘

r apunta a s, que a su vez apunta al dato en el heap.
r puede leer el dato, pero s sigue siendo el owner.
Cuando r salga del scope, no pasa nada (no es owner).
Cuando s salga del scope, se llama drop() y se libera el heap.
```

### Por que se llama borrowing

En Rust, el acto de crear una referencia se llama **borrowing** (tomar prestado). La analogia es directa:

- Tienes un libro (un valor). Eres el duenio (owner).
- Le prestas el libro a un amigo (creas una referencia).
- Tu amigo puede leerlo (acceder al valor a traves de la referencia).
- Tu amigo no puede destruirlo ni regalarselo a otra persona (no es el owner).
- Cuando tu amigo termina, te lo devuelve (la referencia sale del scope).
- Tu sigues siendo el duenio todo el tiempo.

```rust
fn main() {
    let s = String::from("hola");
    let longitud = calcular_longitud(&s);  // prestamos s, no la movemos
    println!("'{s}' tiene {longitud} caracteres"); // s sigue siendo nuestro
}

fn calcular_longitud(s: &String) -> usize {
    s.len()
}   // s es solo una referencia prestada. No se libera nada.
```

### Immutable References (&T)

Por defecto, una referencia es inmutable: puedes leer el dato pero no modificarlo.

```rust
fn main() {
    let s = String::from("hola");
    imprimir(&s);       // prestamo inmutable
    imprimir(&s);       // podemos prestar multiples veces
    println!("{s}");    // s sigue intacto
}

fn imprimir(texto: &String) {
    println!("{texto}");
    // texto.push_str("!");  // ERROR: no puedes modificar una referencia inmutable
}
```

Puedes tener **multiples referencias inmutables** al mismo tiempo. Esto es seguro porque nadie esta modificando el dato:

```rust
let s = String::from("hola");

let r1 = &s;
let r2 = &s;
let r3 = &s;
println!("{r1} {r2} {r3}");  // OK: tres lectores simultaneos, ningun escritor
```

### Mutable References (&mut T)

Si necesitas modificar un valor prestado, usas una referencia mutable con `&mut`:

```rust
fn main() {
    let mut s = String::from("hola");  // la variable debe ser 'mut'
    agregar_texto(&mut s);              // prestamo mutable
    println!("{s}"); // "hola, mundo"
}

fn agregar_texto(s: &mut String) {
    s.push_str(", mundo");  // podemos modificar porque es &mut
}
```

Nota que la variable original debe estar declarada como `mut`. Tiene logica: no puedes prestar algo como mutable si el duenio mismo no permitio que se modifique.

### Reglas de References (y por que existen)

Rust impone reglas estrictas sobre las references. Estas reglas no son arbitrarias; cada una previene una categoria concreta de bugs.

**Regla 1: Puedes tener multiples `&T` (inmutables) al mismo tiempo.**

Esto es seguro: si nadie modifica el dato, cualquier cantidad de lectores pueden acceder sin problemas.

```rust
let s = String::from("hola");
let r1 = &s;
let r2 = &s;
println!("{r1} {r2}");  // OK: solo lectura, sin conflictos
```

**Regla 2: Solo puedes tener una `&mut T` (mutable) a la vez.**

Por que? Porque tener dos referencias mutables al mismo dato abre la puerta a **data races**. Un data race ocurre cuando dos accesos al mismo dato suceden simultaneamente (o de forma intercalada) y al menos uno es una escritura. El resultado es impredecible.

```rust
let mut s = String::from("hola");

let r1 = &mut s;
// let r2 = &mut s;  // ERROR: cannot borrow `s` as mutable more than once at a time

r1.push_str(" mundo");
println!("{r1}");
```

Imagina si Rust permitiera dos `&mut` al mismo dato: una referencia podria estar insertando elementos en un `Vec` (lo que puede causar que el Vec reubique su memoria) mientras la otra referencia sigue leyendo la posicion antigua. El resultado seria leer memoria invalida.

**Regla 3: No puedes mezclar `&T` y `&mut T` al mismo tiempo.**

Por que? Porque si alguien tiene una referencia inmutable, asume que el dato no va a cambiar mientras la use. Si permites una referencia mutable al mismo tiempo, esa suposicion se rompe.

```rust
let mut s = String::from("hola");

let r1 = &s;     // referencia inmutable
let r2 = &s;     // otra referencia inmutable - OK
// let r3 = &mut s; // ERROR: no puedes tener mutable mientras existan inmutables

println!("{r1}, {r2}");
// r1 y r2 ya no se usan despues de aqui (sus "lifetimes" terminan)

let r3 = &mut s;  // OK ahora: r1 y r2 ya no estan activos
r3.push_str(" mundo");
println!("{r3}");
```

Nota algo importante: Rust no mira los scopes de las llaves sino el **ultimo uso** de cada referencia. Este comportamiento se llama Non-Lexical Lifetimes (NLL). En el ejemplo anterior, r1 y r2 "mueren" despues de `println!("{r1}, {r2}")`, no al final del bloque. Por eso r3 es valido.

### Dangling References (y como Rust las previene)

Una **dangling reference** (referencia colgante) es una referencia que apunta a memoria que ya fue liberada. En C/C++, este es un bug clasico y peligroso. En Rust, el compilador lo hace **imposible**.

Intenta este codigo:

```rust
fn crear_dangling() -> &String {   // ERROR de compilacion
    let s = String::from("hola");
    &s  // intentamos retornar una referencia a s
}   // pero s se destruye aqui! la referencia apuntaria a nada
```

El compilador rechaza este codigo con un error claro: `s` se crea dentro de la funcion, se destruye cuando la funcion termina, y la referencia `&s` quedaria apuntando a memoria liberada. Rust detecta esto en compile time y no te deja compilar.

La solucion correcta es devolver el valor directamente (transferir ownership):

```rust
fn crear_string() -> String {
    let s = String::from("hola");
    s  // movemos el ownership al llamador, sin referencia colgante
}
```

---

## Slices

### Que problema resuelven los slices

Imagina que tienes un String largo y quieres trabajar con solo una parte de el. Tienes dos opciones: copiar esa porcion a un nuevo String (caro: duplica memoria) o, de alguna manera, "apuntar" a esa porcion dentro del original (barato: solo una referencia). Los **slices** son esa segunda opcion.

Un slice es una referencia a una **porcion contigua** de una coleccion. No posee los datos (no es owner), solo los referencia. Es una vista sobre datos existentes.

```rust
let s = String::from("hola mundo");

let hola = &s[0..4];    // "hola" - un slice de los primeros 4 bytes
let mundo = &s[5..10];  // "mundo" - un slice de los ultimos 5 bytes
let todo = &s[..];      // "hola mundo" - un slice de todo el String
```

```
Stack                    Heap
┌──────────────┐
│ s ──────────────────> ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
│ ptr, len: 10 │        │ h │ o │ l │ a │   │ m │ u │ n │ d │ o │
│ cap: 10      │        └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
└──────────────┘          ^               ^
                          │               │
┌──────────────┐          │   ┌──────────────┐
│ hola ────────────────────   │ mundo ───────┘
│ ptr, len: 4  │              │ ptr, len: 5  │
└──────────────┘              └──────────────┘

Los slices apuntan directamente dentro de la memoria del String original.
No copian nada. Son solo un puntero + una longitud.
```

### String vs &str: owner vs vista

Esta es una distincion fundamental en Rust y fuente comun de confusion al inicio:

**`String`** es un tipo que **posee** (es owner de) una cadena de texto almacenada en el heap. Puede crecer, encogerse y modificarse. Es como un `Vec<u8>` con la garantia de que contiene UTF-8 valido.

**`&str`** (string slice) es una **referencia** a una secuencia de bytes UTF-8. No posee los datos. Es una vista inmutable sobre un String, un literal de texto o cualquier fuente de bytes UTF-8.

```rust
// String: owner de los datos en el heap. Puede modificarse.
let mut owned = String::from("hola");
owned.push_str(" mundo");  // OK: es owner, puede mutar

// &str: referencia a datos existentes. No puede modificarse.
let slice: &str = &owned[0..4];  // vista sobre los primeros 4 bytes del String
// slice.push_str("!");          // ERROR: &str no es mutable ni es owner

// Los literales de texto son &str (apuntan a datos embebidos en el binario)
let literal: &str = "hola";  // estos bytes estan en el ejecutable, no en el heap
```

**Cuando usar cada uno:**

- Usa `String` cuando necesites poseer el texto: almacenarlo en un struct, modificarlo, construirlo dinamicamente.
- Usa `&str` cuando solo necesites leer el texto: parametros de funciones, comparaciones, busquedas.

### Slices de arrays

Los slices no son exclusivos de strings. Puedes tener slices de cualquier coleccion contigua, como arrays y Vec:

```rust
let numeros = [1, 2, 3, 4, 5];       // array: [i32; 5] (tamanio fijo, stack)
let slice: &[i32] = &numeros[1..4];  // slice: &[i32] -> [2, 3, 4]

println!("{:?}", slice);  // [2, 3, 4]

let vector = vec![10, 20, 30, 40];     // Vec<i32> (tamanio dinamico, heap)
let slice2: &[i32] = &vector[..2];     // slice: &[i32] -> [10, 20]

println!("{:?}", slice2);  // [10, 20]
```

La relacion es analoga a String y &str:

| Owner (posee los datos) | Slice (vista/referencia) |
|---|---|
| `String` | `&str` |
| `Vec<T>` | `&[T]` |
| `[T; N]` (array) | `&[T]` |

### Por que las funciones deberian aceptar &str en vez de &String

Este es un patron importante en Rust idiomatico. Compara estas dos firmas:

```rust
// Menos flexible: solo acepta referencias a String
fn contar_vocales_v1(texto: &String) -> usize {
    texto.chars().filter(|c| "aeiouAEIOU".contains(*c)).count()
}

// Mas flexible: acepta &String, &str, y slices de String
fn contar_vocales_v2(texto: &str) -> usize {
    texto.chars().filter(|c| "aeiouAEIOU".contains(*c)).count()
}
```

La version con `&str` es mas flexible porque Rust puede convertir automaticamente un `&String` a un `&str` (gracias a un mecanismo llamado deref coercion). Esto significa que la funcion acepta ambos:

```rust
let owned = String::from("hola mundo");
let literal = "hola mundo";

// La version con &str acepta todo:
contar_vocales_v2(&owned);      // &String se convierte automaticamente a &str
contar_vocales_v2(literal);     // &str directamente
contar_vocales_v2(&owned[0..4]); // un slice de String tambien es &str

// La version con &String solo acepta &String:
contar_vocales_v1(&owned);      // OK
// contar_vocales_v1(literal);  // ERROR: esperaba &String, recibio &str
```

La regla general: **si tu funcion solo necesita leer un texto, acepta `&str`**. Esto la hace compatible con la mayor cantidad de tipos posibles.

### Ejemplo practico: encontrar la primera palabra

```rust
fn primera_palabra(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s  // si no hay espacio, todo el texto es una sola palabra
}

fn main() {
    let texto = String::from("hola mundo cruel");

    let palabra = primera_palabra(&texto);
    println!("Primera palabra: {palabra}");  // "hola"

    // El slice 'palabra' es una referencia dentro de 'texto'.
    // Si intentamos modificar 'texto' mientras 'palabra' existe:
    // texto.clear();  // ERROR: no puedes mutar 'texto' mientras 'palabra' lo referencia
    // Esto es ownership en accion: el compilador previene bugs.

    println!("Texto completo: {texto}");
}
```

Este es uno de los ejemplos mas poderosos de como ownership previene bugs en la practica. En C, podrias obtener un puntero a la primera palabra, luego modificar o liberar el string original, y el puntero quedaria dangling. En Rust, el compilador lo impide.

---

## Ejecutar Este Ejemplo

```bash
cd 03_ownership_borrowing
cargo run
```

---

## Ejercicios

1. Crea una funcion que reciba un `&str` y retorne `true` si contiene la palabra "rust"
2. Escribe una funcion que tome ownership de un `String`, lo modifique, y lo retorne
3. Experimenta: intenta crear dos `&mut` references al mismo valor y observa el error
4. Escribe una funcion que reciba un `&[i32]` (slice de enteros) y retorne la suma de todos los elementos
5. Intenta escribir una funcion que retorne una referencia a un String creado dentro de la funcion. Observa el error del compilador y corrigelo retornando el String directamente.
