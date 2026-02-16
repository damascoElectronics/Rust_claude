// =============================================================
// Ejercicio 1: Mayor Generico
// Implementa una funcion generica que encuentre el mayor valor
// en un slice. T debe implementar PartialOrd + Copy.
// Practica: generics, trait bounds
// =============================================================

pub fn mayor<T: PartialOrd + Copy>(lista: &[T]) -> Option<T> {
    if lista.is_empty() {
        return None;
    }
    let mut max = lista[0];
    for &item in &lista[1..] {
        if item > max {
            max = item;
        }
    }
    Some(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enteros() {
        assert_eq!(mayor(&[1, 5, 3, 9, 2]), Some(9));
    }

    #[test]
    fn test_flotantes() {
        assert_eq!(mayor(&[1.5, 3.7, 2.1]), Some(3.7));
    }

    #[test]
    fn test_chars() {
        assert_eq!(mayor(&['a', 'z', 'm']), Some('z'));
    }

    #[test]
    fn test_vacio() {
        let vacio: &[i32] = &[];
        assert_eq!(mayor(vacio), None);
    }
}
