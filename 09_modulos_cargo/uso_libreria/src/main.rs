// Ejemplo de uso de un library crate local

// Importar modules de nuestra libreria
use mi_libreria::basica;
use mi_libreria::avanzada;
use mi_libreria::utilidades;

fn main() {
    println!("=== Usando mi_libreria ===\n");

    // Operaciones basicas
    println!("--- Matematicas Basica ---");
    println!("5 + 3 = {}", basica::sumar(5.0, 3.0));
    println!("10 - 4 = {}", basica::restar(10.0, 4.0));
    println!("6 * 7 = {}", basica::multiplicar(6.0, 7.0));

    match basica::dividir(10.0, 3.0) {
        Some(r) => println!("10 / 3 = {r:.4}"),
        None => println!("Error: division por cero"),
    }

    // Operaciones avanzadas
    println!("\n--- Matematicas Avanzada ---");
    println!("5! = {}", avanzada::factorial(5));
    println!("fib(10) = {}", avanzada::fibonacci(10));
    println!("2^10 = {}", avanzada::potencia(2.0, 10));
    println!("MCD(12, 8) = {}", avanzada::mcd(12, 8));

    println!("\nPrimos del 1 al 30:");
    let primos: Vec<u64> = (1..=30).filter(|&n| avanzada::es_primo(n)).collect();
    println!("  {:?}", primos);

    // Utilidades
    println!("\n--- Utilidades ---");
    println!("Repetir: {}", utilidades::repetir("Rust", 3, " "));
    println!("Vocales en 'Hola Mundo': {}", utilidades::contar_vocales("Hola Mundo"));
    println!("Invertir 'Rust': {}", utilidades::invertir("Rust"));
    println!("Capitalizar: {}", utilidades::capitalizar_palabras("hola mundo rust"));
}
