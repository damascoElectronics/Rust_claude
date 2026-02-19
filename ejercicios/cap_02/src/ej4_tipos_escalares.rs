// =============================================================
// Ejercicio 4: Tipos Escalares
// Practica con los distintos tipos escalares de Rust:
// enteros (i32, u32), flotantes (f64), booleanos (bool), caracteres (char).
// Practica: type annotations, casting, limites de tipos
// =============================================================

/// Retorna true si el caracter es una letra (a-z o A-Z)
pub fn es_letra(c: char) -> bool {
    match c
    {
        'a'..='z' | 'A'..='Z' => true,
        _ => false,
    }
}

/// Retorna true si el numero cabe en un u8 (0..=255)
pub fn cabe_en_u8(n: i32) -> bool {
    match n
    {
        0..=255 => true,
        _ => false,
    }
}

/// Convierte un bool a su representacion numerica: true -> 1, false -> 0
pub fn bool_a_entero(b: bool) -> i32 {
    match b
    {
        b if b => 1,
        _ => 0,
    }

}

/// Retorna el tipo de numero como string:
/// "positivo", "negativo" o "cero"
pub fn clasificar_numero(n: f64) -> &'static str {
   match n
   {
        n if n > 0.0 => "positivo",
        n if n < 0.0 => "negativo",
        _ => "cero",
   }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_es_letra() {
        assert!(es_letra('a'));
        assert!(es_letra('Z'));
        assert!(!es_letra('5'));
        assert!(!es_letra(' '));
    }

    #[test]
    fn test_cabe_en_u8() {
        assert!(cabe_en_u8(0));
        assert!(cabe_en_u8(255));
        assert!(!cabe_en_u8(256));
        assert!(!cabe_en_u8(-1));
    }

    #[test]
    fn test_bool_a_entero() {
        assert_eq!(bool_a_entero(true), 1);
        assert_eq!(bool_a_entero(false), 0);
    }

    #[test]
    fn test_clasificar_numero() {
        assert_eq!(clasificar_numero(3.14), "positivo");
        assert_eq!(clasificar_numero(-2.5), "negativo");
        assert_eq!(clasificar_numero(0.0), "cero");
    }
}
