// Capitulo 2: Variables, Types y Functions

fn main() {
    // =============================================
    // VARIABLES Y MUTABILIDAD
    // =============================================

    // Variables son immutable por defecto
    let x = 5;
    println!("x = {x}");

    // Para hacer una variable mutable, usa 'mut'
    let mut contador = 0;
    println!("contador = {contador}");
    contador += 1;
    println!("contador despues de incrementar = {contador}");

    // Shadowing: crear nueva variable con el mismo nombre
    let x = x + 1;   // shadow de x (ahora es 6)
    let x = x * 2;   // shadow de nuevo (ahora es 12)
    println!("x despues de shadowing = {x}");

    // Shadowing permite cambiar el type
    let texto = "Hola Rust";
    let texto = texto.len(); // ahora es usize, no &str
    println!("longitud del texto = {texto}");

    // =============================================
    // CONSTANTS
    // =============================================

    const MAX_INTENTOS: u32 = 3;
    const PI: f64 = 3.14159265;
    println!("Max intentos: {MAX_INTENTOS}, PI: {PI}");

    // =============================================
    // TIPOS DE DATOS (DATA TYPES)
    // =============================================

    // -- Scalar Types --

    // Integers
    let entero_i32: i32 = -42;
    let entero_u8: u8 = 255;
    let con_separador = 1_000_000; // separadores visuales
    println!("i32: {entero_i32}, u8: {entero_u8}, separador: {con_separador}");

    // Floating point
    let pi: f64 = 3.14;
    let medio: f32 = 0.5;
    println!("pi: {pi}, medio: {medio}");

    // Boolean
    let activo: bool = true;
    let mayor = 10 > 5;
    println!("activo: {activo}, 10 > 5: {mayor}");

    // Char (Unicode - 4 bytes)
    let letra: char = 'R';
    let emoji: char = '🦀';
    println!("letra: {letra}, emoji: {emoji}");

    // -- Compound Types --

    // Tuple: agrupa valores de diferentes types
    let persona: (&str, i32, bool) = ("Ana", 30, true);
    let (nombre, edad, activo) = persona; // destructuring
    println!("{nombre} tiene {edad} anios, activo: {activo}");
    println!("Acceso por indice: {}", persona.0);

    // Array: longitud fija, mismo type
    let dias = ["Lun", "Mar", "Mie", "Jue", "Vie", "Sab", "Dom"];
    println!("Primer dia: {}", dias[0]);
    println!("Ultimo dia: {}", dias[6]);

    let ceros = [0; 5]; // [0, 0, 0, 0, 0]
    println!("Array de ceros: {:?}", ceros);

    // =============================================
    // FUNCTIONS
    // =============================================

    saludar("Mundo");

    let suma = sumar(10, 25);
    println!("10 + 25 = {suma}");

    let fahrenheit = celsius_a_fahrenheit(100.0);
    println!("100°C = {fahrenheit}°F");

    // Expressions en un block
    let resultado = {
        let base = 5;
        let exponente = 2;
        base * exponente  // expression (sin ;) - retorna el valor
    };
    println!("Resultado del block: {resultado}");

    // Funcion con multiples returns usando tuple
    let (min, max) = min_max(42, 17);
    println!("min: {min}, max: {max}");
}

// Funcion sin return value
fn saludar(nombre: &str) {
    println!("Hola, {nombre}!");
}

// Funcion con return value (implicit return - sin ;)
fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

// Funcion con explicit return (usando keyword 'return')
fn celsius_a_fahrenheit(celsius: f64) -> f64 {
    return (celsius * 9.0 / 5.0) + 32.0;
}

// Retornar multiples valores con tuple
fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}
