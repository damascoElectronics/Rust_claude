// =============================================================
// Ejercicio 6: Referencias Mutables
// Practica con &mut T:
// - Solo una referencia mutable a la vez
// - No mezclar &T con &mut T al mismo tiempo
// - Modificar datos a traves de referencias
// Practica: &mut T, reglas de borrowing, modificacion in-place
// =============================================================

/// Duplica el valor de un entero a traves de una referencia mutable
pub fn duplicar(n: &mut i32) {
    *n *= 2;
}

/// Agrega un elemento al final de un vector via referencia mutable
/// y retorna el nuevo largo
pub fn agregar_y_contar(vec: &mut Vec<i32>, valor: i32) -> usize {
    vec.push(valor);
    vec.len()
}

/// Pone en mayusculas la primera letra de un String via &mut String
pub fn capitalizar(texto: &mut String) {
    if let Some(primera) = texto.chars().next() {
        let resto = texto[primera.len_utf8()..].to_string();
        *texto = primera.to_uppercase().to_string() + &resto;
    }
}

/// Reemplaza todos los negativos en un slice con cero
pub fn limpiar_negativos(numeros: &mut [i32]) {
    for n in numeros.iter_mut() {
        if *n < 0 {
            *n = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicar() {
        let mut x = 5;
        duplicar(&mut x);
        assert_eq!(x, 10);
    }

    #[test]
    fn test_agregar_y_contar() {
        let mut v = vec![1, 2, 3];
        let len = agregar_y_contar(&mut v, 4);
        assert_eq!(len, 4);
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_capitalizar() {
        let mut s = String::from("hola");
        capitalizar(&mut s);
        assert_eq!(s, "Hola");
    }

    #[test]
    fn test_capitalizar_vacio() {
        let mut s = String::from("");
        capitalizar(&mut s);
        assert_eq!(s, "");
    }

    #[test]
    fn test_limpiar_negativos() {
        let mut nums = vec![1, -2, 3, -4, 5];
        limpiar_negativos(&mut nums);
        assert_eq!(nums, vec![1, 0, 3, 0, 5]);
    }

    #[test]
    fn test_limpiar_negativos_sin_negativos() {
        let mut nums = vec![1, 2, 3];
        limpiar_negativos(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }
}
