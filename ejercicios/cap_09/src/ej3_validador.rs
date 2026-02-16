// =============================================================
// Ejercicio 3: Validador
// Usa funciones de otros modulos (ej1_matematica) para validar
// resultados de operaciones matematicas.
// Practica: use con paths, comunicacion entre modulos
// =============================================================

use crate::ej1_matematica;

pub struct Resultado {
    pub operacion: String,
    pub valor: f64,
    pub es_valido: bool,
}

impl Resultado {
    pub fn describir(&self) -> String {
        let estado = if self.es_valido { "valido" } else { "invalido" };
        format!("{}: {} ({})", self.operacion, self.valor, estado)
    }
}

pub fn validar_suma(a: f64, b: f64, resultado_esperado: f64) -> Resultado {
    let valor = ej1_matematica::basica::sumar(a, b);
    Resultado {
        operacion: format!("{} + {}", a, b),
        valor,
        es_valido: (valor - resultado_esperado).abs() < f64::EPSILON,
    }
}

pub fn validar_factorial(n: u64, resultado_esperado: u64) -> Resultado {
    let valor = ej1_matematica::avanzada::factorial(n) as f64;
    Resultado {
        operacion: format!("{}!", n),
        valor,
        es_valido: valor as u64 == resultado_esperado,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suma_correcta() {
        let r = validar_suma(3.0, 4.0, 7.0);
        assert!(r.es_valido);
        assert_eq!(r.valor, 7.0);
    }

    #[test]
    fn test_suma_incorrecta() {
        let r = validar_suma(3.0, 4.0, 8.0);
        assert!(!r.es_valido);
    }

    #[test]
    fn test_factorial() {
        let r = validar_factorial(5, 120);
        assert!(r.es_valido);
        assert_eq!(r.describir(), "5!: 120 (valido)");
    }
}
