// =============================================================
// Capitulo 2: Variables, Tipos y Funciones - Ejercicios
// =============================================================

// --- Ejercicio 1 ---
// Convierte grados Celsius a Fahrenheit usando la formula: F = (C * 9/5) + 32
pub fn celsius_a_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

// --- Ejercicio 2 ---
// Calcula el n-esimo numero de Fibonacci (0-indexed).
// fibonacci(0) = 0, fibonacci(1) = 1, fibonacci(2) = 1, fibonacci(3) = 2, ...
pub fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 1..n {
        let temp = b;
        b = a + b;
        a = temp;
    }
    b
}

// --- Ejercicio 3 ---
// Usa shadowing para:
// 1. Recibir un &str
// 2. Retornar su longitud (usize) multiplicada por 2
// Internamente debes usar shadowing: primero guardar el len, luego shadow con *2
pub fn doble_longitud(texto: &str) -> usize {
    let resultado = texto.len();
    let resultado = resultado * 2; // shadowing: cambia el valor
    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Tests Ejercicio 1: Celsius a Fahrenheit ---

    #[test]
    fn test_celsius_agua_congela() {
        // El agua congela a 0°C = 32°F
        let resultado = celsius_a_fahrenheit(0.0);
        assert_eq!(resultado, 32.0, "0°C deberia ser 32°F");
    }

    #[test]
    fn test_celsius_agua_hierve() {
        // El agua hierve a 100°C = 212°F
        let resultado = celsius_a_fahrenheit(100.0);
        assert_eq!(resultado, 212.0, "100°C deberia ser 212°F");
    }

    #[test]
    fn test_celsius_negativo() {
        // -40°C = -40°F (punto donde coinciden)
        let resultado = celsius_a_fahrenheit(-40.0);
        assert_eq!(resultado, -40.0, "-40°C deberia ser -40°F");
    }

    #[test]
    fn test_celsius_temperatura_corporal() {
        let resultado = celsius_a_fahrenheit(37.0);
        assert!((resultado - 98.6).abs() < 0.01, "37°C deberia ser ~98.6°F");
    }

    // --- Tests Ejercicio 2: Fibonacci ---

    #[test]
    fn test_fibonacci_cero() {
        assert_eq!(fibonacci(0), 0, "fibonacci(0) = 0");
    }

    #[test]
    fn test_fibonacci_uno() {
        assert_eq!(fibonacci(1), 1, "fibonacci(1) = 1");
    }

    #[test]
    fn test_fibonacci_diez() {
        assert_eq!(fibonacci(10), 55, "fibonacci(10) = 55");
    }

    #[test]
    fn test_fibonacci_veinte() {
        assert_eq!(fibonacci(20), 6765, "fibonacci(20) = 6765");
    }

    // --- Tests Ejercicio 3: Doble longitud (shadowing) ---

    #[test]
    fn test_doble_longitud_hola() {
        assert_eq!(doble_longitud("hola"), 8, "'hola' tiene 4 chars, doble = 8");
    }

    #[test]
    fn test_doble_longitud_vacio() {
        assert_eq!(doble_longitud(""), 0, "string vacio tiene longitud 0");
    }

    #[test]
    fn test_doble_longitud_rust() {
        assert_eq!(doble_longitud("Rust"), 8, "'Rust' tiene 4 chars, doble = 8");
    }
}
