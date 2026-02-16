// =============================================================
// Ejercicio 3: Contar Signos
// Dado un slice de enteros, retorna un tuple (positivos, negativos, ceros)
// con la cantidad de cada uno.
// Practica: pattern matching con Ordering, tuples
// =============================================================

pub fn contar_signos(numeros: &[i32]) -> (usize, usize, usize) {
    let mut positivos = 0;
    let mut negativos = 0;
    let mut ceros = 0;

    for &n in numeros {
        match n.cmp(&0) {
            std::cmp::Ordering::Greater => positivos += 1,
            std::cmp::Ordering::Less => negativos += 1,
            std::cmp::Ordering::Equal => ceros += 1,
        }
    }

    (positivos, negativos, ceros)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixtos() {
        assert_eq!(contar_signos(&[1, -2, 3, 0, -4, 5]), (3, 2, 1));
    }

    #[test]
    fn test_todos_positivos() {
        assert_eq!(contar_signos(&[1, 2, 3]), (3, 0, 0));
    }

    #[test]
    fn test_vacio() {
        assert_eq!(contar_signos(&[]), (0, 0, 0));
    }
}
