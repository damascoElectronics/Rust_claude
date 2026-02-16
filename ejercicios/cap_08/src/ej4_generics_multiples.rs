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
        todo!("tu código va aquí")
    }

    /// Intercambia los tipos: Par<T, U> -> Par<U, T>
    pub fn intercambiar(self) -> Par<U, T> {
        todo!("tu código va aquí")
    }
}

/// Implementacion solo para Par<String, i32> (tipo concreto)
impl Par<String, i32> {
    pub fn describir(&self) -> String {
        todo!("tu código va aquí")
    }
}

/// Usa where clause para trait bounds complejos
pub fn formatear_par<T, U>(par: &Par<T, U>) -> String
where
    T: std::fmt::Display,
    U: std::fmt::Display,
{
    todo!("tu código va aquí")
}

/// Funcion generica con multiples bounds usando where
pub fn mayor_y_mostrar<T>(lista: &[T]) -> Option<String>
where
    T: PartialOrd + Copy + std::fmt::Display,
{
    todo!("tu código va aquí")
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
