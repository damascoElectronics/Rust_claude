// =============================================================
// Capitulo 9: Modulos y Cargo - Ejercicios
// =============================================================

// --- Ejercicio 1 ---
// Organiza codigo en modulos: crea un modulo `matematica` con
// sub-modulos `basica` y `avanzada`.
pub mod matematica {
    pub mod basica {
        pub fn sumar(a: f64, b: f64) -> f64 {
            a + b
        }

        pub fn restar(a: f64, b: f64) -> f64 {
            a - b
        }

        pub fn multiplicar(a: f64, b: f64) -> f64 {
            a * b
        }
    }

    pub mod avanzada {
        pub fn potencia(base: f64, exponente: u32) -> f64 {
            let mut resultado = 1.0;
            for _ in 0..exponente {
                resultado *= base;
            }
            resultado
        }

        pub fn factorial(n: u64) -> u64 {
            if n <= 1 {
                1
            } else {
                n * factorial(n - 1)
            }
        }
    }

    // Re-export de las funciones mas usadas
    pub use basica::sumar;
    pub use avanzada::factorial;
}

// --- Ejercicio 2 ---
// Crea un modulo `convertidor` con funciones de conversion de unidades.
// Demuestra el uso de pub, pub(crate) y funciones privadas.
pub mod convertidor {
    // Constante privada del modulo
    const FACTOR_KM_A_MILLAS: f64 = 0.621371;
    const FACTOR_KG_A_LIBRAS: f64 = 2.20462;

    pub fn km_a_millas(km: f64) -> f64 {
        km * FACTOR_KM_A_MILLAS
    }

    pub fn millas_a_km(millas: f64) -> f64 {
        millas / FACTOR_KM_A_MILLAS
    }

    pub fn kg_a_libras(kg: f64) -> f64 {
        kg * FACTOR_KG_A_LIBRAS
    }

    pub fn libras_a_kg(libras: f64) -> f64 {
        libras / FACTOR_KG_A_LIBRAS
    }
}

// --- Ejercicio 3 ---
// Crea un modulo `validador` que use funciones de otros modulos internos.
// Demuestra el uso de `use` y paths relativos/absolutos.
pub mod validador {
    use super::matematica;

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

    // Valida que una suma sea correcta
    pub fn validar_suma(a: f64, b: f64, resultado_esperado: f64) -> Resultado {
        let valor = matematica::basica::sumar(a, b);
        Resultado {
            operacion: format!("{} + {}", a, b),
            valor,
            es_valido: (valor - resultado_esperado).abs() < f64::EPSILON,
        }
    }

    // Valida que un factorial sea correcto
    pub fn validar_factorial(n: u64, resultado_esperado: u64) -> Resultado {
        let valor = matematica::avanzada::factorial(n) as f64;
        Resultado {
            operacion: format!("{}!", n),
            valor,
            es_valido: valor as u64 == resultado_esperado,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Tests Ejercicio 1: matematica (modulos) ---

    #[test]
    fn test_sumar() {
        assert_eq!(matematica::basica::sumar(2.0, 3.0), 5.0);
    }

    #[test]
    fn test_potencia() {
        assert_eq!(matematica::avanzada::potencia(2.0, 10), 1024.0);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(matematica::factorial(5), 120); // via re-export
        assert_eq!(matematica::avanzada::factorial(0), 1);
    }

    #[test]
    fn test_re_export() {
        // sumar disponible directamente via re-export
        assert_eq!(matematica::sumar(10.0, 5.0), 15.0);
    }

    // --- Tests Ejercicio 2: convertidor ---

    #[test]
    fn test_km_a_millas() {
        let resultado = convertidor::km_a_millas(10.0);
        assert!((resultado - 6.21371).abs() < 0.001);
    }

    #[test]
    fn test_millas_a_km() {
        let resultado = convertidor::millas_a_km(6.21371);
        assert!((resultado - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_kg_a_libras() {
        let resultado = convertidor::kg_a_libras(1.0);
        assert!((resultado - 2.20462).abs() < 0.001);
    }

    // --- Tests Ejercicio 3: validador (uso entre modulos) ---

    #[test]
    fn test_validar_suma_correcta() {
        let r = validador::validar_suma(3.0, 4.0, 7.0);
        assert!(r.es_valido);
        assert_eq!(r.valor, 7.0);
    }

    #[test]
    fn test_validar_suma_incorrecta() {
        let r = validador::validar_suma(3.0, 4.0, 8.0);
        assert!(!r.es_valido);
    }

    #[test]
    fn test_validar_factorial() {
        let r = validador::validar_factorial(5, 120);
        assert!(r.es_valido);
        assert_eq!(r.describir(), "5!: 120 (valido)");
    }
}
