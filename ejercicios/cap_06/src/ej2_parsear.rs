// =============================================================
// Ejercicio 2: Parsear y Duplicar
// Parsea un string a i32 y duplica el valor.
// Usa el operador ? para propagar el error de parse.
// Practica: operador ?, propagacion de errores
// =============================================================

use std::num::ParseIntError;

pub fn parsear_y_duplicar(texto: &str) -> Result<i32, ParseIntError> {
    let numero: i32 = texto.parse()?;
    Ok(numero * 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valido() {
        assert_eq!(parsear_y_duplicar("21"), Ok(42));
    }

    #[test]
    fn test_invalido() {
        assert!(parsear_y_duplicar("abc").is_err());
    }

    #[test]
    fn test_negativo() {
        assert_eq!(parsear_y_duplicar("-5"), Ok(-10));
    }
}
