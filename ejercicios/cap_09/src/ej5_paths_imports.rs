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
                Perro { nombre: String::from(nombre) }
            }

            pub fn hablar(&self) -> String {
                format!("{} dice: guau!", self.nombre)
            }
        }

        impl Gato {
            pub fn new(nombre: &str) -> Self {
                Gato { nombre: String::from(nombre) }
            }

            pub fn hablar(&self) -> String {
                format!("{} dice: miau!", self.nombre)
            }
        }

        /// Usa super:: para acceder al modulo padre
        pub fn contar_animales() -> String {
            // super:: sube un nivel (a animales)
            // Desde aqui podriamos acceder a super::acuaticos si quisieramos
            String::from("terrestres: perros y gatos")
        }
    }

    pub mod acuaticos {
        pub struct Pez {
            pub especie: String,
        }

        impl Pez {
            pub fn new(especie: &str) -> Self {
                Pez { especie: String::from(especie) }
            }

            pub fn habitat(&self) -> String {
                format!("{} vive en el agua", self.especie)
            }
        }
    }

    /// Funcion que usa items de sub-modulos con paths relativos
    pub fn resumen() -> String {
        // use self:: para referir al modulo actual
        let perro = self::terrestres::Perro::new("Rex");
        let pez = self::acuaticos::Pez::new("Nemo");
        format!("{} y {}", perro.hablar(), pez.habitat())
    }
}

/// Modulo que demuestra use con alias
pub mod presentacion {
    // use con alias (as) para nombres mas cortos
    use super::animales::terrestres::Perro as PerroTerrestre;
    use super::animales::acuaticos::Pez;

    pub fn crear_presentacion() -> String {
        let p = PerroTerrestre::new("Firulais");
        let f = Pez::new("Doris");
        format!("{} | {}", p.hablar(), f.habitat())
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
