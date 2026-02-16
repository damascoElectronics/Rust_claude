// =============================================================
// Ejercicio 3: Pila Generica
// Crea un struct Pila<T> (stack) con push, pop, esta_vacia, len.
// Implementa Display para Pila<T> donde T: Display.
// Practica: generics en structs, impl con trait bounds, Display
// =============================================================

use std::fmt;

pub struct Pila<T> {
    elementos: Vec<T>,
}

impl<T> Pila<T> {
    pub fn new() -> Self {
        Pila {
            elementos: Vec::new(),
        }
    }

    pub fn push(&mut self, valor: T) {
        self.elementos.push(valor);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elementos.pop()
    }

    pub fn esta_vacia(&self) -> bool {
        self.elementos.is_empty()
    }

    pub fn len(&self) -> usize {
        self.elementos.len()
    }
}

impl<T: fmt::Display> fmt::Display for Pila<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let strs: Vec<String> = self.elementos.iter().map(|e| e.to_string()).collect();
        write!(f, "[{}]", strs.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut pila: Pila<i32> = Pila::new();
        assert!(pila.esta_vacia());
        pila.push(1);
        pila.push(2);
        pila.push(3);
        assert_eq!(pila.len(), 3);
        assert_eq!(pila.pop(), Some(3));
        assert_eq!(pila.pop(), Some(2));
        assert_eq!(pila.len(), 1);
    }

    #[test]
    fn test_pop_vacia() {
        let mut pila: Pila<i32> = Pila::new();
        assert_eq!(pila.pop(), None);
    }

    #[test]
    fn test_display() {
        let mut pila: Pila<i32> = Pila::new();
        pila.push(10);
        pila.push(20);
        pila.push(30);
        assert_eq!(format!("{}", pila), "[10, 20, 30]");
    }
}
