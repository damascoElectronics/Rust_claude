// =============================================================
// Ejercicio 1: Modulo Matematica
// Organiza codigo en sub-modulos: basica (sumar, restar, multiplicar)
// y avanzada (potencia, factorial). Usa pub use para re-exports.
// Practica: modulos anidados, visibilidad, re-exports
// =============================================================

pub mod basica {
    pub fn sumar(a: f64, b: f64) -> f64 {
        a + b
    }

    pub fn restar(a: f64, b: f64) -> f64 {
        a - b
    }

    pub fn multiplicar(a: f64, b: f64) -> f64 {
        a * b
    }
}

pub mod avanzada {
    pub fn potencia(base: f64, exponente: u32) -> f64 {
        let mut resultado = 1.0;
        for _ in 0..exponente {
            resultado *= base;
        }
        resultado
    }

    pub fn factorial(n: u64) -> u64 {
        if n <= 1 {
            1
        } else {
            n * factorial(n - 1)
        }
    }
}

// Re-exports de las funciones mas usadas
pub use basica::sumar;
pub use avanzada::factorial;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sumar() {
        assert_eq!(basica::sumar(2.0, 3.0), 5.0);
    }

    #[test]
    fn test_potencia() {
        assert_eq!(avanzada::potencia(2.0, 10), 1024.0);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(5), 120); // via re-export
        assert_eq!(avanzada::factorial(0), 1);
    }

    #[test]
    fn test_re_export() {
        assert_eq!(sumar(10.0, 5.0), 15.0);
    }
}
