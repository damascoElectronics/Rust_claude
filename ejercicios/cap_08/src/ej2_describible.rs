// =============================================================
// Ejercicio 2: Trait Describible
// Define un trait Describible con metodo describir() -> String.
// Implementalo para Perro y Gato.
// Practica: definir traits, implementarlos en structs
// =============================================================

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perro() {
        let p = Perro {
            nombre: String::from("Rex"),
            raza: String::from("Pastor Aleman"),
        };
        assert_eq!(p.describir(), "Rex es un perro de raza Pastor Aleman");
    }

    #[test]
    fn test_gato_interior() {
        let g = Gato {
            nombre: String::from("Michi"),
            es_interior: true,
        };
        assert_eq!(g.describir(), "Michi es un gato interior");
    }

    #[test]
    fn test_gato_exterior() {
        let g = Gato {
            nombre: String::from("Luna"),
            es_interior: false,
        };
        assert_eq!(g.describir(), "Luna es un gato exterior");
    }
}
