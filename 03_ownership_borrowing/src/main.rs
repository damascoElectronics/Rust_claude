// Capitulo 3: Ownership y Borrowing
// El concepto mas importante y unico de Rust

fn main() {
    // =============================================
    // OWNERSHIP BASICO
    // =============================================

    // Tipos en el stack implementan Copy trait
    let x = 5;
    let y = x; // Copy - ambos son validos
    println!("Copy: x={x}, y={y}");

    // Tipos en el heap se mueven (move)
    let s1 = String::from("hola");
    let s2 = s1; // Move - s1 ya NO es valido
    // println!("{s1}"); // ERROR si descomentas: value used after move
    println!("Move: s2={s2}");

    // Clone: copia profunda explicita
    let s3 = String::from("mundo");
    let s4 = s3.clone();
    println!("Clone: s3={s3}, s4={s4}");

    // =============================================
    // OWNERSHIP Y FUNCTIONS
    // =============================================

    let mensaje = String::from("Rust es genial");
    tomar_ownership(mensaje);
    // println!("{mensaje}"); // ERROR: mensaje fue movido a la funcion

    let numero = 42;
    hacer_copia(numero);
    println!("Despues de hacer_copia: numero={numero}"); // OK, i32 es Copy

    // Retornar ownership desde una funcion
    let s = crear_string();
    println!("String creado en funcion: {s}");

    // Dar y recibir ownership
    let s = String::from("ida y vuelta");
    let s = dar_y_recibir(s);
    println!("Ida y vuelta: {s}");

    // =============================================
    // REFERENCES Y BORROWING
    // =============================================

    // Immutable reference (&T)
    let texto = String::from("Hola Rust");
    let longitud = calcular_longitud(&texto); // prestamos texto
    println!("'{texto}' tiene {longitud} caracteres"); // texto sigue valido

    // Mutable reference (&mut T)
    let mut saludo = String::from("Hola");
    agregar_texto(&mut saludo);
    println!("Despues de agregar: {saludo}");

    // Multiples immutable references - OK
    let dato = String::from("compartido");
    let r1 = &dato;
    let r2 = &dato;
    println!("Multiples refs: {r1}, {r2}");

    // Solo una mutable reference a la vez
    let mut dato = String::from("exclusivo");
    {
        let r1 = &mut dato;
        r1.push_str("!");
        println!("Mutable ref: {r1}");
    } // r1 sale del scope aqui
    let r2 = &mut dato; // OK - r1 ya no existe
    r2.push_str("!");
    println!("Segunda mutable ref: {r2}");

    // =============================================
    // SLICES
    // =============================================

    let frase = String::from("Rust es seguro y rapido");

    let primera = primera_palabra(&frase);
    println!("Primera palabra: {primera}");

    // Array slices
    let numeros = [1, 2, 3, 4, 5];
    let medio = &numeros[1..4]; // [2, 3, 4]
    println!("Slice de array: {:?}", medio);
}

// Funcion que toma ownership (el String se mueve aqui)
fn tomar_ownership(s: String) {
    println!("Tome ownership de: {s}");
} // s se libera aqui (drop)

// Funcion con tipo Copy (se copia, no se mueve)
fn hacer_copia(n: i32) {
    println!("Copia del numero: {n}");
}

// Funcion que crea y retorna un String (transfiere ownership al caller)
fn crear_string() -> String {
    String::from("creado en funcion")
}

// Funcion que recibe y retorna ownership
fn dar_y_recibir(s: String) -> String {
    println!("Tengo: {s}");
    s // retorna ownership
}

// Funcion con immutable reference (borrowing)
fn calcular_longitud(s: &String) -> usize {
    s.len()
} // s es solo una referencia, el valor original NO se libera

// Funcion con mutable reference
fn agregar_texto(s: &mut String) {
    s.push_str(" Mundo");
}

// Funcion que retorna un string slice
fn primera_palabra(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}
