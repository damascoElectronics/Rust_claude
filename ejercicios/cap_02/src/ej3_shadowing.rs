// =============================================================
// Ejercicio 3: Shadowing
// Usa shadowing para:
// 1. Recibir un &str
// 2. Obtener su longitud con .len()
// 3. Hacer shadow del resultado multiplicandolo por 2
// 4. Retornar el valor final (usize)
// =============================================================

pub fn doble_longitud(texto: &str) -> usize {
    let resultado = texto.len();
    let resultado = resultado * 2; // shadowing: cambia el valor
    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doble_longitud_hola() {
        assert_eq!(doble_longitud("hola"), 8); // 4 chars * 2
    }

    #[test]
    fn test_doble_longitud_vacio() {
        assert_eq!(doble_longitud(""), 0);
    }

    #[test]
    fn test_doble_longitud_rust() {
        assert_eq!(doble_longitud("Rust"), 8); // 4 chars * 2
    }
}
