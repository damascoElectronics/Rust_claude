// =============================================================
// Ejercicio 1: Celsius a Fahrenheit
// Convierte grados Celsius a Fahrenheit usando: F = (C * 9/5) + 32
// =============================================================

pub fn celsius_a_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agua_congela() {
        // 0°C = 32°F
        assert_eq!(celsius_a_fahrenheit(0.0), 32.0);
    }

    #[test]
    fn test_agua_hierve() {
        // 100°C = 212°F
        assert_eq!(celsius_a_fahrenheit(100.0), 212.0);
    }

    #[test]
    fn test_negativo() {
        // -40°C = -40°F (punto donde coinciden)
        assert_eq!(celsius_a_fahrenheit(-40.0), -40.0);
    }

    #[test]
    fn test_temperatura_corporal() {
        // 37°C ~ 98.6°F
        let resultado = celsius_a_fahrenheit(37.0);
        assert!((resultado - 98.6).abs() < 0.01);
    }
}
