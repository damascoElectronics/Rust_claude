// =============================================================
// Capitulo 3: Ownership y Borrowing - Ejercicios
// =============================================================

// --- Ejercicio 1 ---
// Recibe una referencia a String y retorna true si contiene "rust" (case insensitive)
pub fn contiene_rust(texto: &String) -> bool {
    texto.to_lowercase().contains("rust")
}

// --- Ejercicio 2 ---
// Toma ownership de un String, le agrega " - modificado" al final, y lo retorna
pub fn tomar_y_modificar(mut texto: String) -> String {
    texto.push_str(" - modificado");
    texto
}

// --- Ejercicio 3 ---
// Recibe un slice de enteros por referencia y retorna el mayor valor.
// Si el slice esta vacio retorna None.
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

    // --- Tests Ejercicio 1: contiene_rust ---

    #[test]
    fn test_contiene_rust_minusculas() {
        let texto = String::from("me gusta rust");
        assert!(contiene_rust(&texto), "deberia encontrar 'rust' en minusculas");
    }

    #[test]
    fn test_contiene_rust_mayusculas() {
        let texto = String::from("Aprendiendo RUST hoy");
        assert!(contiene_rust(&texto), "deberia encontrar 'RUST' en mayusculas");
    }

    #[test]
    fn test_no_contiene_rust() {
        let texto = String::from("hola mundo");
        assert!(!contiene_rust(&texto), "no deberia encontrar 'rust'");
    }

    #[test]
    fn test_contiene_rust_mixto() {
        let texto = String::from("RuSt es genial");
        assert!(contiene_rust(&texto), "deberia encontrar 'RuSt' mixto");
    }

    // --- Tests Ejercicio 2: tomar_y_modificar ---

    #[test]
    fn test_tomar_y_modificar_basico() {
        let original = String::from("hola");
        let resultado = tomar_y_modificar(original);
        assert_eq!(resultado, "hola - modificado");
    }

    #[test]
    fn test_tomar_y_modificar_vacio() {
        let original = String::from("");
        let resultado = tomar_y_modificar(original);
        assert_eq!(resultado, " - modificado");
    }

    #[test]
    fn test_tomar_y_modificar_con_texto_largo() {
        let original = String::from("aprendiendo Rust");
        let resultado = tomar_y_modificar(original);
        assert_eq!(resultado, "aprendiendo Rust - modificado");
    }

    // --- Tests Ejercicio 3: encontrar_mayor ---

    #[test]
    fn test_mayor_basico() {
        assert_eq!(encontrar_mayor(&[1, 5, 3, 9, 2]), Some(9));
    }

    #[test]
    fn test_mayor_negativos() {
        assert_eq!(encontrar_mayor(&[-5, -1, -10, -3]), Some(-1));
    }

    #[test]
    fn test_mayor_vacio() {
        let vacio: &[i32] = &[];
        assert_eq!(encontrar_mayor(vacio), None);
    }

    #[test]
    fn test_mayor_un_elemento() {
        assert_eq!(encontrar_mayor(&[42]), Some(42));
    }
}
