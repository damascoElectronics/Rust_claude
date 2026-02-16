// =============================================================
// Ejercicio 1: Etapa de Vida
// Usa match con ranges para clasificar una edad:
// 0..=12 -> "Nino", 13..=17 -> "Adolescente",
// 18..=64 -> "Adulto", 65+ -> "Adulto mayor"
// Practica: match con ranges
// =============================================================

pub fn etapa_de_vida(edad: u32) -> &'static str {
    match edad {
        0..=12 => "Nino",
        13..=17 => "Adolescente",
        18..=64 => "Adulto",
        _ => "Adulto mayor",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nino() {
        assert_eq!(etapa_de_vida(5), "Nino");
        assert_eq!(etapa_de_vida(0), "Nino");
        assert_eq!(etapa_de_vida(12), "Nino");
    }

    #[test]
    fn test_adolescente() {
        assert_eq!(etapa_de_vida(13), "Adolescente");
        assert_eq!(etapa_de_vida(17), "Adolescente");
    }

    #[test]
    fn test_adulto() {
        assert_eq!(etapa_de_vida(18), "Adulto");
        assert_eq!(etapa_de_vida(30), "Adulto");
        assert_eq!(etapa_de_vida(64), "Adulto");
    }

    #[test]
    fn test_adulto_mayor() {
        assert_eq!(etapa_de_vida(65), "Adulto mayor");
        assert_eq!(etapa_de_vida(90), "Adulto mayor");
    }
}
