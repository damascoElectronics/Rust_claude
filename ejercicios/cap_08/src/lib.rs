// =============================================================
// Capitulo 8: Generics, Traits y Lifetimes - Ejercicios
// =============================================================

use std::fmt;

// --- Ejercicio 1 ---
// Implementa una funcion generica que encuentre el mayor valor en un slice.
// El tipo T debe implementar PartialOrd y Copy.
pub fn mayor<T: PartialOrd + Copy>(lista: &[T]) -> Option<T> {
    if lista.is_empty() {
        return None;
    }
    let mut max = lista[0];
    for &item in &lista[1..] {
        if item > max {
            max = item;
        }
    }
    Some(max)
}

// --- Ejercicio 2 ---
// Define un trait Describible con un metodo describir() -> String.
// Implementalo para dos structs: Perro y Gato.
pub trait Describible {
    fn describir(&self) -> String;
}

pub struct Perro {
    pub nombre: String,
    pub raza: String,
}

pub struct Gato {
    pub nombre: String,
    pub es_interior: bool,
}

impl Describible for Perro {
    fn describir(&self) -> String {
        format!("{} es un perro de raza {}", self.nombre, self.raza)
    }
}

impl Describible for Gato {
    fn describir(&self) -> String {
        let tipo = if self.es_interior { "interior" } else { "exterior" };
        format!("{} es un gato {}", self.nombre, tipo)
    }
}

// --- Ejercicio 3 ---
// Crea un struct generico Pila<T> (stack) con metodos push, pop y esta_vacia.
// Implementa Display para Pila<T> donde T: Display, mostrando los elementos separados por ", ".
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

    // --- Tests Ejercicio 1: mayor (generics) ---

    #[test]
    fn test_mayor_enteros() {
        assert_eq!(mayor(&[1, 5, 3, 9, 2]), Some(9));
    }

    #[test]
    fn test_mayor_flotantes() {
        assert_eq!(mayor(&[1.5, 3.7, 2.1]), Some(3.7));
    }

    #[test]
    fn test_mayor_chars() {
        assert_eq!(mayor(&['a', 'z', 'm']), Some('z'));
    }

    #[test]
    fn test_mayor_vacio() {
        let vacio: &[i32] = &[];
        assert_eq!(mayor(vacio), None);
    }

    // --- Tests Ejercicio 2: Describible (traits) ---

    #[test]
    fn test_describir_perro() {
        let p = Perro {
            nombre: String::from("Rex"),
            raza: String::from("Pastor Aleman"),
        };
        assert_eq!(p.describir(), "Rex es un perro de raza Pastor Aleman");
    }

    #[test]
    fn test_describir_gato_interior() {
        let g = Gato {
            nombre: String::from("Michi"),
            es_interior: true,
        };
        assert_eq!(g.describir(), "Michi es un gato interior");
    }

    #[test]
    fn test_describir_gato_exterior() {
        let g = Gato {
            nombre: String::from("Luna"),
            es_interior: false,
        };
        assert_eq!(g.describir(), "Luna es un gato exterior");
    }

    // --- Tests Ejercicio 3: Pila (generics + Display) ---

    #[test]
    fn test_pila_push_pop() {
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
    fn test_pila_pop_vacia() {
        let mut pila: Pila<i32> = Pila::new();
        assert_eq!(pila.pop(), None);
    }

    #[test]
    fn test_pila_display() {
        let mut pila: Pila<i32> = Pila::new();
        pila.push(10);
        pila.push(20);
        pila.push(30);
        assert_eq!(format!("{}", pila), "[10, 20, 30]");
    }
}
