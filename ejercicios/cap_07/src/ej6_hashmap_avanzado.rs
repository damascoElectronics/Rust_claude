// =============================================================
// Ejercicio 6: HashMap Avanzado
// Practica avanzada con HashMap:
// - get() retornando Option<&V>
// - Iteracion sobre keys y values
// - Combinar multiples colecciones
// - Operaciones de agrupacion
// Practica: HashMap iteration, get(), collect, grouping
// =============================================================

use std::collections::HashMap;

/// Dadas dos listas (keys y values), construye un HashMap.
/// Si tienen diferente largo, usa solo hasta el minimo.
pub fn zip_a_hashmap(keys: &[&str], values: &[i32]) -> HashMap<String, i32> {
    todo!("tu código va aquí")
}

/// Retorna el valor asociado a una key, o un mensaje de error
pub fn buscar_valor(mapa: &HashMap<String, i32>, key: &str) -> String {
    todo!("tu código va aquí")
}

/// Agrupa numeros por su resto al dividir entre `divisor`.
/// Retorna HashMap<i32, Vec<i32>> donde la key es el resto.
pub fn agrupar_por_resto(numeros: &[i32], divisor: i32) -> HashMap<i32, Vec<i32>> {
    todo!("tu código va aquí")
}

/// Itera sobre un HashMap y retorna la key con el mayor valor
pub fn key_del_mayor(mapa: &HashMap<String, i32>) -> Option<String> {
    todo!("tu código va aquí")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zip_a_hashmap() {
        let mapa = zip_a_hashmap(&["a", "b", "c"], &[1, 2, 3]);
        assert_eq!(mapa.get("a"), Some(&1));
        assert_eq!(mapa.get("b"), Some(&2));
        assert_eq!(mapa.get("c"), Some(&3));
        assert_eq!(mapa.len(), 3);
    }

    #[test]
    fn test_zip_diferente_largo() {
        let mapa = zip_a_hashmap(&["x", "y"], &[10, 20, 30]);
        assert_eq!(mapa.len(), 2); // solo usa 2
    }

    #[test]
    fn test_buscar_valor_existe() {
        let mut mapa = HashMap::new();
        mapa.insert(String::from("rust"), 100);
        assert_eq!(buscar_valor(&mapa, "rust"), "rust: 100");
    }

    #[test]
    fn test_buscar_valor_no_existe() {
        let mapa: HashMap<String, i32> = HashMap::new();
        assert_eq!(buscar_valor(&mapa, "rust"), "rust no encontrado");
    }

    #[test]
    fn test_agrupar_por_resto() {
        let grupos = agrupar_por_resto(&[1, 2, 3, 4, 5, 6], 3);
        let mut g0 = grupos.get(&0).unwrap().clone();
        g0.sort();
        assert_eq!(g0, vec![3, 6]);
        let mut g1 = grupos.get(&1).unwrap().clone();
        g1.sort();
        assert_eq!(g1, vec![1, 4]);
    }

    #[test]
    fn test_key_del_mayor() {
        let mut mapa = HashMap::new();
        mapa.insert(String::from("a"), 10);
        mapa.insert(String::from("b"), 50);
        mapa.insert(String::from("c"), 30);
        assert_eq!(key_del_mayor(&mapa), Some(String::from("b")));
    }

    #[test]
    fn test_key_del_mayor_vacio() {
        let mapa: HashMap<String, i32> = HashMap::new();
        assert_eq!(key_del_mayor(&mapa), None);
    }
}
