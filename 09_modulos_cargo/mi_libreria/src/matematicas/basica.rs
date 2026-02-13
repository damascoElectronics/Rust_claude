// Sub-module: matematicas::basica
// Operaciones matematicas fundamentales

/// Suma dos numeros
pub fn sumar(a: f64, b: f64) -> f64 {
    a + b
}

/// Resta b de a
pub fn restar(a: f64, b: f64) -> f64 {
    a - b
}

/// Multiplica dos numeros
pub fn multiplicar(a: f64, b: f64) -> f64 {
    a * b
}

/// Divide a entre b. Retorna None si b es cero
pub fn dividir(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

/// Calcula el valor absoluto
pub fn absoluto(n: f64) -> f64 {
    if n < 0.0 { -n } else { n }
}

// =============================================
// TESTS UNITARIOS
// =============================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sumar() {
        assert_eq!(sumar(2.0, 3.0), 5.0);
        assert_eq!(sumar(-1.0, 1.0), 0.0);
    }

    #[test]
    fn test_restar() {
        assert_eq!(restar(5.0, 3.0), 2.0);
    }

    #[test]
    fn test_multiplicar() {
        assert_eq!(multiplicar(3.0, 4.0), 12.0);
        assert_eq!(multiplicar(0.0, 100.0), 0.0);
    }

    #[test]
    fn test_dividir() {
        assert_eq!(dividir(10.0, 2.0), Some(5.0));
        assert_eq!(dividir(10.0, 0.0), None);
    }

    #[test]
    fn test_absoluto() {
        assert_eq!(absoluto(-5.0), 5.0);
        assert_eq!(absoluto(3.0), 3.0);
    }
}
