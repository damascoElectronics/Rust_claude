// =============================================================
// Ejercicio 4: Generics con Multiples Type Parameters
// Practica:
// - Structs con <T, U> (dos tipos genericos)
// - where clause para trait bounds mas legibles
// - impl especializados para tipos concretos
// =============================================================

/// Par generico con dos tipos diferentes
pub struct Par<T, U> {
    pub primero: T,
    pub segundo: U,
}

impl<T, U> Par<T, U> {
    pub fn new(primero: T, segundo: U) -> Self {
        Par { primero, segundo }
    }

    /// Intercambia los tipos: Par<T, U> -> Par<U, T>
    pub fn intercambiar(self) -> Par<U, T> {
        Par {
            primero: self.segundo,
            segundo: self.primero,
        }
    }
}

/// Implementacion solo para Par<String, i32> (tipo concreto)
impl Par<String, i32> {
    pub fn describir(&self) -> String {
        format!("{} ({})", self.primero, self.segundo)
    }
}

/// Usa where clause para trait bounds complejos
pub fn formatear_par<T, U>(par: &Par<T, U>) -> String
where
    T: std::fmt::Display,
    U: std::fmt::Display,
{
    format!("({}, {})", par.primero, par.segundo)
}

/// Funcion generica con multiples bounds usando where
pub fn mayor_y_mostrar<T>(lista: &[T]) -> Option<String>
where
    T: PartialOrd + Copy + std::fmt::Display,
{
    if lista.is_empty() {
        return None;
    }
    let mut max = lista[0];
    for &item in &lista[1..] {
        if item > max {
            max = item;
        }
    }
    Some(format!("El mayor es: {}", max))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_par_new() {
        let p = Par::new("hola", 42);
        assert_eq!(p.primero, "hola");
        assert_eq!(p.segundo, 42);
    }

    #[test]
    fn test_par_intercambiar() {
        let p = Par::new(1, "mundo");
        let p2 = p.intercambiar();
        assert_eq!(p2.primero, "mundo");
        assert_eq!(p2.segundo, 1);
    }

    #[test]
    fn test_par_describir_especializado() {
        let p = Par::new(String::from("Rust"), 2015);
        assert_eq!(p.describir(), "Rust (2015)");
    }

    #[test]
    fn test_formatear_par() {
        let p = Par::new(42, 3.14);
        assert_eq!(formatear_par(&p), "(42, 3.14)");
    }

    #[test]
    fn test_mayor_y_mostrar() {
        assert_eq!(
            mayor_y_mostrar(&[3, 7, 2, 9, 1]),
            Some(String::from("El mayor es: 9"))
        );
        assert_eq!(mayor_y_mostrar::<i32>(&[]), None);
    }
}
