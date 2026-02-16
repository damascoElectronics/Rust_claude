// =============================================================
// Ejercicio 7: String Slices
// Practica con &str y slicing de strings:
// - &s[start..end] para obtener porciones
// - &str como parametro (mas flexible que &String)
// - Slices de arrays tambien
// Practica: &str, string slicing, slice de arrays
// =============================================================

/// Retorna la primera palabra de un string (hasta el primer espacio).
/// Si no hay espacios, retorna todo el string.
pub fn primera_palabra(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

/// Retorna la ultima palabra de un string (desde el ultimo espacio).
pub fn ultima_palabra(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate().rev() {
        if byte == b' ' {
            return &s[i + 1..];
        }
    }
    s
}

/// Retorna un sub-slice de un array: los elementos desde `inicio` hasta `fin` (exclusive)
pub fn sub_slice(numeros: &[i32], inicio: usize, fin: usize) -> &[i32] {
    &numeros[inicio..fin]
}

/// Cuenta cuantas palabras tiene un string (separadas por espacios)
pub fn contar_palabras(s: &str) -> usize {
    if s.trim().is_empty() {
        return 0;
    }
    s.split_whitespace().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primera_palabra() {
        assert_eq!(primera_palabra("hola mundo"), "hola");
        assert_eq!(primera_palabra("solo"), "solo");
        assert_eq!(primera_palabra("tres palabras aqui"), "tres");
    }

    #[test]
    fn test_ultima_palabra() {
        assert_eq!(ultima_palabra("hola mundo"), "mundo");
        assert_eq!(ultima_palabra("solo"), "solo");
        assert_eq!(ultima_palabra("a b c"), "c");
    }

    #[test]
    fn test_sub_slice() {
        let nums = [10, 20, 30, 40, 50];
        assert_eq!(sub_slice(&nums, 1, 4), &[20, 30, 40]);
        assert_eq!(sub_slice(&nums, 0, 2), &[10, 20]);
    }

    #[test]
    fn test_contar_palabras() {
        assert_eq!(contar_palabras("hola mundo"), 2);
        assert_eq!(contar_palabras("una"), 1);
        assert_eq!(contar_palabras(""), 0);
        assert_eq!(contar_palabras("  espacios  extra  "), 2);
    }
}
