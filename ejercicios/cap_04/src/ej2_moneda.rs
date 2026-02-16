// =============================================================
// Ejercicio 2: Moneda
// Crea un enum Moneda con variantes Peso, Dolar, Euro.
// Implementa a_pesos() que convierte una cantidad a pesos:
// Peso -> x1, Dolar -> x1000, Euro -> x1100
// Practica: enums, match, methods en enums
// =============================================================

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pesos_a_pesos() {
        assert_eq!(Moneda::Peso.a_pesos(100.0), 100.0);
    }

    #[test]
    fn test_dolares_a_pesos() {
        assert_eq!(Moneda::Dolar.a_pesos(10.0), 10000.0);
    }

    #[test]
    fn test_euros_a_pesos() {
        assert_eq!(Moneda::Euro.a_pesos(5.0), 5500.0);
    }
}
