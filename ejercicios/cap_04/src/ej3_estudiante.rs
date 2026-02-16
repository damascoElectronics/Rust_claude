// =============================================================
// Ejercicio 3: Estudiante
// Crea un struct Estudiante con nombre (String) y notas (Vec<f64>).
// Implementa promedio() -> Option<f64> y estado() -> &str
// ("Aprobado" si promedio >= 6.0, "Desaprobado" si no, "Sin notas" si vacio)
// Practica: structs con Vec, Option, match con guards
// =============================================================

pub struct Estudiante {
    pub nombre: String,
    pub notas: Vec<f64>,
}

impl Estudiante {
    pub fn new(nombre: &str, notas: Vec<f64>) -> Self {
        Estudiante {
            nombre: String::from(nombre),
            notas,
        }
    }

    pub fn promedio(&self) -> Option<f64> {
        if self.notas.is_empty() {
            return None;
        }
        let suma: f64 = self.notas.iter().sum();
        Some(suma / self.notas.len() as f64)
    }

    pub fn estado(&self) -> &str {
        match self.promedio() {
            Some(p) if p >= 6.0 => "Aprobado",
            Some(_) => "Desaprobado",
            None => "Sin notas",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aprobado() {
        let e = Estudiante::new("Ana", vec![8.0, 7.0, 9.0]);
        assert_eq!(e.promedio(), Some(8.0));
        assert_eq!(e.estado(), "Aprobado");
    }

    #[test]
    fn test_desaprobado() {
        let e = Estudiante::new("Juan", vec![4.0, 3.0, 5.0]);
        assert_eq!(e.promedio(), Some(4.0));
        assert_eq!(e.estado(), "Desaprobado");
    }

    #[test]
    fn test_sin_notas() {
        let e = Estudiante::new("Pedro", vec![]);
        assert_eq!(e.promedio(), None);
        assert_eq!(e.estado(), "Sin notas");
    }
}
