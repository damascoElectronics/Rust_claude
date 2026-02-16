// =============================================================
// Ejercicio 2: Convertidor de Unidades
// Funciones de conversion: km<->millas, kg<->libras.
// Constantes privadas del modulo, funciones publicas.
// Practica: visibilidad (pub vs privado), constantes
// =============================================================

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_km_a_millas() {
        let resultado = km_a_millas(10.0);
        assert!((resultado - 6.21371).abs() < 0.001);
    }

    #[test]
    fn test_millas_a_km() {
        let resultado = millas_a_km(6.21371);
        assert!((resultado - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_kg_a_libras() {
        let resultado = kg_a_libras(1.0);
        assert!((resultado - 2.20462).abs() < 0.001);
    }
}
