// =============================================================
// Ejercicio 5: Traits de la Standard Library
// Practica implementando traits comunes:
// - Debug: para imprimir con {:?}
// - Clone + Copy: para tipos copiables
// - PartialEq: para comparar con ==
// - Default: valores por defecto
// - Display: formato personalizado con {}
// =============================================================

use std::fmt;

/// Struct que implementa multiples traits estandar
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Punto {
    pub x: f64,
    pub y: f64,
}

impl Punto {
    pub fn new(x: f64, y: f64) -> Self {
        Punto { x, y }
    }

    pub fn distancia_al_origen(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Default for Punto {
    fn default() -> Self {
        Punto { x: 0.0, y: 0.0 }
    }
}

impl fmt::Display for Punto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Struct con Default para configuracion
#[derive(Debug, PartialEq)]
pub struct Configuracion {
    pub intentos: u32,
    pub timeout_ms: u64,
    pub verbose: bool,
}

impl Default for Configuracion {
    fn default() -> Self {
        Configuracion {
            intentos: 3,
            timeout_ms: 5000,
            verbose: false,
        }
    }
}

/// Funcion que acepta cualquier tipo Debug + Display
pub fn log_valor<T: fmt::Debug + fmt::Display>(valor: &T) -> String {
    format!("Display: {} | Debug: {:?}", valor, valor)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Tests: Copy ---
    #[test]
    fn test_punto_copy() {
        let p1 = Punto::new(3.0, 4.0);
        let p2 = p1; // Copy, no move
        assert_eq!(p1, p2); // p1 sigue valido
    }

    // --- Tests: Clone ---
    #[test]
    fn test_punto_clone() {
        let p1 = Punto::new(1.0, 2.0);
        let p2 = p1.clone();
        assert_eq!(p1, p2);
    }

    // --- Tests: PartialEq ---
    #[test]
    fn test_punto_igualdad() {
        assert_eq!(Punto::new(1.0, 2.0), Punto::new(1.0, 2.0));
        assert_ne!(Punto::new(1.0, 2.0), Punto::new(3.0, 4.0));
    }

    // --- Tests: Default ---
    #[test]
    fn test_punto_default() {
        let p = Punto::default();
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
    }

    #[test]
    fn test_configuracion_default() {
        let c = Configuracion::default();
        assert_eq!(c.intentos, 3);
        assert_eq!(c.timeout_ms, 5000);
        assert!(!c.verbose);
    }

    // --- Tests: Display ---
    #[test]
    fn test_punto_display() {
        let p = Punto::new(3.5, 4.5);
        assert_eq!(format!("{}", p), "(3.5, 4.5)");
    }

    // --- Tests: Debug ---
    #[test]
    fn test_punto_debug() {
        let p = Punto::new(1.0, 2.0);
        let debug_str = format!("{:?}", p);
        assert!(debug_str.contains("Punto"));
        assert!(debug_str.contains("1.0"));
    }

    // --- Tests: distancia ---
    #[test]
    fn test_distancia_al_origen() {
        let p = Punto::new(3.0, 4.0);
        assert!((p.distancia_al_origen() - 5.0).abs() < 0.0001);
    }

    // --- Tests: log_valor ---
    #[test]
    fn test_log_valor() {
        let p = Punto::new(1.0, 2.0);
        let log = log_valor(&p);
        assert!(log.contains("Display: (1, 2)"));
        assert!(log.contains("Debug: Punto"));
    }
}
