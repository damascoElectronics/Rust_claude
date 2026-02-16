// =============================================================
// Ejercicio 3: Encontrar Mayor
// Recibe un slice de enteros por referencia (&[i32]) y retorna
// el mayor valor como Option<i32>. None si esta vacio.
// Practica: borrowing con slices
// =============================================================

pub fn encontrar_mayor(numeros: &[i32]) -> Option<i32> {
    if numeros.is_empty() {
        return None;
    }
    let mut mayor = numeros[0];
    for &n in &numeros[1..] {
        if n > mayor {
            mayor = n;
        }
    }
    Some(mayor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basico() {
        assert_eq!(encontrar_mayor(&[1, 5, 3, 9, 2]), Some(9));
    }

    #[test]
    fn test_negativos() {
        assert_eq!(encontrar_mayor(&[-5, -1, -10, -3]), Some(-1));
    }

    #[test]
    fn test_vacio() {
        let vacio: &[i32] = &[];
        assert_eq!(encontrar_mayor(vacio), None);
    }

    #[test]
    fn test_un_elemento() {
        assert_eq!(encontrar_mayor(&[42]), Some(42));
    }
}
