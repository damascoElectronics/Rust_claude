// =============================================================
// Ejercicio 5: Tuplas y Arrays
// Practica con compound types:
// - Tuples: crear, destructuring, acceso por indice, retornar desde funciones
// - Arrays: crear, acceso por indice, [valor; cantidad]
// Practica: tuples, arrays, destructuring, function return types
// =============================================================

/// Retorna una tupla (min, max) de dos numeros
/// Pista: compara a y b con if, retorna (menor, mayor)
pub fn min_max(a: i32, b: i32) -> (i32, i32) {
    todo!("tu código va aquí")
}

/// Recibe una tupla (x, y) de un punto 2D y retorna la distancia
/// al origen usando el acceso por indice: punto.0, punto.1
/// Distancia = (x*x + y*y) como f64, luego .sqrt()
/// Pista: let x = punto.0 as f64;
pub fn distancia_al_origen(punto: (i32, i32)) -> f64 {
    todo!("tu código va aquí")
}

/// Recibe una tupla de 3 elementos y usa destructuring
/// para retornarlos en orden inverso.
/// Ejemplo: (1, 2, 3) -> (3, 2, 1)
/// Pista: let (a, b, c) = tupla;
pub fn invertir_tupla(tupla: (i32, i32, i32)) -> (i32, i32, i32) {
    todo!("tu código va aquí")
}

/// Retorna la suma de todos los elementos de un array de 5 elementos
/// Pista: usa un for loop para iterar sobre numeros.iter()
pub fn suma_array(numeros: [i32; 5]) -> i32 {
    todo!("tu código va aquí")
}

/// Crea y retorna un array de tamanio 5 donde todos los elementos son `valor`
/// Pista: usa la sintaxis [valor; 5]
pub fn crear_array_repetido(valor: i32) -> [i32; 5] {
    todo!("tu código va aquí")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_max() {
        assert_eq!(min_max(5, 3), (3, 5));
        assert_eq!(min_max(1, 1), (1, 1));
        assert_eq!(min_max(-2, 10), (-2, 10));
    }

    #[test]
    fn test_distancia_al_origen() {
        let d = distancia_al_origen((3, 4));
        assert!((d - 5.0).abs() < 0.0001); // triangulo 3-4-5
        assert!((distancia_al_origen((0, 0))).abs() < 0.0001);
    }

    #[test]
    fn test_invertir_tupla() {
        assert_eq!(invertir_tupla((1, 2, 3)), (3, 2, 1));
        assert_eq!(invertir_tupla((10, 20, 30)), (30, 20, 10));
        assert_eq!(invertir_tupla((5, 5, 5)), (5, 5, 5));
    }

    #[test]
    fn test_suma_array() {
        assert_eq!(suma_array([1, 2, 3, 4, 5]), 15);
        assert_eq!(suma_array([0, 0, 0, 0, 0]), 0);
        assert_eq!(suma_array([-1, -2, -3, -4, -5]), -15);
    }

    #[test]
    fn test_crear_array_repetido() {
        assert_eq!(crear_array_repetido(7), [7, 7, 7, 7, 7]);
        assert_eq!(crear_array_repetido(0), [0, 0, 0, 0, 0]);
    }
}
