// =============================================================
// Ejercicio 1: Dividir con Result
// Implementa una funcion que divida dos numeros.
// Retorna Err(MathError::DivisionPorCero) si el divisor es 0.
// Practica: Result, custom error types, Display
// =============================================================

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum MathError {
    DivisionPorCero,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionPorCero => write!(f, "no se puede dividir por cero"),
        }
    }
}

pub fn dividir(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionPorCero)
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_division_normal() {
        assert_eq!(dividir(10.0, 2.0), Ok(5.0));
    }

    #[test]
    fn test_division_por_cero() {
        assert_eq!(dividir(10.0, 0.0), Err(MathError::DivisionPorCero));
    }

    #[test]
    fn test_division_negativos() {
        assert_eq!(dividir(-10.0, 2.0), Ok(-5.0));
    }
}
