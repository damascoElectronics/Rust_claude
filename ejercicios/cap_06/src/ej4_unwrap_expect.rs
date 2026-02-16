// =============================================================
// Ejercicio 4: unwrap, expect y metodos de Result/Option
// Practica con los metodos shortcut:
// - unwrap(): extrae el valor o hace panic
// - expect(): como unwrap pero con mensaje custom
// - unwrap_or(): valor por defecto si es Err/None
// - unwrap_or_else(): closure por defecto
// - is_ok(), is_err(), is_some(), is_none()
// =============================================================

/// Usa unwrap_or para parsear un string a i32.
/// Si falla el parseo, retorna el valor por defecto dado.
pub fn parsear_o_default(texto: &str, default: i32) -> i32 {
    texto.parse::<i32>().unwrap_or(default)
}

/// Usa unwrap_or_else para obtener un valor de un Option.
/// Si es None, calcula el valor por defecto con una closure.
pub fn obtener_o_calcular(opt: Option<i32>, base: i32) -> i32 {
    opt.unwrap_or_else(|| base * 2)
}

/// Usa is_ok/is_err para contar cuantos strings son numeros validos
pub fn contar_numeros_validos(textos: &[&str]) -> (usize, usize) {
    let mut validos = 0;
    let mut invalidos = 0;
    for texto in textos {
        if texto.parse::<i32>().is_ok() {
            validos += 1;
        } else {
            invalidos += 1;
        }
    }
    (validos, invalidos)
}

/// Usa map() en Option para transformar el valor interno sin sacarlo.
/// Retorna el largo del string si existe, o None.
pub fn largo_opcional(texto: Option<&str>) -> Option<usize> {
    texto.map(|t| t.len())
}

/// Usa and_then() para encadenar operaciones que pueden fallar.
/// Parsea un string a i32, y si es positivo retorna Some(n), sino None.
pub fn parsear_positivo(texto: &str) -> Option<i32> {
    texto.parse::<i32>().ok().and_then(|n| {
        if n > 0 { Some(n) } else { None }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsear_o_default_valido() {
        assert_eq!(parsear_o_default("42", 0), 42);
    }

    #[test]
    fn test_parsear_o_default_invalido() {
        assert_eq!(parsear_o_default("abc", 99), 99);
    }

    #[test]
    fn test_obtener_o_calcular_some() {
        assert_eq!(obtener_o_calcular(Some(10), 5), 10);
    }

    #[test]
    fn test_obtener_o_calcular_none() {
        assert_eq!(obtener_o_calcular(None, 5), 10); // 5 * 2
    }

    #[test]
    fn test_contar_numeros_validos() {
        assert_eq!(contar_numeros_validos(&["1", "abc", "3", "xyz"]), (2, 2));
        assert_eq!(contar_numeros_validos(&["1", "2", "3"]), (3, 0));
    }

    #[test]
    fn test_largo_opcional() {
        assert_eq!(largo_opcional(Some("hola")), Some(4));
        assert_eq!(largo_opcional(None), None);
    }

    #[test]
    fn test_parsear_positivo() {
        assert_eq!(parsear_positivo("42"), Some(42));
        assert_eq!(parsear_positivo("-5"), None);
        assert_eq!(parsear_positivo("abc"), None);
        assert_eq!(parsear_positivo("0"), None);
    }
}
