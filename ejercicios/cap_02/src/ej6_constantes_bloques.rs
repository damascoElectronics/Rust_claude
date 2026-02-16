// =============================================================
// Ejercicio 6: Constantes y Block Expressions
// Practica:
// - const: declarar y usar constantes
// - Block expressions: bloques {} que retornan un valor
// - Statements vs expressions: diferencia entre ; y sin ;
// =============================================================

const VELOCIDAD_LUZ_KMS: f64 = 299_792.458;
const SEGUNDOS_POR_MINUTO: u32 = 60;
const MINUTOS_POR_HORA: u32 = 60;

/// Calcula cuantos km recorre la luz en `segundos` segundos
pub fn distancia_luz(segundos: f64) -> f64 {
    VELOCIDAD_LUZ_KMS * segundos
}

/// Convierte horas a segundos usando las constantes definidas
pub fn horas_a_segundos(horas: u32) -> u32 {
    horas * MINUTOS_POR_HORA * SEGUNDOS_POR_MINUTO
}

/// Usa una block expression para calcular el IMC y retornar la categoria.
/// IMC = peso / (altura * altura)
/// < 18.5 -> "bajo peso", 18.5..25.0 -> "normal", 25.0..30.0 -> "sobrepeso", >= 30.0 -> "obesidad"
pub fn categoria_imc(peso_kg: f64, altura_m: f64) -> &'static str {
    let categoria = {
        let imc = peso_kg / (altura_m * altura_m);
        if imc < 18.5 {
            "bajo peso"
        } else if imc < 25.0 {
            "normal"
        } else if imc < 30.0 {
            "sobrepeso"
        } else {
            "obesidad"
        }
    };
    categoria
}

/// Demuestra statements vs expressions:
/// Calcula el area de un triangulo usando una block expression
/// area = (base * altura) / 2
pub fn area_triangulo(base: f64, altura: f64) -> f64 {
    let area = {
        let producto = base * altura;
        producto / 2.0 // expression: retorna el valor (sin ;)
    };
    area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distancia_luz_un_segundo() {
        assert!((distancia_luz(1.0) - 299_792.458).abs() < 0.001);
    }

    #[test]
    fn test_horas_a_segundos() {
        assert_eq!(horas_a_segundos(1), 3600);
        assert_eq!(horas_a_segundos(2), 7200);
        assert_eq!(horas_a_segundos(0), 0);
    }

    #[test]
    fn test_imc_bajo_peso() {
        assert_eq!(categoria_imc(50.0, 1.80), "bajo peso");
    }

    #[test]
    fn test_imc_normal() {
        assert_eq!(categoria_imc(70.0, 1.75), "normal");
    }

    #[test]
    fn test_imc_sobrepeso() {
        assert_eq!(categoria_imc(85.0, 1.75), "sobrepeso");
    }

    #[test]
    fn test_area_triangulo() {
        assert_eq!(area_triangulo(10.0, 5.0), 25.0);
        assert_eq!(area_triangulo(6.0, 3.0), 9.0);
    }
}
