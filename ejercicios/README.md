# Ejercicios de Rust - Capitulos 2 al 9

Cada capitulo tiene **3 ejercicios** en archivos separados, cada uno con sus propios tests.

---

## Estructura

```
ejercicios/
├── cap_02/src/           # Variables, Tipos y Funciones
│   ├── ej1_celsius.rs        -> Convertir Celsius a Fahrenheit
│   ├── ej2_fibonacci.rs      -> Calcular n-esimo Fibonacci
│   └── ej3_shadowing.rs      -> Shadowing con .len() y multiplicacion
│
├── cap_03/src/           # Ownership y Borrowing
│   ├── ej1_contiene_rust.rs  -> Buscar "rust" con &String (borrowing)
│   ├── ej2_tomar_ownership.rs -> Tomar ownership y modificar String
│   └── ej3_encontrar_mayor.rs -> Mayor valor en &[i32] (slices)
│
├── cap_04/src/           # Structs y Enums
│   ├── ej1_rectangulo.rs     -> Struct con area() y es_cuadrado()
│   ├── ej2_moneda.rs         -> Enum Moneda con conversion a pesos
│   └── ej3_estudiante.rs     -> Struct con Vec<f64>, promedio y estado
│
├── cap_05/src/           # Control de Flujo y Pattern Matching
│   ├── ej1_etapa_de_vida.rs  -> match con ranges (edad -> etapa)
│   ├── ej2_fizzbuzz.rs       -> match con tuples (FizzBuzz)
│   └── ej3_contar_signos.rs  -> match con Ordering (positivos/negativos/ceros)
│
├── cap_06/src/           # Manejo de Errores
│   ├── ej1_dividir.rs        -> Result con custom error (DivisionPorCero)
│   ├── ej2_parsear.rs        -> Operador ? para propagar errores
│   └── ej3_validar_usuario.rs -> Multiples paths de error con early return
│
├── cap_07/src/           # Collections
│   ├── ej1_filtrar_pares.rs  -> Vec: filter, collect, sort
│   ├── ej2_contar_palabras.rs -> HashMap: entry API, frecuencias
│   └── ej3_mas_largo.rs      -> Slices de String, Option
│
├── cap_08/src/           # Generics, Traits y Lifetimes
│   ├── ej1_mayor_generico.rs -> Funcion generica con trait bounds
│   ├── ej2_describible.rs    -> Definir e implementar un trait
│   └── ej3_pila.rs           -> Struct generico Pila<T> con Display
│
└── cap_09/src/           # Modulos y Cargo
    ├── ej1_matematica.rs     -> Sub-modulos y re-exports (pub use)
    ├── ej2_convertidor.rs    -> Visibilidad: constantes privadas, fn publicas
    └── ej3_validador.rs      -> Comunicacion entre modulos con use/crate
```

---

## Como ejecutar los tests

Desde el directorio `ejercicios/`:

```bash
cd ejercicios

# Correr TODOS los tests (todos los capitulos)
cargo test

# Correr tests de un capitulo especifico
cargo test -p cap_02
cargo test -p cap_05

# Correr tests de un ejercicio especifico
cargo test -p cap_02 ej1_celsius
cargo test -p cap_06 ej3_validar_usuario

# Ver output detallado (nombre de cada test)
cargo test -- --show-output
```

---

## Que esperar como output

Cuando corras `cargo test`, deberias ver algo asi:

```
running 4 tests
test ej1_celsius::tests::test_agua_congela ... ok
test ej1_celsius::tests::test_agua_hierve ... ok
test ej1_celsius::tests::test_negativo ... ok
test ej1_celsius::tests::test_temperatura_corporal ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Si **todos los tests pasan**, veras `ok` en verde junto a cada test y un resumen al final con `0 failed`.

Si un test **falla**, veras el detalle del error con el valor esperado vs el valor que obtuviste:

```
test ej1_celsius::tests::test_agua_congela ... FAILED

failures:
---- ej1_celsius::tests::test_agua_congela stdout ----
assertion `left == right` failed
  left: 30.0
 right: 32.0

test result: FAILED. 3 passed; 1 failed; 0 ignored
```

---

## Que debe hacer cada ejercicio

Cada archivo `.rs` tiene al inicio un comentario que explica:
- Que debe hacer la funcion
- Que conceptos de Rust practica
- Los tests al final del archivo verifican que la implementacion sea correcta

Lee el comentario del inicio, implementa la funcion, y corre los tests para verificar.
