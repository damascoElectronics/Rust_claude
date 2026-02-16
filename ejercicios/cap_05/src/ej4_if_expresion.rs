// =============================================================
// Ejercicio 4: If como Expresion
// En Rust, if/else es una expresion que retorna un valor.
// Se puede asignar el resultado de un if a una variable.
// Practica: if expressions, ternary-like syntax
// =============================================================

/// Retorna "par" o "impar" usando if como expresion
pub fn par_o_impar(n: i32) -> &'static str {
    if n % 2 == 0 { "par" } else { "impar" }
}

/// Retorna el valor absoluto usando if como expresion
pub fn valor_absoluto(n: i32) -> i32 {
    if n >= 0 { n } else { -n }
}

/// Asigna una calificacion basada en un puntaje:
/// 90+ -> "A", 80+ -> "B", 70+ -> "C", 60+ -> "D", menos -> "F"
pub fn calificacion(puntaje: u32) -> &'static str {
    if puntaje >= 90 {
        "A"
    } else if puntaje >= 80 {
        "B"
    } else if puntaje >= 70 {
        "C"
    } else if puntaje >= 60 {
        "D"
    } else {
        "F"
    }
}

/// Retorna el mayor de tres numeros usando if expressions anidados
pub fn mayor_de_tres(a: i32, b: i32, c: i32) -> i32 {
    let mayor_ab = if a > b { a } else { b };
    if mayor_ab > c { mayor_ab } else { c }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_par_o_impar() {
        assert_eq!(par_o_impar(4), "par");
        assert_eq!(par_o_impar(7), "impar");
        assert_eq!(par_o_impar(0), "par");
    }

    #[test]
    fn test_valor_absoluto() {
        assert_eq!(valor_absoluto(5), 5);
        assert_eq!(valor_absoluto(-5), 5);
        assert_eq!(valor_absoluto(0), 0);
    }

    #[test]
    fn test_calificacion() {
        assert_eq!(calificacion(95), "A");
        assert_eq!(calificacion(85), "B");
        assert_eq!(calificacion(75), "C");
        assert_eq!(calificacion(65), "D");
        assert_eq!(calificacion(50), "F");
    }

    #[test]
    fn test_mayor_de_tres() {
        assert_eq!(mayor_de_tres(1, 2, 3), 3);
        assert_eq!(mayor_de_tres(9, 5, 7), 9);
        assert_eq!(mayor_de_tres(4, 8, 2), 8);
    }
}
