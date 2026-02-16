// =============================================================
// Ejercicio 5: Metodos de String
// Practica con String y &str:
// - String::from(), String::new(), push_str(), push()
// - format! macro para concatenar
// - .chars() para iterar sobre caracteres
// - Slicing con [start..end]
// Practica: String API, format!, chars iteration
// =============================================================

/// Construye un String caracter por caracter usando push()
pub fn construir_abecedario(hasta: char) -> String {
    let mut resultado = String::new();
    for c in 'a'..=hasta {
        resultado.push(c);
    }
    resultado
}

/// Usa format! para crear un string formateado con multiples valores
pub fn ficha_persona(nombre: &str, edad: u32, ciudad: &str) -> String {
    format!("Nombre: {} | Edad: {} | Ciudad: {}", nombre, edad, ciudad)
}

/// Cuenta vocales en un string usando .chars()
pub fn contar_vocales(texto: &str) -> usize {
    texto
        .chars()
        .filter(|c| "aeiouAEIOU".contains(*c))
        .count()
}

/// Invierte cada palabra individualmente pero mantiene el orden de palabras.
/// "hola mundo" -> "aloh odnum"
pub fn invertir_palabras(texto: &str) -> String {
    texto
        .split_whitespace()
        .map(|palabra| palabra.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

/// Usa push_str para concatenar partes con un separador
pub fn unir_con_separador(partes: &[&str], sep: &str) -> String {
    let mut resultado = String::new();
    for (i, parte) in partes.iter().enumerate() {
        resultado.push_str(parte);
        if i < partes.len() - 1 {
            resultado.push_str(sep);
        }
    }
    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construir_abecedario() {
        assert_eq!(construir_abecedario('e'), "abcde");
        assert_eq!(construir_abecedario('a'), "a");
    }

    #[test]
    fn test_ficha_persona() {
        assert_eq!(
            ficha_persona("Ana", 25, "Buenos Aires"),
            "Nombre: Ana | Edad: 25 | Ciudad: Buenos Aires"
        );
    }

    #[test]
    fn test_contar_vocales() {
        assert_eq!(contar_vocales("hola mundo"), 4);
        assert_eq!(contar_vocales("xyz"), 0);
        assert_eq!(contar_vocales("AEIOU"), 5);
    }

    #[test]
    fn test_invertir_palabras() {
        assert_eq!(invertir_palabras("hola mundo"), "aloh odnum");
        assert_eq!(invertir_palabras("rust"), "tsur");
    }

    #[test]
    fn test_unir_con_separador() {
        assert_eq!(unir_con_separador(&["a", "b", "c"], " - "), "a - b - c");
        assert_eq!(unir_con_separador(&["solo"], ","), "solo");
    }
}
