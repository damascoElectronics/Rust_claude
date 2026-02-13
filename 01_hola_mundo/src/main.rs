// Capitulo 1: Hola Mundo
// Este es tu primer programa en Rust

// La funcion main() es el entry point de todo programa Rust
fn main() {
    // println! es un macro que imprime texto en la consola
    println!("Hola, mundo!");

    // Puedes usar {} como placeholder para insertar valores
    let nombre = "Rustacean";
    println!("Bienvenido, {}!", nombre);

    // Multiples placeholders
    let lenguaje = "Rust";
    let anio = 2015;
    println!("{} fue lanzado en {}", lenguaje, anio);

    // Tambien puedes usar posiciones numeradas
    println!("{0} es rapido. {0} es seguro. {0} es genial!", lenguaje);

    // O nombres de variables directamente (Rust 1.58+)
    let version = "2021";
    println!("Edicion: {version}");
}
