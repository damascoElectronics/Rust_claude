// =============================================================
// Capitulo 7: Collections - Ejercicios
// =============================================================

use std::collections::HashMap;

// --- Ejercicio 1 ---
// Recibe un Vec<i32> y retorna un nuevo Vec solo con los numeros pares, ordenados.
pub fn filtrar_pares(numeros: Vec<i32>) -> Vec<i32> {
    let mut pares: Vec<i32> = numeros.into_iter().filter(|n| n % 2 == 0).collect();
    pares.sort();
    pares
}

// --- Ejercicio 2 ---
// Cuenta la frecuencia de cada palabra en un texto.
// Retorna un HashMap<String, usize> con las palabras en minusculas como key.
pub fn contar_palabras(texto: &str) -> HashMap<String, usize> {
    let mut frecuencias: HashMap<String, usize> = HashMap::new();
    for palabra in texto.split_whitespace() {
        let palabra_lower = palabra.to_lowercase();
        let contador = frecuencias.entry(palabra_lower).or_insert(0);
        *contador += 1;
    }
    frecuencias
}

// --- Ejercicio 3 ---
// Recibe un vector de strings y retorna el string mas largo.
// Si hay empate, retorna el primero encontrado. Si esta vacio retorna None.
pub fn mas_largo(textos: &[String]) -> Option<&String> {
    if textos.is_empty() {
        return None;
    }
    let mut mayor = &textos[0];
    for texto in &textos[1..] {
        if texto.len() > mayor.len() {
            mayor = texto;
        }
    }
    Some(mayor)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Tests Ejercicio 1: filtrar_pares ---

    #[test]
    fn test_filtrar_pares_mixtos() {
        assert_eq!(filtrar_pares(vec![5, 2, 8, 1, 4, 3]), vec![2, 4, 8]);
    }

    #[test]
    fn test_filtrar_pares_sin_pares() {
        assert_eq!(filtrar_pares(vec![1, 3, 5]), Vec::<i32>::new());
    }

    #[test]
    fn test_filtrar_pares_vacio() {
        assert_eq!(filtrar_pares(vec![]), Vec::<i32>::new());
    }

    // --- Tests Ejercicio 2: contar_palabras ---

    #[test]
    fn test_contar_palabras_basico() {
        let resultado = contar_palabras("hola mundo hola");
        assert_eq!(resultado.get("hola"), Some(&2));
        assert_eq!(resultado.get("mundo"), Some(&1));
    }

    #[test]
    fn test_contar_palabras_case_insensitive() {
        let resultado = contar_palabras("Rust rust RUST");
        assert_eq!(resultado.get("rust"), Some(&3));
        assert_eq!(resultado.len(), 1);
    }

    #[test]
    fn test_contar_palabras_vacio() {
        let resultado = contar_palabras("");
        assert!(resultado.is_empty());
    }

    // --- Tests Ejercicio 3: mas_largo ---

    #[test]
    fn test_mas_largo_basico() {
        let textos = vec![
            String::from("hi"),
            String::from("hola"),
            String::from("hey"),
        ];
        assert_eq!(mas_largo(&textos), Some(&String::from("hola")));
    }

    #[test]
    fn test_mas_largo_empate() {
        let textos = vec![
            String::from("abc"),
            String::from("xyz"),
        ];
        // En empate retorna el primero
        assert_eq!(mas_largo(&textos), Some(&String::from("abc")));
    }

    #[test]
    fn test_mas_largo_vacio() {
        let textos: Vec<String> = vec![];
        assert_eq!(mas_largo(&textos), None);
    }
}
