// =============================================================
// Ejercicio 5: Loops
// Practica con los distintos tipos de loops:
// - loop: infinito, con break que retorna valor
// - while: con condicion
// - Loop labels: 'label para loops anidados
// =============================================================

/// Usa `loop` con break para encontrar el primer multiplo de `divisor`
/// mayor o igual a `minimo`. loop retorna el valor via break.
pub fn primer_multiplo(divisor: u32, minimo: u32) -> u32 {
    todo!("tu código va aquí")
}

/// Usa `while` para contar cuantas veces se puede dividir `n` entre 2
/// antes de que sea menor a 1
pub fn contar_divisiones(mut n: f64) -> u32 {
    todo!("tu código va aquí")
}

/// Usa loop labels para buscar un valor en una "matriz" (Vec de Vecs).
/// Retorna Some((fila, columna)) si lo encuentra, None si no.
pub fn buscar_en_matriz(matriz: &Vec<Vec<i32>>, objetivo: i32) -> Option<(usize, usize)> {
    todo!("tu código va aquí")
}

/// Usa while para implementar una cuenta regresiva
/// Retorna un string como "3, 2, 1, despegue!"
pub fn cuenta_regresiva(desde: u32) -> String {
    todo!("tu código va aquí")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primer_multiplo() {
        assert_eq!(primer_multiplo(3, 10), 12); // 12 es el primer multiplo de 3 >= 10
        assert_eq!(primer_multiplo(5, 15), 15); // 15 ya es multiplo de 5
        assert_eq!(primer_multiplo(7, 1), 7);
    }

    #[test]
    fn test_contar_divisiones() {
        assert_eq!(contar_divisiones(8.0), 4);  // 8 -> 4 -> 2 -> 1 -> 0.5 (4 veces)
        assert_eq!(contar_divisiones(1.0), 1);  // 1 -> 0.5 (1 vez)
        assert_eq!(contar_divisiones(0.5), 0);  // ya es < 1
    }

    #[test]
    fn test_buscar_en_matriz() {
        let m = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        assert_eq!(buscar_en_matriz(&m, 5), Some((1, 1)));
        assert_eq!(buscar_en_matriz(&m, 9), Some((2, 2)));
        assert_eq!(buscar_en_matriz(&m, 99), None);
    }

    #[test]
    fn test_cuenta_regresiva() {
        assert_eq!(cuenta_regresiva(3), "3, 2, 1, despegue!");
        assert_eq!(cuenta_regresiva(1), "1, despegue!");
    }
}
