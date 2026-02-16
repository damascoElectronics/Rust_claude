// =============================================================
// Capitulo 4: Structs y Enums - Ejercicios
// =============================================================

// --- Ejercicio 1 ---
// Crea un struct Rectangulo con campos ancho y alto (f64).
// Implementa un metodo area() que retorne el area
// y un metodo es_cuadrado() que retorne true si ancho == alto.
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

// --- Ejercicio 2 ---
// Crea un enum Moneda con variantes: Peso, Dolar, Euro.
// Implementa un metodo a_pesos() que convierta a pesos argentinos (aprox):
// Peso -> 1.0, Dolar -> 1000.0, Euro -> 1100.0
pub enum Moneda {
    Peso,
    Dolar,
    Euro,
}

impl Moneda {
    pub fn a_pesos(&self, cantidad: f64) -> f64 {
        match self {
            Moneda::Peso => cantidad * 1.0,
            Moneda::Dolar => cantidad * 1000.0,
            Moneda::Euro => cantidad * 1100.0,
        }
    }
}

// --- Ejercicio 3 ---
// Crea un struct Estudiante con nombre (String) y notas (Vec<f64>).
// Implementa promedio() que retorne Option<f64> (None si no tiene notas)
// y estado() que retorne "Aprobado" si promedio >= 6.0, sino "Desaprobado".
pub struct Estudiante {
    pub nombre: String,
    pub notas: Vec<f64>,
}

impl Estudiante {
    pub fn new(nombre: &str, notas: Vec<f64>) -> Self {
        Estudiante {
            nombre: String::from(nombre),
            notas,
        }
    }

    pub fn promedio(&self) -> Option<f64> {
        if self.notas.is_empty() {
            return None;
        }
        let suma: f64 = self.notas.iter().sum();
        Some(suma / self.notas.len() as f64)
    }

    pub fn estado(&self) -> &str {
        match self.promedio() {
            Some(p) if p >= 6.0 => "Aprobado",
            Some(_) => "Desaprobado",
            None => "Sin notas",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Tests Ejercicio 1: Rectangulo ---

    #[test]
    fn test_area_rectangulo() {
        let r = Rectangulo::new(5.0, 3.0);
        assert_eq!(r.area(), 15.0);
    }

    #[test]
    fn test_es_cuadrado_true() {
        let c = Rectangulo::new(4.0, 4.0);
        assert!(c.es_cuadrado(), "4x4 deberia ser cuadrado");
    }

    #[test]
    fn test_es_cuadrado_false() {
        let r = Rectangulo::new(4.0, 5.0);
        assert!(!r.es_cuadrado(), "4x5 no deberia ser cuadrado");
    }

    // --- Tests Ejercicio 2: Moneda ---

    #[test]
    fn test_pesos_a_pesos() {
        let m = Moneda::Peso;
        assert_eq!(m.a_pesos(100.0), 100.0);
    }

    #[test]
    fn test_dolares_a_pesos() {
        let m = Moneda::Dolar;
        assert_eq!(m.a_pesos(10.0), 10000.0);
    }

    #[test]
    fn test_euros_a_pesos() {
        let m = Moneda::Euro;
        assert_eq!(m.a_pesos(5.0), 5500.0);
    }

    // --- Tests Ejercicio 3: Estudiante ---

    #[test]
    fn test_estudiante_aprobado() {
        let e = Estudiante::new("Ana", vec![8.0, 7.0, 9.0]);
        assert_eq!(e.promedio(), Some(8.0));
        assert_eq!(e.estado(), "Aprobado");
    }

    #[test]
    fn test_estudiante_desaprobado() {
        let e = Estudiante::new("Juan", vec![4.0, 3.0, 5.0]);
        assert_eq!(e.promedio(), Some(4.0));
        assert_eq!(e.estado(), "Desaprobado");
    }

    #[test]
    fn test_estudiante_sin_notas() {
        let e = Estudiante::new("Pedro", vec![]);
        assert_eq!(e.promedio(), None);
        assert_eq!(e.estado(), "Sin notas");
    }
}
