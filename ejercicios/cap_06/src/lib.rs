// =============================================================
// Capitulo 6: Manejo de Errores - Ejercicios
// =============================================================

use std::fmt;
use std::num::ParseIntError;

// --- Ejercicio 1 ---
// Implementa una funcion que divida dos numeros.
// Retorna Err si el divisor es 0, Ok con el resultado si no.
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

// --- Ejercicio 2 ---
// Parsea un string a i32 y duplica el valor.
// Usa el operador ? para propagar el error de parse.
pub fn parsear_y_duplicar(texto: &str) -> Result<i32, ParseIntError> {
    let numero: i32 = texto.parse()?;
    Ok(numero * 2)
}

// --- Ejercicio 3 ---
// Implementa una funcion que valide un nombre de usuario:
// - No puede estar vacio -> Err("nombre vacio")
// - Debe tener al menos 3 caracteres -> Err("nombre muy corto")
// - No puede tener espacios -> Err("nombre con espacios")
// - Si es valido retorna Ok con el nombre en minusculas
pub fn validar_usuario(nombre: &str) -> Result<String, &'static str> {
    if nombre.is_empty() {
        return Err("nombre vacio");
    }
    if nombre.len() < 3 {
        return Err("nombre muy corto");
    }
    if nombre.contains(' ') {
        return Err("nombre con espacios");
    }
    Ok(nombre.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Tests Ejercicio 1: dividir ---

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

    // --- Tests Ejercicio 2: parsear_y_duplicar ---

    #[test]
    fn test_parsear_valido() {
        assert_eq!(parsear_y_duplicar("21"), Ok(42));
    }

    #[test]
    fn test_parsear_invalido() {
        assert!(parsear_y_duplicar("abc").is_err());
    }

    #[test]
    fn test_parsear_negativo() {
        assert_eq!(parsear_y_duplicar("-5"), Ok(-10));
    }

    // --- Tests Ejercicio 3: validar_usuario ---

    #[test]
    fn test_usuario_valido() {
        assert_eq!(validar_usuario("RustFan"), Ok(String::from("rustfan")));
    }

    #[test]
    fn test_usuario_vacio() {
        assert_eq!(validar_usuario(""), Err("nombre vacio"));
    }

    #[test]
    fn test_usuario_corto() {
        assert_eq!(validar_usuario("ab"), Err("nombre muy corto"));
    }

    #[test]
    fn test_usuario_con_espacios() {
        assert_eq!(validar_usuario("rust fan"), Err("nombre con espacios"));
    }
}
