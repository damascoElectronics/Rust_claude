// =============================================================
// Ejercicio 2: Fibonacci
// Calcula el n-esimo numero de Fibonacci (0-indexed).
// fibonacci(0) = 0, fibonacci(1) = 1, fibonacci(2) = 1, ...
// =============================================================

pub fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 1..n {
        let temp = b;
        b = a + b;
        a = temp;
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_cero() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn test_fibonacci_uno() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn test_fibonacci_diez() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_fibonacci_veinte() {
        assert_eq!(fibonacci(20), 6765);
    }
}
