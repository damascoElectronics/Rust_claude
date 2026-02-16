// =============================================================
// Ejercicio 7: if let y while let
// Atajos para pattern matching cuando solo te importa un caso:
// - if let: match de un solo patron
// - while let: loop mientras el patron coincida
// Practica: if let, while let, Option matching
// =============================================================

/// Usa if let para extraer el valor de un Option.
/// Si tiene valor, retorna "Valor: X", sino "Sin valor".
pub fn describir_option(opt: Option<i32>) -> String {
    todo!("tu código va aquí")
}

/// Usa while let para sacar elementos de un vector (como stack con pop)
/// hasta que se vacie, y retorna la suma de todos.
pub fn sumar_stack(mut stack: Vec<i32>) -> i32 {
    todo!("tu código va aquí")
}

/// Usa if let para procesar solo variantes especificas de un enum
pub enum Accion {
    Imprimir(String),
    Sumar(i32, i32),
    Nada,
}

pub fn ejecutar_accion(accion: &Accion) -> String {
    todo!("tu código va aquí")
}

/// Usa while let para parsear numeros de un iterador hasta encontrar uno invalido
pub fn parsear_hasta_error(textos: &[&str]) -> Vec<i32> {
    todo!("tu código va aquí")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describir_option_some() {
        assert_eq!(describir_option(Some(42)), "Valor: 42");
    }

    #[test]
    fn test_describir_option_none() {
        assert_eq!(describir_option(None), "Sin valor");
    }

    #[test]
    fn test_sumar_stack() {
        assert_eq!(sumar_stack(vec![1, 2, 3, 4]), 10);
        assert_eq!(sumar_stack(vec![]), 0);
    }

    #[test]
    fn test_accion_imprimir() {
        let a = Accion::Imprimir(String::from("hola"));
        assert_eq!(ejecutar_accion(&a), "Imprimiendo: hola");
    }

    #[test]
    fn test_accion_sumar() {
        let a = Accion::Sumar(3, 7);
        assert_eq!(ejecutar_accion(&a), "Resultado: 10");
    }

    #[test]
    fn test_accion_nada() {
        assert_eq!(ejecutar_accion(&Accion::Nada), "Sin accion");
    }

    #[test]
    fn test_parsear_hasta_error() {
        assert_eq!(parsear_hasta_error(&["1", "2", "3", "abc", "5"]), vec![1, 2, 3]);
        assert_eq!(parsear_hasta_error(&["abc"]), Vec::<i32>::new());
        assert_eq!(parsear_hasta_error(&["10", "20"]), vec![10, 20]);
    }
}
