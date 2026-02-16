// =============================================================
// Ejercicio 6: Enums con Datos Asociados
// Practica:
// - Enums con diferentes tipos de datos en cada variante
// - Destructuring de enums con match
// - Enums como mini sistema de tipos
// =============================================================

/// Cada variante tiene datos diferentes
pub enum Forma {
    Circulo(f64),                    // radio
    Rectangulo(f64, f64),            // ancho, alto
    Triangulo { base: f64, altura: f64 }, // campos nombrados
}

impl Forma {
    pub fn area(&self) -> f64 {
        todo!("tu código va aquí")
    }

    pub fn nombre(&self) -> &str {
        todo!("tu código va aquí")
    }
}

/// Enum que representa un mensaje en un sistema
pub enum Mensaje {
    Salir,                           // sin datos (unit variant)
    Texto(String),                   // con un String
    Mover { x: i32, y: i32 },       // con campos nombrados
    Color(u8, u8, u8),              // con tuple de datos
}

impl Mensaje {
    pub fn describir(&self) -> String {
        todo!("tu código va aquí")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_circulo() {
        let c = Forma::Circulo(1.0);
        assert!((c.area() - std::f64::consts::PI).abs() < 0.0001);
    }

    #[test]
    fn test_area_rectangulo() {
        let r = Forma::Rectangulo(5.0, 3.0);
        assert_eq!(r.area(), 15.0);
    }

    #[test]
    fn test_area_triangulo() {
        let t = Forma::Triangulo { base: 10.0, altura: 4.0 };
        assert_eq!(t.area(), 20.0);
    }

    #[test]
    fn test_nombre_forma() {
        assert_eq!(Forma::Circulo(1.0).nombre(), "circulo");
        assert_eq!(Forma::Rectangulo(1.0, 1.0).nombre(), "rectangulo");
        assert_eq!(Forma::Triangulo { base: 1.0, altura: 1.0 }.nombre(), "triangulo");
    }

    #[test]
    fn test_mensaje_salir() {
        assert_eq!(Mensaje::Salir.describir(), "Comando: salir");
    }

    #[test]
    fn test_mensaje_texto() {
        let m = Mensaje::Texto(String::from("hola"));
        assert_eq!(m.describir(), "Texto: hola");
    }

    #[test]
    fn test_mensaje_mover() {
        let m = Mensaje::Mover { x: 10, y: 20 };
        assert_eq!(m.describir(), "Mover a (10, 20)");
    }

    #[test]
    fn test_mensaje_color() {
        let m = Mensaje::Color(255, 128, 0);
        assert_eq!(m.describir(), "Color: rgb(255, 128, 0)");
    }
}
