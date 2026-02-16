// =============================================================
// Ejercicio 1: Rectangulo
// Crea un struct Rectangulo con campos ancho y alto (f64).
// Implementa area() y es_cuadrado().
// Practica: structs, impl blocks, methods
// =============================================================

pub struct Rectangulo {
    pub ancho: f64,
    pub alto: f64,
}

impl Rectangulo {
    pub fn new(ancho: f64, alto: f64) -> Self {
        Rectangulo { ancho, alto }
    }

    pub fn area(&self) -> f64 {
        self.ancho * self.alto
    }

    pub fn es_cuadrado(&self) -> bool {
        (self.ancho - self.alto).abs() < f64::EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let r = Rectangulo::new(5.0, 3.0);
        assert_eq!(r.area(), 15.0);
    }

    #[test]
    fn test_es_cuadrado_true() {
        let c = Rectangulo::new(4.0, 4.0);
        assert!(c.es_cuadrado());
    }

    #[test]
    fn test_es_cuadrado_false() {
        let r = Rectangulo::new(4.0, 5.0);
        assert!(!r.es_cuadrado());
    }
}
