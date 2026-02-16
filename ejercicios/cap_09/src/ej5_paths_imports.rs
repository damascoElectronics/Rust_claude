// =============================================================
// Ejercicio 5: Paths e Imports
// Practica con el sistema de paths y use statements:
// - Paths absolutos (crate::modulo::item)
// - Paths relativos (super::, self::)
// - use con alias (as)
// - Nested paths (use std::io::{self, Read})
// - Glob operator (use module::*)
// =============================================================

pub mod animales {
    pub mod terrestres {
        pub struct Perro {
            pub nombre: String,
        }

        pub struct Gato {
            pub nombre: String,
        }

        impl Perro {
            pub fn new(nombre: &str) -> Self {
                todo!("tu código va aquí")
            }

            pub fn hablar(&self) -> String {
                todo!("tu código va aquí")
            }
        }

        impl Gato {
            pub fn new(nombre: &str) -> Self {
                todo!("tu código va aquí")
            }

            pub fn hablar(&self) -> String {
                todo!("tu código va aquí")
            }
        }

        /// Usa super:: para acceder al modulo padre
        pub fn contar_animales() -> String {
            todo!("tu código va aquí")
        }
    }

    pub mod acuaticos {
        pub struct Pez {
            pub especie: String,
        }

        impl Pez {
            pub fn new(especie: &str) -> Self {
                todo!("tu código va aquí")
            }

            pub fn habitat(&self) -> String {
                todo!("tu código va aquí")
            }
        }
    }

    /// Funcion que usa items de sub-modulos con paths relativos
    pub fn resumen() -> String {
        todo!("tu código va aquí")
    }
}

/// Modulo que demuestra use con alias
pub mod presentacion {
    // use con alias (as) para nombres mas cortos
    use super::animales::terrestres::Perro as PerroTerrestre;
    use super::animales::acuaticos::Pez;

    pub fn crear_presentacion() -> String {
        todo!("tu código va aquí")
    }
}

#[cfg(test)]
mod tests {
    // Nested path import: importa multiples items del mismo modulo
    use super::animales::terrestres::{Perro, Gato};
    use super::animales::acuaticos::Pez;

    #[test]
    fn test_perro() {
        let p = Perro::new("Rex");
        assert_eq!(p.hablar(), "Rex dice: guau!");
    }

    #[test]
    fn test_gato() {
        let g = Gato::new("Michi");
        assert_eq!(g.hablar(), "Michi dice: miau!");
    }

    #[test]
    fn test_pez() {
        let f = Pez::new("Salmon");
        assert_eq!(f.habitat(), "Salmon vive en el agua");
    }

    #[test]
    fn test_resumen() {
        let r = super::animales::resumen();
        assert!(r.contains("Rex dice: guau!"));
        assert!(r.contains("Nemo vive en el agua"));
    }

    #[test]
    fn test_presentacion_con_alias() {
        let r = super::presentacion::crear_presentacion();
        assert!(r.contains("Firulais dice: guau!"));
        assert!(r.contains("Doris vive en el agua"));
    }

    #[test]
    fn test_contar_animales() {
        let r = super::animales::terrestres::contar_animales();
        assert_eq!(r, "terrestres: perros y gatos");
    }
}
