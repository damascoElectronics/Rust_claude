// =============================================================
// Ejercicio 3: Mas Largo
// Recibe un slice de Strings y retorna el mas largo.
// Si hay empate, retorna el primero. Si esta vacio retorna None.
// Practica: slices de String, Option, references
// =============================================================

pub fn mas_largo(textos: &[String]) -> Option<&String> {
    if textos.is_empty() {
        return None;
    }
    let mut mayor = &textos[0];
    for texto in &textos[1..] {
        if texto.len() > mayor.len() {
            mayor = texto;
        }
    }
    Some(mayor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basico() {
        let textos = vec![
            String::from("hi"),
            String::from("hola"),
            String::from("hey"),
        ];
        assert_eq!(mas_largo(&textos), Some(&String::from("hola")));
    }

    #[test]
    fn test_empate() {
        let textos = vec![String::from("abc"), String::from("xyz")];
        assert_eq!(mas_largo(&textos), Some(&String::from("abc")));
    }

    #[test]
    fn test_vacio() {
        let textos: Vec<String> = vec![];
        assert_eq!(mas_largo(&textos), None);
    }
}
