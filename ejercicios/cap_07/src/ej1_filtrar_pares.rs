// =============================================================
// Ejercicio 1: Filtrar Pares
// Recibe un Vec<i32> y retorna un nuevo Vec solo con los
// numeros pares, ordenados de menor a mayor.
// Practica: Vec, iteradores, filter, collect, sort
// =============================================================

pub fn filtrar_pares(numeros: Vec<i32>) -> Vec<i32> {
    let mut pares: Vec<i32> = numeros.into_iter().filter(|n| n % 2 == 0).collect();
    pares.sort();
    pares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixtos() {
        assert_eq!(filtrar_pares(vec![5, 2, 8, 1, 4, 3]), vec![2, 4, 8]);
    }

    #[test]
    fn test_sin_pares() {
        assert_eq!(filtrar_pares(vec![1, 3, 5]), Vec::<i32>::new());
    }

    #[test]
    fn test_vacio() {
        assert_eq!(filtrar_pares(vec![]), Vec::<i32>::new());
    }
}
