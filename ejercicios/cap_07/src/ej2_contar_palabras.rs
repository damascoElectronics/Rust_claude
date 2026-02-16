// =============================================================
// Ejercicio 2: Contar Palabras
// Cuenta la frecuencia de cada palabra en un texto.
// Retorna HashMap<String, usize> con palabras en minusculas.
// Practica: HashMap, entry API, iteradores sobre strings
// =============================================================

use std::collections::HashMap;

pub fn contar_palabras(texto: &str) -> HashMap<String, usize> {
    let mut frecuencias: HashMap<String, usize> = HashMap::new();
    for palabra in texto.split_whitespace() {
        let palabra_lower = palabra.to_lowercase();
        let contador = frecuencias.entry(palabra_lower).or_insert(0);
        *contador += 1;
    }
    frecuencias
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basico() {
        let resultado = contar_palabras("hola mundo hola");
        assert_eq!(resultado.get("hola"), Some(&2));
        assert_eq!(resultado.get("mundo"), Some(&1));
    }

    #[test]
    fn test_case_insensitive() {
        let resultado = contar_palabras("Rust rust RUST");
        assert_eq!(resultado.get("rust"), Some(&3));
        assert_eq!(resultado.len(), 1);
    }

    #[test]
    fn test_vacio() {
        let resultado = contar_palabras("");
        assert!(resultado.is_empty());
    }
}
