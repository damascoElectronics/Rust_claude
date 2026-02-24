# Capitulo 1: Hola Mundo y Cargo

> Referencia: *The Rust Programming Language* - Cap. 1: Getting Started

---

## Tu Primer Programa en Rust

El clasico "Hello, World!" en Rust:

```rust
fn main() {
    println!("Hola, mundo!");
}
```

Este programa de tres lineas contiene varios conceptos fundamentales del lenguaje. Vamos a
desglosar cada elemento para entender exactamente que esta pasando.

### Conceptos Clave, Linea por Linea

- `fn` -- Es la keyword (palabra reservada) que le dice al compilador de Rust que estas
  declarando una **funcion**. En Rust, todo bloque de codigo ejecutable vive dentro de una
  funcion. Cada vez que quieras definir un comportamiento reutilizable, empezaras con `fn`
  seguido del nombre de la funcion.

- `main()` -- Es la **entry point** (punto de entrada) de todo programa en Rust. Cuando
  ejecutas un programa compilado, el sistema operativo busca esta funcion y comienza a
  ejecutar el codigo que hay dentro de ella. Si tu programa no tiene una funcion `main`, el
  compilador producira un error y no podras generar un ejecutable. No importa cuantas
  funciones tenga tu programa: la ejecucion siempre comienza en `main`.

- `println!` -- Es un **macro**, no una funcion comun. El signo `!` al final es la pista:
  en Rust, cualquier llamada que termine con `!` es una invocacion a un macro. Un macro es
  un mecanismo que genera codigo en tiempo de compilacion. La diferencia practica con una
  funcion normal es que los macros pueden aceptar un numero variable de argumentos y producir
  codigo mas complejo del que escribiste. Por ejemplo, `println!` puede recibir texto con
  marcadores de formato como `println!("Tengo {} anios", 25)`, y el macro se encarga de
  generar todo el codigo necesario para formatear e imprimir esa cadena. Si `println` fuera
  una funcion normal (sin el `!`), no podria ofrecer esa flexibilidad. Por ahora, lo
  importante es recordar: si tiene `!`, es un macro.

- Las llaves `{}` delimitan el **body** (cuerpo) de la funcion. Todo el codigo que
  pertenece a `main` va dentro de estas llaves. Rust usa llaves para definir bloques de
  codigo en funciones, condicionales, bucles y otras estructuras.

- El `;` (semicolon) marca el **final de una sentencia**. En Rust, la mayoria de las lineas
  de codigo son sentencias, y cada sentencia debe terminar con punto y coma. Si olvidas
  el `;`, el compilador te dara un error. Esto no es arbitrario: Rust distingue entre
  **sentencias** (que ejecutan algo pero no devuelven un valor) y **expresiones** (que
  producen un valor). El `;` convierte una expresion en una sentencia. Esta distincion sera
  muy importante cuando aprendas sobre funciones que devuelven valores, pero por ahora, la
  regla practica es: termina cada linea con `;`.

### Que Pasa Cuando Ejecutas Este Programa

Cuando escribes `cargo run`, ocurren dos cosas en secuencia:

1. **Compilacion**: El compilador de Rust (`rustc`) lee tu codigo fuente (`main.rs`), lo
   analiza, verifica que no tenga errores y lo traduce a **codigo maquina nativo**. El
   resultado es un archivo binario ejecutable que tu sistema operativo puede correr
   directamente, sin necesidad de ningun interprete o maquina virtual.

2. **Ejecucion**: Cargo ejecuta automaticamente ese binario. El sistema operativo carga el
   programa en memoria, busca la funcion `main`, y comienza a ejecutar las instrucciones.
   En este caso, la unica instruccion es imprimir "Hola, mundo!" en la terminal.

El archivo binario resultante es un programa independiente. Puedes copiarlo a otra maquina
con el mismo sistema operativo y ejecutarlo sin tener Rust instalado.

---

## Cargo: El Build System y Package Manager

### Que Problema Resuelve Cargo

Sin Cargo, trabajar con Rust seria considerablemente mas tedioso. Tendrias que:

- Invocar el compilador `rustc` manualmente cada vez que quieras compilar, pasandole
  todos los archivos fuente y las flags necesarias.
- Descargar las dependencias (bibliotecas externas) a mano, colocarlas en el directorio
  correcto y decirle al compilador donde encontrarlas.
- Gestionar las versiones de esas dependencias tu mismo, asegurandote de que sean
  compatibles entre si.
- Escribir scripts para automatizar las pruebas, la compilacion en modo optimizado y
  otras tareas repetitivas.

Cargo resuelve todo esto con una sola herramienta. Combina tres roles:

- **Build system** -- Compila tu codigo y el de todas tus dependencias, en el orden
  correcto, pasando las flags adecuadas al compilador.
- **Package manager** -- Descarga, actualiza y gestiona las dependencias de tu proyecto.
  Las bibliotecas externas en el ecosistema de Rust se llaman **crates**, y se publican
  en [crates.io](https://crates.io), el registro central.
- **Test runner** -- Ejecuta las pruebas unitarias e integrales de tu proyecto con un
  solo comando.

En la practica, casi nunca vas a invocar `rustc` directamente. Cargo es la forma estandar
de trabajar con Rust.

### Comandos Esenciales de Cargo

```bash
cargo new nombre_proyecto
```

Crea un nuevo proyecto de Rust desde cero. Genera un directorio con el nombre que le
indiques, dentro del cual coloca un archivo `Cargo.toml` preconfigurado, un directorio
`src/` con un `main.rs` que contiene un "Hello, World!" basico, e inicializa un repositorio
Git. Es el punto de partida de cualquier proyecto nuevo. Usa este comando cada vez que
empieces un programa o biblioteca desde cero.

```bash
cargo build
```

Compila tu proyecto y todas sus dependencias. El binario resultante se guarda en
`target/debug/` (por defecto se compila en modo debug). Este comando es util cuando
quieres compilar sin ejecutar, por ejemplo para verificar que todo compila correctamente
o para generar el binario y distribuirlo por separado. La primera vez que lo ejecutas en un
proyecto, Cargo tambien descarga y compila todas las dependencias declaradas en `Cargo.toml`.

```bash
cargo run
```

Compila **y** ejecuta tu proyecto en un solo paso. Es equivalente a hacer `cargo build`
seguido de ejecutar el binario manualmente. Este es el comando que mas usaras durante el
desarrollo, porque te permite ver el resultado de tus cambios inmediatamente. Si el codigo
no ha cambiado desde la ultima compilacion, Cargo es lo suficientemente inteligente para
saltarse la compilacion y ejecutar directamente el binario existente.

```bash
cargo check
```

Verifica que tu codigo compila correctamente **sin generar un binario**. Este es un detalle
importante: `cargo check` ejecuta el analisis del compilador (parsing, type checking, borrow
checking) pero se detiene antes de la fase de generacion de codigo. Como resultado, es
significativamente mas rapido que `cargo build`. En proyectos grandes, la diferencia puede
ser de segundos a minutos. Usa `cargo check` cuando estes escribiendo codigo y solo quieras
saber si compila. Usa `cargo build` cuando necesites el binario.

```bash
cargo build --release
```

Compila tu proyecto en **modo release** (optimizado). La diferencia con `cargo build` es
la siguiente:

- **Modo debug** (el default): El compilador genera el binario rapido, pero el programa
  resultante es mas lento. Incluye informacion de depuracion y no aplica optimizaciones
  agresivas. Es ideal para desarrollo, porque lo que te importa es compilar rapido e
  iterar.
- **Modo release**: El compilador tarda mas en generar el binario, pero el programa
  resultante es mucho mas rapido. Aplica todas las optimizaciones disponibles y elimina
  la informacion de depuracion. El binario se guarda en `target/release/` en lugar de
  `target/debug/`. Usa este modo cuando vayas a distribuir tu programa a usuarios,
  cuando hagas benchmarks de rendimiento, o cuando despliegues a produccion.

---

## Estructura de un Proyecto Cargo

```
mi_proyecto/
├── Cargo.toml    # Archivo de configuracion (manifest)
├── Cargo.lock    # Versiones exactas de dependencias (auto-generado)
├── src/
│   └── main.rs   # Codigo fuente principal
└── target/       # Directorio de compilacion (auto-generado)
```

### Por Que Esta Estructura

Cargo impone una convencion sobre la organizacion del proyecto para que todos los proyectos
de Rust se vean iguales. Esto tiene una ventaja practica: cuando abres cualquier proyecto
Rust ajeno, ya sabes donde buscar cada cosa.

- **`src/main.rs`** -- Es el punto de entrada de un programa binario (executable). Cargo
  busca este archivo por convencion. Si existe `src/main.rs`, Cargo sabe que el proyecto
  produce un binario ejecutable y que la compilacion debe empezar por ahi. No necesitas
  configurar nada para indicarselo. Si en cambio quisieras crear una **biblioteca** (una
  crate que otros programas importan), el archivo de entrada seria `src/lib.rs`. Un
  proyecto puede tener ambos.

- **`Cargo.lock`** -- Este archivo lo genera y mantiene Cargo automaticamente. Contiene las
  versiones **exactas** de cada dependencia que se uso en la ultima compilacion exitosa,
  incluyendo las dependencias transitivas (es decir, las dependencias de tus dependencias).
  Su proposito es garantizar **compilaciones reproducibles**: si tu companiiero de equipo
  clona el repositorio y ejecuta `cargo build`, obtendra exactamente las mismas versiones
  de todas las bibliotecas, no versiones mas nuevas que podrian introducir incompatibilidades.
  Nunca deberias editar este archivo a mano. Cargo lo actualiza cuando agregas, eliminas o
  actualizas dependencias.

- **`target/`** -- Es el directorio donde Cargo coloca todos los artefactos de compilacion:
  binarios, archivos intermedios, dependencias compiladas, etc. Esta en el `.gitignore` por
  defecto porque su contenido se puede regenerar ejecutando `cargo build`. Puede crecer
  bastante en proyectos con muchas dependencias.

---

## El Archivo `Cargo.toml`

`Cargo.toml` es el **manifest** de tu proyecto. Un manifest es un archivo de configuracion
que le dice a Cargo todo lo que necesita saber sobre tu proyecto: como se llama, que version
tiene, que edicion del lenguaje usa y de que bibliotecas externas depende. Usa el formato
[TOML](https://toml.io) (Tom's Obvious, Minimal Language), que es un formato de
configuracion disenado para ser facil de leer.

```toml
[package]
name = "hola_mundo"
version = "0.1.0"
edition = "2021"

[dependencies]
```

### Que Significa Cada Campo

- **`name`** -- El nombre de tu proyecto. Cargo lo usa para nombrar el binario resultante
  de la compilacion. Si tu proyecto se llama `"hola_mundo"`, el ejecutable sera
  `target/debug/hola_mundo` (o `hola_mundo.exe` en Windows). Si publicas tu crate en
  crates.io, este sera el nombre con el que otros lo importen.

- **`version`** -- La version de tu proyecto, siguiendo el esquema de
  [Semantic Versioning](https://semver.org/) (MAJOR.MINOR.PATCH). `"0.1.0"` es la version
  inicial que Cargo asigna por defecto. Cuando hagas cambios en tu proyecto, deberias
  incrementar este numero siguiendo las convenciones: incrementa PATCH para correcciones,
  MINOR para nueva funcionalidad compatible y MAJOR para cambios que rompen compatibilidad.

- **`edition`** -- La **edicion de Rust** que tu proyecto usa. Rust introduce ediciones
  cada tres anios (2015, 2018, 2021, 2024) que pueden incluir cambios en la sintaxis del
  lenguaje. Lo importante es que las ediciones no son versiones del compilador: puedes usar
  el compilador mas reciente y seguir compilando con `edition = "2021"`. Las ediciones
  permiten que Rust evolucione sin romper el codigo existente. Los proyectos nuevos deben
  usar la edicion mas reciente disponible.

- **`[dependencies]`** -- Esta seccion lista las bibliotecas externas (crates) de las que
  depende tu proyecto. Un proyecto nuevo no tiene dependencias, asi que esta seccion esta
  vacia. Para agregar una dependencia, escribes su nombre y la version deseada.

### Como Agregar una Dependencia

Por ejemplo, si quieres usar la crate `rand` para generar numeros aleatorios, agregas la
siguiente linea en la seccion `[dependencies]`:

```toml
[dependencies]
rand = "0.8"
```

La proxima vez que ejecutes `cargo build` o `cargo run`, Cargo descargara automaticamente
la crate `rand` (version 0.8.x) junto con todas sus dependencias transitivas, las compilara
y las enlazara con tu programa. Tambien actualizara el archivo `Cargo.lock` con las
versiones exactas descargadas.

---

## Compilacion: Por Que Rust Compila a Codigo Nativo

### Que Es un Compilador

Un compilador es un programa que traduce codigo fuente (el texto que tu escribes) a un
formato que la maquina puede ejecutar. En el caso de Rust, el compilador (`rustc`) traduce
tu codigo directamente a **codigo maquina nativo**: las instrucciones binarias que el
procesador de tu computadora entiende. El resultado es un archivo ejecutable independiente.

Esto contrasta con los **lenguajes interpretados** como Python o JavaScript, donde un
programa intermediario (el interprete) lee tu codigo y lo ejecuta linea por linea en tiempo
de ejecucion. Los lenguajes compilados como Rust tienden a producir programas mas rapidos,
porque la traduccion a codigo maquina ya se hizo de antemano y el procesador ejecuta las
instrucciones directamente, sin intermediarios.

### Errores en Tiempo de Compilacion

Una de las mayores ventajas de Rust es que una cantidad enorme de errores se detectan **en
tiempo de compilacion**, antes de que el programa se ejecute. Esto incluye:

- Errores de tipos (intentar sumar un numero con un texto).
- Errores de memoria (usar una referencia que ya no es valida).
- Errores de concurrencia (acceder a datos compartidos de forma insegura).

En lenguajes interpretados, muchos de estos errores solo aparecen cuando el programa esta
corriendo, posiblemente en produccion. En Rust, si tu programa compila, ya tienes una
garantia solida de que toda una categoria de bugs no esta presente. Los mensajes de error
del compilador de Rust son intencionalmente detallados y explicativos: te dicen que esta
mal, donde esta el problema, y frecuentemente sugieren como corregirlo.

---

## Compilacion sin Cargo

Tambien puedes compilar directamente con `rustc`:

```bash
rustc main.rs    # Compila el archivo
./main           # Ejecuta el binario resultante
```

Esto funciona para programas de un solo archivo sin dependencias externas. Pero en cuanto
tu proyecto crezca o necesite una biblioteca externa, gestionar la compilacion manualmente
se vuelve impractico. Por eso, para proyectos reales, **siempre usa Cargo**.

---

## Ejecutar Este Ejemplo

```bash
cd 01_hola_mundo
cargo run
```

---

## Ejercicios

1. Modifica el mensaje para que imprima tu nombre
2. Crea un nuevo proyecto con `cargo new mi_proyecto` y explora la estructura
3. Usa `cargo check` vs `cargo build` y observa la diferencia en velocidad
