// =============================================================
// Capitulo 5: Control de Flujo y Pattern Matching - Ejercicios
// =============================================================

// --- Ejercicio 1 ---
// Usa match para clasificar una edad en una etapa de la vida:
// 0..=12 -> "Nino", 13..=17 -> "Adolescente", 18..=64 -> "Adulto", 65.. -> "Adulto mayor"
pub fn etapa_de_vida(edad: u32) -> &'static str {
    match edad {
        0..=12 => "Nino",
        13..=17 => "Adolescente",
        18..=64 => "Adulto",
        _ => "Adulto mayor",
    }
}

// --- Ejercicio 2 ---
// FizzBuzz: dado un numero, retorna:
// - "FizzBuzz" si es divisible por 3 y 5
// - "Fizz" si es divisible por 3
// - "Buzz" si es divisible por 5
// - El numero como string si no aplica ninguna regla
pub fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => String::from("FizzBuzz"),
        (0, _) => String::from("Fizz"),
        (_, 0) => String::from("Buzz"),
        _ => n.to_string(),
    }
}

// --- Ejercicio 3 ---
// Dado un vector de enteros, usa iteradores y pattern matching para retornar
// un tuple (positivos, negativos, ceros) con la cuenta de cada uno.
pub fn contar_signos(numeros: &[i32]) -> (usize, usize, usize) {
    let mut positivos = 0;
    let mut negativos = 0;
    let mut ceros = 0;

    for &n in numeros {
        match n.cmp(&0) {
            std::cmp::Ordering::Greater => positivos += 1,
            std::cmp::Ordering::Less => negativos += 1,
            std::cmp::Ordering::Equal => ceros += 1,
        }
    }

    (positivos, negativos, ceros)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Tests Ejercicio 1: etapa_de_vida ---

    #[test]
    fn test_nino() {
        assert_eq!(etapa_de_vida(5), "Nino");
        assert_eq!(etapa_de_vida(0), "Nino");
        assert_eq!(etapa_de_vida(12), "Nino");
    }

    #[test]
    fn test_adolescente() {
        assert_eq!(etapa_de_vida(13), "Adolescente");
        assert_eq!(etapa_de_vida(17), "Adolescente");
    }

    #[test]
    fn test_adulto() {
        assert_eq!(etapa_de_vida(18), "Adulto");
        assert_eq!(etapa_de_vida(30), "Adulto");
        assert_eq!(etapa_de_vida(64), "Adulto");
    }

    #[test]
    fn test_adulto_mayor() {
        assert_eq!(etapa_de_vida(65), "Adulto mayor");
        assert_eq!(etapa_de_vida(90), "Adulto mayor");
    }

    // --- Tests Ejercicio 2: FizzBuzz ---

    #[test]
    fn test_fizzbuzz_fizz() {
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(9), "Fizz");
    }

    #[test]
    fn test_fizzbuzz_buzz() {
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(10), "Buzz");
    }

    #[test]
    fn test_fizzbuzz_fizzbuzz() {
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(30), "FizzBuzz");
    }

    #[test]
    fn test_fizzbuzz_numero() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(7), "7");
    }

    // --- Tests Ejercicio 3: contar_signos ---

    #[test]
    fn test_signos_mixtos() {
        assert_eq!(contar_signos(&[1, -2, 3, 0, -4, 5]), (3, 2, 1));
    }

    #[test]
    fn test_signos_todos_positivos() {
        assert_eq!(contar_signos(&[1, 2, 3]), (3, 0, 0));
    }

    #[test]
    fn test_signos_vacio() {
        assert_eq!(contar_signos(&[]), (0, 0, 0));
    }
}
