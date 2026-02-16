// =============================================================
// Ejercicio 4: Operaciones con Vec
// Practica con metodos de Vec:
// - push(), pop(), len(), is_empty()
// - get() vs [] para acceso seguro
// - Iteracion mutable (modificar elementos in-place)
// Practica: Vec API, get() -> Option, iter_mut, dereference *
// =============================================================

/// Simula una pila (stack) usando Vec: push varios valores, pop uno,
/// retorna (elementos_restantes, elemento_sacado)
pub fn operaciones_stack(valores: &[i32]) -> (Vec<i32>, Option<i32>) {
    todo!("tu código va aquí")
}

/// Acceso seguro con get(): retorna el elemento en `indice`
/// o "fuera de rango" si no existe
pub fn acceso_seguro(vec: &[i32], indice: usize) -> String {
    todo!("tu código va aquí")
}

/// Multiplica cada elemento del vector por 2 usando iteracion mutable
pub fn duplicar_elementos(vec: &mut Vec<i32>) {
    todo!("tu código va aquí")
}

/// Retorna un nuevo Vec con solo los elementos unicos (sin duplicados),
/// manteniendo el orden de primera aparicion
pub fn unicos(vec: &[i32]) -> Vec<i32> {
    todo!("tu código va aquí")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operaciones_stack() {
        let (rest, sacado) = operaciones_stack(&[1, 2, 3]);
        assert_eq!(rest, vec![1, 2]);
        assert_eq!(sacado, Some(3));
    }

    #[test]
    fn test_operaciones_stack_vacio() {
        let (rest, sacado) = operaciones_stack(&[]);
        assert!(rest.is_empty());
        assert_eq!(sacado, None);
    }

    #[test]
    fn test_acceso_seguro_valido() {
        assert_eq!(acceso_seguro(&[10, 20, 30], 1), "Encontrado: 20");
    }

    #[test]
    fn test_acceso_seguro_invalido() {
        assert_eq!(acceso_seguro(&[10, 20, 30], 99), "fuera de rango");
    }

    #[test]
    fn test_duplicar_elementos() {
        let mut v = vec![1, 2, 3, 4];
        duplicar_elementos(&mut v);
        assert_eq!(v, vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_unicos() {
        assert_eq!(unicos(&[1, 2, 2, 3, 1, 4, 3]), vec![1, 2, 3, 4]);
        assert_eq!(unicos(&[5, 5, 5]), vec![5]);
    }
}
