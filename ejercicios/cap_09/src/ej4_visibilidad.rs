// =============================================================
// Ejercicio 4: Visibilidad y Privacy
// Practica con las reglas de visibilidad:
// - Todo es privado por defecto
// - pub: publico para todos
// - pub(crate): publico solo dentro del crate
// - pub(super): publico solo para el modulo padre
// - Campos privados en structs publicos
// =============================================================

pub mod banco {
    /// Struct publica pero con campos privados (encapsulacion)
    pub struct CuentaBancaria {
        titular: String,         // privado: no se accede desde afuera
        saldo: f64,              // privado: solo se modifica via metodos
        pub numero: u64,         // publico: se puede leer desde afuera
    }

    impl CuentaBancaria {
        pub fn new(titular: &str, numero: u64) -> Self {
            CuentaBancaria {
                titular: String::from(titular),
                saldo: 0.0,
                numero,
            }
        }

        pub fn titular(&self) -> &str {
            &self.titular
        }

        pub fn saldo(&self) -> f64 {
            self.saldo
        }

        pub fn depositar(&mut self, monto: f64) -> Result<f64, &'static str> {
            if monto <= 0.0 {
                return Err("monto debe ser positivo");
            }
            self.saldo += monto;
            Ok(self.saldo)
        }

        pub fn retirar(&mut self, monto: f64) -> Result<f64, &'static str> {
            if monto <= 0.0 {
                return Err("monto debe ser positivo");
            }
            if monto > self.saldo {
                return Err("saldo insuficiente");
            }
            self.saldo -= monto;
            Ok(self.saldo)
        }
    }

    /// Sub-modulo con visibilidad pub(super) - solo visible desde `banco`
    mod auditor {
        pub(super) fn verificar_saldo(saldo: f64) -> bool {
            saldo >= 0.0
        }
    }

    /// Funcion publica que usa el modulo privado internamente
    pub fn es_cuenta_valida(cuenta: &CuentaBancaria) -> bool {
        auditor::verificar_saldo(cuenta.saldo)
    }
}

#[cfg(test)]
mod tests {
    use super::banco::*;

    #[test]
    fn test_crear_cuenta() {
        let cuenta = CuentaBancaria::new("Ana", 12345);
        assert_eq!(cuenta.titular(), "Ana");
        assert_eq!(cuenta.numero, 12345);
        assert_eq!(cuenta.saldo(), 0.0);
    }

    #[test]
    fn test_depositar() {
        let mut cuenta = CuentaBancaria::new("Ana", 1);
        assert_eq!(cuenta.depositar(100.0), Ok(100.0));
        assert_eq!(cuenta.saldo(), 100.0);
    }

    #[test]
    fn test_depositar_negativo() {
        let mut cuenta = CuentaBancaria::new("Ana", 1);
        assert_eq!(cuenta.depositar(-50.0), Err("monto debe ser positivo"));
    }

    #[test]
    fn test_retirar() {
        let mut cuenta = CuentaBancaria::new("Ana", 1);
        cuenta.depositar(100.0).unwrap();
        assert_eq!(cuenta.retirar(30.0), Ok(70.0));
    }

    #[test]
    fn test_retirar_saldo_insuficiente() {
        let mut cuenta = CuentaBancaria::new("Ana", 1);
        assert_eq!(cuenta.retirar(50.0), Err("saldo insuficiente"));
    }

    #[test]
    fn test_cuenta_valida() {
        let cuenta = CuentaBancaria::new("Ana", 1);
        assert!(super::banco::es_cuenta_valida(&cuenta));
    }

    // No se puede acceder a cuenta.saldo directamente (es privado)
    // No se puede acceder a banco::auditor::verificar_saldo (es pub(super))
    // Esto demuestra la encapsulacion de Rust
}
