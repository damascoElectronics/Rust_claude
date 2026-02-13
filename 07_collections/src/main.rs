// Capitulo 7: Collections (Vec, String, HashMap)

use std::collections::HashMap;

fn main() {
    // =============================================
    // VEC<T> (VECTOR)
    // =============================================
    println!("=== VEC<T> ===\n");

    // Crear vectores
    let mut numeros: Vec<i32> = Vec::new();
    numeros.push(10);
    numeros.push(20);
    numeros.push(30);
    println!("Vector: {:?}", numeros);

    // Crear con macro
    let frutas = vec!["manzana", "banana", "cereza", "durazno"];
    println!("Frutas: {:?}", frutas);

    // Acceso por indice
    println!("Primera fruta: {}", frutas[0]);

    // Acceso seguro con .get()
    match frutas.get(10) {
        Some(f) => println!("Fruta 10: {f}"),
        None => println!("No hay fruta en indice 10"),
    }

    // Iterar y modificar
    let mut valores = vec![1, 2, 3, 4, 5];
    for v in &mut valores {
        *v *= 10; // dereference y multiplicar
    }
    println!("Valores x10: {:?}", valores);

    // Metodos utiles
    println!("Longitud: {}", valores.len());
    println!("Contiene 30? {}", valores.contains(&30));

    let ultimo = valores.pop();
    println!("Pop: {:?}, Vec: {:?}", ultimo, valores);

    // Slices de vectores
    let primeros_dos = &valores[0..2];
    println!("Primeros dos: {:?}", primeros_dos);

    // Ordenar
    let mut desordenado = vec![5, 2, 8, 1, 9, 3];
    desordenado.sort();
    println!("Ordenado: {:?}", desordenado);

    // Filter, map, collect
    let pares: Vec<i32> = (1..=10).filter(|n| n % 2 == 0).collect();
    println!("Pares 1-10: {:?}", pares);

    let cuadrados: Vec<i32> = (1..=5).map(|n| n * n).collect();
    println!("Cuadrados: {:?}", cuadrados);

    // =============================================
    // STRING
    // =============================================
    println!("\n=== STRING ===\n");

    // Crear strings
    let mut saludo = String::new();
    saludo.push_str("Hola");
    saludo.push(' ');
    saludo.push_str("Rust");
    println!("saludo: {saludo}");

    let desde_literal = String::from("Desde literal");
    let con_to_string = "Con to_string".to_string();
    println!("{desde_literal}, {con_to_string}");

    // Concatenar con format! (no mueve ownership)
    let nombre = String::from("Rust");
    let version = String::from("2021");
    let completo = format!("{nombre} edition {version}");
    println!("{completo}");
    println!("nombre sigue valido: {nombre}"); // no se movio

    // Concatenar con + (mueve el primer operando)
    let a = String::from("Hola ");
    let b = String::from("Mundo");
    let c = a + &b; // a se mueve, b se presta
    println!("a + &b = {c}");
    // println!("{a}"); // ERROR: a fue movido

    // Longitud en bytes vs caracteres
    let emoji_text = String::from("Hola 🦀!");
    println!("'{emoji_text}': {} bytes, {} chars",
        emoji_text.len(),
        emoji_text.chars().count()
    );

    // Iterar por caracteres
    print!("Caracteres: ");
    for c in "Rust 🦀".chars() {
        print!("[{c}] ");
    }
    println!();

    // Metodos utiles de String
    let texto = String::from("  Hola Mundo  ");
    println!("trim: '{}'", texto.trim());
    println!("to_uppercase: {}", texto.trim().to_uppercase());
    println!("to_lowercase: {}", texto.trim().to_lowercase());
    println!("contains 'Mundo': {}", texto.contains("Mundo"));
    println!("replace: {}", texto.trim().replace("Mundo", "Rust"));

    // Split
    let csv = "rojo,verde,azul,amarillo";
    let colores: Vec<&str> = csv.split(',').collect();
    println!("Split: {:?}", colores);

    // =============================================
    // HASHMAP<K, V>
    // =============================================
    println!("\n=== HASHMAP<K, V> ===\n");

    // Crear y llenar un HashMap
    let mut puntuaciones: HashMap<String, i32> = HashMap::new();
    puntuaciones.insert(String::from("Equipo Azul"), 100);
    puntuaciones.insert(String::from("Equipo Rojo"), 85);
    puntuaciones.insert(String::from("Equipo Verde"), 92);

    println!("Puntuaciones: {:?}", puntuaciones);

    // Acceder a un valor
    let equipo = "Equipo Azul";
    match puntuaciones.get(equipo) {
        Some(puntos) => println!("{equipo}: {puntos} puntos"),
        None => println!("{equipo} no encontrado"),
    }

    // Iterar
    println!("\nTabla de puntuaciones:");
    for (equipo, puntos) in &puntuaciones {
        println!("  {equipo}: {puntos}");
    }

    // entry: insertar solo si no existe
    puntuaciones
        .entry(String::from("Equipo Amarillo"))
        .or_insert(50);
    puntuaciones
        .entry(String::from("Equipo Azul"))
        .or_insert(999); // no cambia, ya existe
    println!("\nDespues de entry: {:?}", puntuaciones);

    // Contar palabras con HashMap
    println!("\n--- Contador de palabras ---");
    let texto = "rust es genial rust es rapido rust es seguro";
    let mut conteo: HashMap<&str, i32> = HashMap::new();

    for palabra in texto.split_whitespace() {
        let cuenta = conteo.entry(palabra).or_insert(0);
        *cuenta += 1;
    }

    let mut conteo_vec: Vec<(&&str, &i32)> = conteo.iter().collect();
    conteo_vec.sort_by(|a, b| b.1.cmp(a.1)); // ordenar por frecuencia

    for (palabra, cuenta) in conteo_vec {
        println!("  '{palabra}': {cuenta} veces");
    }

    // Crear HashMap desde iteradores
    let equipos = vec!["Alpha", "Beta", "Gamma"];
    let scores = vec![95, 87, 91];
    let tabla: HashMap<_, _> = equipos.into_iter().zip(scores.into_iter()).collect();
    println!("\nTabla desde zip: {:?}", tabla);

    // =============================================
    // EJERCICIO: ESTADISTICAS DE UN VECTOR
    // =============================================
    println!("\n=== ESTADISTICAS ===\n");

    let datos = vec![4, 2, 7, 2, 8, 2, 5, 7, 1, 9, 2, 5];
    println!("Datos: {:?}", datos);
    println!("Media: {:.2}", media(&datos));
    println!("Mediana: {:.1}", mediana(&datos));
    println!("Moda: {}", moda(&datos));
}

fn media(datos: &[i32]) -> f64 {
    let suma: i32 = datos.iter().sum();
    suma as f64 / datos.len() as f64
}

fn mediana(datos: &[i32]) -> f64 {
    let mut ordenados = datos.to_vec();
    ordenados.sort();
    let len = ordenados.len();
    if len % 2 == 0 {
        (ordenados[len / 2 - 1] + ordenados[len / 2]) as f64 / 2.0
    } else {
        ordenados[len / 2] as f64
    }
}

fn moda(datos: &[i32]) -> i32 {
    let mut conteo = HashMap::new();
    for &n in datos {
        *conteo.entry(n).or_insert(0) += 1;
    }
    *conteo.iter().max_by_key(|(_, &v)| v).unwrap().0
}
