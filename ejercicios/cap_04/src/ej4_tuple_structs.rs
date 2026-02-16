// =============================================================
// Ejercicio 4: Tuple Structs y Struct Update Syntax
// Practica:
// - Tuple structs: campos sin nombre (Color(u8, u8, u8))
// - Unit-like structs: sin campos (struct Vacio;)
// - Struct update syntax (..)
// =============================================================

/// Tuple struct para representar un color RGB
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn rojo() -> Self {
        todo!("tu código va aquí")
    }

    pub fn verde() -> Self {
        todo!("tu código va aquí")
    }

    pub fn azul() -> Self {
        todo!("tu código va aquí")
    }

    /// Retorna el color como string hex "#RRGGBB"
    pub fn a_hex(&self) -> String {
        todo!("tu código va aquí")
    }

    /// Retorna true si es un tono de gris (R == G == B)
    pub fn es_gris(&self) -> bool {
        todo!("tu código va aquí")
    }
}

/// Struct normal para demostrar update syntax
pub struct Config {
    pub ancho: u32,
    pub alto: u32,
    pub titulo: String,
    pub fullscreen: bool,
}

impl Config {
    /// Crea una config por defecto
    pub fn default() -> Self {
        todo!("tu código va aquí")
    }

    /// Crea una config nueva usando update syntax (..) desde otra config
    pub fn con_titulo(base: Config, titulo: &str) -> Self {
        todo!("tu código va aquí")
    }

    pub fn descripcion(&self) -> String {
        todo!("tu código va aquí")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_hex() {
        assert_eq!(Color::rojo().a_hex(), "#FF0000");
        assert_eq!(Color::verde().a_hex(), "#00FF00");
        assert_eq!(Color::azul().a_hex(), "#0000FF");
        assert_eq!(Color(128, 128, 128).a_hex(), "#808080");
    }

    #[test]
    fn test_color_es_gris() {
        assert!(Color(100, 100, 100).es_gris());
        assert!(!Color(100, 200, 100).es_gris());
    }

    #[test]
    fn test_config_default() {
        let c = Config::default();
        assert_eq!(c.ancho, 800);
        assert_eq!(c.alto, 600);
        assert_eq!(c.titulo, "Mi App");
        assert!(!c.fullscreen);
    }

    #[test]
    fn test_config_update_syntax() {
        let base = Config::default();
        let nueva = Config::con_titulo(base, "Rust App");
        assert_eq!(nueva.titulo, "Rust App");
        assert_eq!(nueva.ancho, 800); // viene de base via ..
        assert_eq!(nueva.alto, 600);  // viene de base via ..
    }

    #[test]
    fn test_config_descripcion() {
        let c = Config::default();
        assert_eq!(c.descripcion(), "800x600 'Mi App' fullscreen=false");
    }
}
