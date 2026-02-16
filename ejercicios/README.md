# Ejercicios de Rust - Capitulos 2 al 9

Cada capitulo tiene entre **5 y 7 ejercicios** en archivos separados, cada uno con sus propios tests. En total hay **216 tests** que cubren todos los temas del curso.

---

## Estructura

```
ejercicios/
├── cap_02/src/           # Variables, Tipos y Funciones
│   ├── ej1_celsius.rs        -> Convertir Celsius a Fahrenheit
│   ├── ej2_fibonacci.rs      -> Calcular n-esimo Fibonacci
│   ├── ej3_shadowing.rs      -> Shadowing con .len() y multiplicacion
│   ├── ej4_tipos_escalares.rs -> i32, f64, bool, char, casting
│   ├── ej5_tuplas_arrays.rs  -> Destructuring, arrays [valor; n], indices
│   └── ej6_constantes_bloques.rs -> const, block expressions, statements vs expressions
│
├── cap_03/src/           # Ownership y Borrowing
│   ├── ej1_contiene_rust.rs  -> Buscar "rust" con &String (borrowing)
│   ├── ej2_tomar_ownership.rs -> Tomar ownership y modificar String
│   ├── ej3_encontrar_mayor.rs -> Mayor valor en &[i32] (slices)
│   ├── ej4_copy_vs_move.rs   -> Copy (stack) vs Move (heap), stack vs heap
│   ├── ej5_clone.rs          -> clone() para copias profundas
│   ├── ej6_referencias_mutables.rs -> &mut T, reglas de borrowing, modificar in-place
│   └── ej7_string_slices.rs  -> &str, string slicing, slices de arrays
│
├── cap_04/src/           # Structs y Enums
│   ├── ej1_rectangulo.rs     -> Struct con area() y es_cuadrado()
│   ├── ej2_moneda.rs         -> Enum Moneda con conversion a pesos
│   ├── ej3_estudiante.rs     -> Struct con Vec<f64>, promedio y estado
│   ├── ej4_tuple_structs.rs  -> Tuple structs (Color), struct update syntax (..)
│   ├── ej5_mut_self.rs       -> &self vs &mut self vs self (consume)
│   └── ej6_enum_con_datos.rs -> Enums con datos asociados, destructuring
│
├── cap_05/src/           # Control de Flujo y Pattern Matching
│   ├── ej1_etapa_de_vida.rs  -> match con ranges (edad -> etapa)
│   ├── ej2_fizzbuzz.rs       -> match con tuples (FizzBuzz)
│   ├── ej3_contar_signos.rs  -> match con Ordering (positivos/negativos/ceros)
│   ├── ej4_if_expresion.rs   -> if como expresion (asignar resultado a variable)
│   ├── ej5_loops.rs          -> loop con break value, while, loop labels
│   ├── ej6_for_iteradores.rs -> for con ranges, enumerate(), rev(), colecciones
│   └── ej7_if_let_while_let.rs -> if let, while let, Option matching
│
├── cap_06/src/           # Manejo de Errores
│   ├── ej1_dividir.rs        -> Result con custom error (DivisionPorCero)
│   ├── ej2_parsear.rs        -> Operador ? para propagar errores
│   ├── ej3_validar_usuario.rs -> Multiples paths de error con early return
│   ├── ej4_unwrap_expect.rs  -> unwrap_or, map, and_then, is_ok/is_err
│   └── ej5_propagacion_errores.rs -> Cadenas de ?, From trait, AppError custom
│
├── cap_07/src/           # Collections
│   ├── ej1_filtrar_pares.rs  -> Vec: filter, collect, sort
│   ├── ej2_contar_palabras.rs -> HashMap: entry API, frecuencias
│   ├── ej3_mas_largo.rs      -> Slices de String, Option
│   ├── ej4_vec_operaciones.rs -> push/pop, get() vs [], iter_mut, deref *
│   ├── ej5_string_metodos.rs -> push/push_str, format!, chars(), invertir
│   └── ej6_hashmap_avanzado.rs -> get(), iteracion, agrupacion, zip
│
├── cap_08/src/           # Generics, Traits y Lifetimes
│   ├── ej1_mayor_generico.rs -> Funcion generica con trait bounds
│   ├── ej2_describible.rs    -> Definir e implementar un trait
│   ├── ej3_pila.rs           -> Struct generico Pila<T> con Display
│   ├── ej4_generics_multiples.rs -> Par<T,U>, where clause, impl especializado
│   ├── ej5_traits_estandar.rs -> Debug, Clone, Copy, PartialEq, Default, Display
│   └── ej6_lifetimes.rs      -> 'a en funciones, structs con &, multiples lifetimes
│
└── cap_09/src/           # Modulos y Cargo
    ├── ej1_matematica.rs     -> Sub-modulos y re-exports (pub use)
    ├── ej2_convertidor.rs    -> Visibilidad: constantes privadas, fn publicas
    ├── ej3_validador.rs      -> Comunicacion entre modulos con use/crate
    ├── ej4_visibilidad.rs    -> pub vs privado, pub(super), campos privados
    └── ej5_paths_imports.rs  -> Paths absolutos/relativos, use as, nested imports
```

---

## Temas que cubre cada capitulo

| Cap | Ejercicios | Temas |
|-----|-----------|-------|
| 02 | 6 | Variables, mutabilidad, shadowing, tipos escalares, tuplas, arrays, constantes, block expressions, funciones |
| 03 | 7 | Ownership, stack vs heap, Copy vs Move, clone, borrowing (&T), referencias mutables (&mut T), string slices |
| 04 | 6 | Structs, methods (&self, &mut self, self), tuple structs, update syntax, enums, enums con datos, Option |
| 05 | 7 | if expressions, match con ranges/tuples/Ordering, loop/while/for, loop labels, enumerate, rev, if let, while let |
| 06 | 5 | Result, custom errors, operador ?, unwrap/expect/unwrap_or, map/and_then, From trait, propagacion encadenada |
| 07 | 6 | Vec (push/pop/get/iter_mut), String (push/push_str/format!/chars), HashMap (entry/get/iteracion/agrupacion) |
| 08 | 6 | Generics (<T>, <T,U>), trait bounds, where clause, traits custom, traits estandar (Debug/Clone/Copy/PartialEq/Default), lifetimes |
| 09 | 5 | Modulos, sub-modulos, pub/pub(super), re-exports (pub use), paths absolutos/relativos, use as, nested imports |

---

## Como ejecutar los tests

Desde el directorio `ejercicios/`:

```bash
cd ejercicios

# Correr TODOS los tests (216 tests en total)
cargo test

# Correr tests de un capitulo especifico
cargo test -p cap_02
cargo test -p cap_05

# Correr tests de un ejercicio especifico
cargo test -p cap_02 ej1_celsius
cargo test -p cap_06 ej3_validar_usuario
cargo test -p cap_08 ej6_lifetimes

# Ver output detallado (nombre de cada test)
cargo test -- --show-output
```

---

## Que esperar como output

Cuando corras `cargo test`, deberias ver algo asi:

```
running 25 tests
test ej1_celsius::tests::test_agua_congela ... ok
test ej1_celsius::tests::test_agua_hierve ... ok
...
test ej6_constantes_bloques::tests::test_imc_sobrepeso ... ok

test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
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
