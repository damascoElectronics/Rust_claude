// =============================================================
// Ejercicio 5: Tuplas y Arrays
// Practica con compound types:
// - Tuples: crear, destructuring, acceso por indice, retornar desde funciones
// - Arrays: crear, acceso por indice, [valor; cantidad]
// Practica: tuples, arrays, destructuring, function return types
// =============================================================

/// Retorna una tupla (min, max) de dos numeros
pub fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

/// Recibe una tupla (nombre, edad) y retorna un string formateado
pub fn presentar(persona: (&str, u32)) -> String {
    let (nombre, edad) = persona;
    format!("{} tiene {} anios", nombre, edad)
}

/// Retorna la suma de todos los elementos de un array de 5 elementos
pub fn suma_array(numeros: [i32; 5]) -> i32 {
    let mut suma = 0;
    for &n in numeros.iter() {
        suma += n;
    }
    suma
}

/// Crea y retorna un array de tamanio 5 donde todos los elementos son `valor`
pub fn crear_array_repetido(valor: i32) -> [i32; 5] {
    [valor; 5]
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
    fn test_presentar() {
        assert_eq!(presentar(("Ana", 25)), "Ana tiene 25 anios");
        assert_eq!(presentar(("Rust", 0)), "Rust tiene 0 anios");
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
