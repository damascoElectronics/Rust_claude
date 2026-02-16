// =============================================================
// Ejercicio 2: Tomar Ownership
// Toma ownership de un String, le agrega " - modificado"
// al final, y lo retorna.
// Practica: move semantics y retorno de ownership
// =============================================================

pub fn tomar_y_modificar(mut texto: String) -> String {
    texto.push_str(" - modificado");
    texto
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basico() {
        let original = String::from("hola");
        let resultado = tomar_y_modificar(original);
        assert_eq!(resultado, "hola - modificado");
    }

    #[test]
    fn test_vacio() {
        let original = String::from("");
        let resultado = tomar_y_modificar(original);
        assert_eq!(resultado, " - modificado");
    }

    #[test]
    fn test_texto_largo() {
        let original = String::from("aprendiendo Rust");
        let resultado = tomar_y_modificar(original);
        assert_eq!(resultado, "aprendiendo Rust - modificado");
    }
}
