// =============================================================
// Ejercicio 5: Propagacion de Errores con ?
// Practica encadenando multiples operaciones que pueden fallar
// usando el operador ? para propagar errores automaticamente.
// Practica: ?, From trait, multiples tipos de error, cadenas
// =============================================================

use std::fmt;
use std::num::ParseIntError;

/// Error custom que puede ser de parseo o de validacion
#[derive(Debug, PartialEq)]
pub enum AppError {
    Parseo(String),
    Validacion(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Parseo(msg) => write!(f, "Error de parseo: {}", msg),
            AppError::Validacion(msg) => write!(f, "Error de validacion: {}", msg),
        }
    }
}

/// Implementa From para convertir ParseIntError en AppError automaticamente
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parseo(e.to_string())
    }
}

/// Parsea un string a i32, valida que sea positivo, y retorna su cuadrado.
/// Usa ? para propagar el error de parseo y valida manualmente.
pub fn parsear_y_cuadrado(texto: &str) -> Result<i32, AppError> {
    let n: i32 = texto.parse()?; // ? convierte ParseIntError a AppError via From
    if n <= 0 {
        return Err(AppError::Validacion(String::from("debe ser positivo")));
    }
    Ok(n * n)
}

/// Procesa una lista de strings: parsea cada uno, suma los resultados.
/// Se detiene al primer error.
pub fn sumar_textos(textos: &[&str]) -> Result<i32, AppError> {
    let mut suma = 0;
    for texto in textos {
        let n: i32 = texto.parse()?;
        suma += n;
    }
    Ok(suma)
}

/// Cadena de operaciones: parsea dos strings, los divide.
/// Puede fallar por parseo o por division por cero.
pub fn dividir_textos(texto_a: &str, texto_b: &str) -> Result<f64, AppError> {
    let a: i32 = texto_a.parse()?;
    let b: i32 = texto_b.parse()?;
    if b == 0 {
        return Err(AppError::Validacion(String::from("division por cero")));
    }
    Ok(a as f64 / b as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsear_y_cuadrado_ok() {
        assert_eq!(parsear_y_cuadrado("5"), Ok(25));
    }

    #[test]
    fn test_parsear_y_cuadrado_negativo() {
        assert_eq!(
            parsear_y_cuadrado("-3"),
            Err(AppError::Validacion(String::from("debe ser positivo")))
        );
    }

    #[test]
    fn test_parsear_y_cuadrado_invalido() {
        assert!(matches!(parsear_y_cuadrado("abc"), Err(AppError::Parseo(_))));
    }

    #[test]
    fn test_sumar_textos_ok() {
        assert_eq!(sumar_textos(&["10", "20", "30"]), Ok(60));
    }

    #[test]
    fn test_sumar_textos_error() {
        assert!(sumar_textos(&["10", "abc", "30"]).is_err());
    }

    #[test]
    fn test_dividir_textos_ok() {
        assert_eq!(dividir_textos("10", "4"), Ok(2.5));
    }

    #[test]
    fn test_dividir_textos_division_cero() {
        assert_eq!(
            dividir_textos("10", "0"),
            Err(AppError::Validacion(String::from("division por cero")))
        );
    }

    #[test]
    fn test_dividir_textos_parse_error() {
        assert!(matches!(dividir_textos("abc", "2"), Err(AppError::Parseo(_))));
    }
}
