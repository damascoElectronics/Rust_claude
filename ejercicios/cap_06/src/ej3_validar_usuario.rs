// =============================================================
// Ejercicio 3: Validar Usuario
// Valida un nombre de usuario con multiples reglas:
// - No puede estar vacio -> Err("nombre vacio")
// - Al menos 3 caracteres -> Err("nombre muy corto")
// - Sin espacios -> Err("nombre con espacios")
// - Si es valido -> Ok(nombre en minusculas)
// Practica: multiples paths de error, early return
// =============================================================

pub fn validar_usuario(nombre: &str) -> Result<String, &'static str> {
    if nombre.is_empty() {
        return Err("nombre vacio");
    }
    if nombre.len() < 3 {
        return Err("nombre muy corto");
    }
    if nombre.contains(' ') {
        return Err("nombre con espacios");
    }
    Ok(nombre.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valido() {
        assert_eq!(validar_usuario("RustFan"), Ok(String::from("rustfan")));
    }

    #[test]
    fn test_vacio() {
        assert_eq!(validar_usuario(""), Err("nombre vacio"));
    }

    #[test]
    fn test_corto() {
        assert_eq!(validar_usuario("ab"), Err("nombre muy corto"));
    }

    #[test]
    fn test_con_espacios() {
        assert_eq!(validar_usuario("rust fan"), Err("nombre con espacios"));
    }
}
