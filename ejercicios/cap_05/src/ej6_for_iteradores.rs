// =============================================================
// Ejercicio 6: For e Iteradores
// Practica con for loops:
// - for con ranges (0..5, 1..=10)
// - for con enumerate() (indice + valor)
// - for con rev() (iteracion inversa)
// - for iterando colecciones con &
// =============================================================

/// Usa for con range para sumar todos los numeros de 1 a n (inclusive)
pub fn suma_hasta(n: u32) -> u32 {
    todo!("tu código va aquí")
}

/// Usa enumerate para encontrar el indice del primer numero negativo.
/// Retorna None si no hay negativos.
pub fn indice_primer_negativo(numeros: &[i32]) -> Option<usize> {
    todo!("tu código va aquí")
}

/// Usa rev() para invertir un string caracter por caracter
pub fn invertir_string(texto: &str) -> String {
    todo!("tu código va aquí")
}

/// Usa for para iterar sobre una coleccion y construir un string
/// con el formato "1. elemento\n2. elemento\n..."
pub fn lista_numerada(elementos: &[&str]) -> String {
    todo!("tu código va aquí")
}

/// Usa for con range para generar los primeros `n` numeros pares (empezando en 2)
pub fn primeros_pares(n: usize) -> Vec<i32> {
    todo!("tu código va aquí")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suma_hasta() {
        assert_eq!(suma_hasta(10), 55);
        assert_eq!(suma_hasta(1), 1);
        assert_eq!(suma_hasta(100), 5050);
    }

    #[test]
    fn test_indice_primer_negativo() {
        assert_eq!(indice_primer_negativo(&[1, 2, -3, 4]), Some(2));
        assert_eq!(indice_primer_negativo(&[-1, 2, 3]), Some(0));
        assert_eq!(indice_primer_negativo(&[1, 2, 3]), None);
    }

    #[test]
    fn test_invertir_string() {
        assert_eq!(invertir_string("hola"), "aloh");
        assert_eq!(invertir_string("Rust"), "tsuR");
        assert_eq!(invertir_string(""), "");
    }

    #[test]
    fn test_lista_numerada() {
        assert_eq!(
            lista_numerada(&["rust", "cargo", "crate"]),
            "1. rust\n2. cargo\n3. crate"
        );
    }

    #[test]
    fn test_primeros_pares() {
        assert_eq!(primeros_pares(5), vec![2, 4, 6, 8, 10]);
        assert_eq!(primeros_pares(1), vec![2]);
    }
}
