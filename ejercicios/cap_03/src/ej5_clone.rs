// =============================================================
// Ejercicio 5: Clone
// Practica con clone() para hacer copias profundas de tipos heap.
// - clone() crea una copia independiente en el heap
// - Util cuando necesitas dos owners del mismo dato
// Practica: clone, deep copy, String y Vec
// =============================================================

/// Recibe un String, lo clona, modifica el clon y retorna ambos.
/// El original no se modifica.
pub fn clonar_y_modificar(original: &String) -> (String, String) {
    todo!("tu código va aquí")
}

/// Clona un vector y agrega un elemento al clon.
/// Retorna (original_len, clon_len) para demostrar que son independientes.
pub fn clonar_vec(original: &Vec<i32>, nuevo_elemento: i32) -> (usize, usize) {
    todo!("tu código va aquí")
}

/// Crea copias independientes de un String y las concatena
pub fn repetir_string(texto: &str, veces: usize) -> String {
    todo!("tu código va aquí")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clonar_y_modificar() {
        let original = String::from("hola");
        let (orig, clon) = clonar_y_modificar(&original);
        assert_eq!(orig, "hola");
        assert_eq!(clon, "hola (clon)");
        // original no fue modificado
        assert_eq!(original, "hola");
    }

    #[test]
    fn test_clonar_vec() {
        let original = vec![1, 2, 3];
        let (orig_len, clon_len) = clonar_vec(&original, 4);
        assert_eq!(orig_len, 3); // original no cambio
        assert_eq!(clon_len, 4); // clon tiene uno mas
    }

    #[test]
    fn test_repetir_string() {
        assert_eq!(repetir_string("ja", 3), "ja ja ja");
        assert_eq!(repetir_string("hola", 1), "hola");
    }
}
