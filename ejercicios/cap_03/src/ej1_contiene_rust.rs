// =============================================================
// Ejercicio 1: Contiene Rust
// Recibe una referencia a String (&String) y retorna true
// si contiene "rust" (case insensitive).
// Practica: borrowing con &String
// =============================================================

pub fn contiene_rust(texto: &String) -> bool {
    texto.to_lowercase().contains("rust")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minusculas() {
        let texto = String::from("me gusta rust");
        assert!(contiene_rust(&texto));
    }

    #[test]
    fn test_mayusculas() {
        let texto = String::from("Aprendiendo RUST hoy");
        assert!(contiene_rust(&texto));
    }

    #[test]
    fn test_no_contiene() {
        let texto = String::from("hola mundo");
        assert!(!contiene_rust(&texto));
    }

    #[test]
    fn test_mixto() {
        let texto = String::from("RuSt es genial");
        assert!(contiene_rust(&texto));
    }
}
