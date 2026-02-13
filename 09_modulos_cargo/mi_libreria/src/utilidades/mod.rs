// Module utilidades: funciones de uso general

/// Repite un string n veces separado por un separador
pub fn repetir(texto: &str, n: usize, separador: &str) -> String {
    let partes: Vec<&str> = std::iter::repeat(texto).take(n).collect();
    partes.join(separador)
}

/// Cuenta las vocales en un string
pub fn contar_vocales(texto: &str) -> usize {
    texto
        .chars()
        .filter(|c| "aeiouAEIOU".contains(*c))
        .count()
}

/// Invierte un string
pub fn invertir(texto: &str) -> String {
    texto.chars().rev().collect()
}

/// Capitaliza la primera letra de cada palabra
pub fn capitalizar_palabras(texto: &str) -> String {
    texto
        .split_whitespace()
        .map(|palabra| {
            let mut chars = palabra.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => {
                    let upper: String = c.to_uppercase().collect();
                    upper + &chars.as_str().to_lowercase()
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repetir() {
        assert_eq!(repetir("ha", 3, " "), "ha ha ha");
        assert_eq!(repetir("ab", 2, "-"), "ab-ab");
    }

    #[test]
    fn test_contar_vocales() {
        assert_eq!(contar_vocales("Hola Mundo"), 4);
        assert_eq!(contar_vocales("xyz"), 0);
    }

    #[test]
    fn test_invertir() {
        assert_eq!(invertir("Rust"), "tsuR");
        assert_eq!(invertir("abc"), "cba");
    }

    #[test]
    fn test_capitalizar_palabras() {
        assert_eq!(capitalizar_palabras("hola mundo"), "Hola Mundo");
        assert_eq!(capitalizar_palabras("RUST es GENIAL"), "Rust Es Genial");
    }
}
