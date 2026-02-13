// Sub-module: matematicas::avanzada
// Operaciones matematicas mas complejas

/// Calcula el factorial de n
pub fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

/// Calcula el n-esimo numero de Fibonacci
pub fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a: u64 = 0;
            let mut b: u64 = 1;
            for _ in 2..=n {
                let temp = b;
                b = a + b;
                a = temp;
            }
            b
        }
    }
}

/// Verifica si un numero es primo
pub fn es_primo(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

/// Calcula la potencia: base^exponente
pub fn potencia(base: f64, exponente: u32) -> f64 {
    let mut resultado = 1.0;
    for _ in 0..exponente {
        resultado *= base;
    }
    resultado
}

/// Maximo comun divisor (algoritmo de Euclides)
pub fn mcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_es_primo() {
        assert!(!es_primo(0));
        assert!(!es_primo(1));
        assert!(es_primo(2));
        assert!(es_primo(7));
        assert!(!es_primo(9));
        assert!(es_primo(97));
    }

    #[test]
    fn test_potencia() {
        assert_eq!(potencia(2.0, 10), 1024.0);
        assert_eq!(potencia(3.0, 0), 1.0);
    }

    #[test]
    fn test_mcd() {
        assert_eq!(mcd(12, 8), 4);
        assert_eq!(mcd(100, 75), 25);
    }
}
