// =============================================================
// Ejercicio 4: Copy vs Move
// Demuestra la diferencia entre tipos Copy (stack) y Move (heap).
// - Tipos Copy: i32, f64, bool, char -> se copian automaticamente
// - Tipos Move: String, Vec -> transfieren ownership
// Practica: Copy trait, move semantics, stack vs heap
// =============================================================

/// Recibe dos i32 (Copy), los suma, y retorna los tres valores.
/// Como i32 implementa Copy, a y b siguen validos despues de usarlos.
pub fn sumar_y_retornar(a: i32, b: i32) -> (i32, i32, i32) {
    todo!("tu código va aquí")
}

/// Demuestra que un String se mueve: toma ownership y retorna
/// el largo y el string original (devolviendo ownership).
pub fn largo_con_ownership(s: String) -> (usize, String) {
    todo!("tu código va aquí")
}

/// Simula el comportamiento de Copy: duplica un valor de tipo Copy
/// El original sigue siendo valido despues de la copia
pub fn duplicar_valor(n: i32) -> (i32, i32) {
    todo!("tu código va aquí")
}

/// Recibe valores de distintos tipos Copy y retorna un resumen
pub fn resumen_tipos(entero: i32, flotante: f64, booleano: bool, caracter: char) -> String {
    todo!("tu código va aquí")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sumar_y_retornar() {
        let (a, b, suma) = sumar_y_retornar(3, 7);
        assert_eq!(a, 3);
        assert_eq!(b, 7);
        assert_eq!(suma, 10);
    }

    #[test]
    fn test_largo_con_ownership() {
        let s = String::from("hola");
        let (largo, s_devuelto) = largo_con_ownership(s);
        assert_eq!(largo, 4);
        assert_eq!(s_devuelto, "hola");
        // s ya no es valido aqui, pero s_devuelto si
    }

    #[test]
    fn test_duplicar_valor() {
        let (original, copia) = duplicar_valor(42);
        assert_eq!(original, 42);
        assert_eq!(copia, 42);
    }

    #[test]
    fn test_resumen_tipos() {
        let resultado = resumen_tipos(42, 3.14, true, 'R');
        assert_eq!(resultado, "entero=42, flotante=3.14, bool=true, char='R'");
    }
}
