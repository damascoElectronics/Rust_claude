// =============================================================
// Ejercicio 6: Lifetimes
// Practica con anotaciones de lifetime:
// - 'a en funciones que retornan references
// - Lifetimes en structs que contienen references
// - Lifetime elision: cuando el compilador infiere los lifetimes
// =============================================================

/// Retorna la referencia al string mas largo.
/// Necesita lifetime 'a porque retorna una referencia que podria
/// venir de x o de y, y el compilador necesita saber cuanto vive.
pub fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    todo!("tu código va aquí")
}

/// Retorna la primera parte de un string hasta un separador.
/// Si no encuentra el separador, retorna todo el string.
/// Lifetime implicito: el compilador infiere que el retorno
/// vive tanto como el input.
pub fn antes_de(texto: &str, separador: char) -> &str {
    todo!("tu código va aquí")
}

/// Struct que contiene una referencia - necesita lifetime annotation.
/// Importa dice: "este struct no puede vivir mas que el &str que contiene"
pub struct Extracto<'a> {
    pub contenido: &'a str,
}

impl<'a> Extracto<'a> {
    pub fn new(texto: &'a str) -> Self {
        todo!("tu código va aquí")
    }

    /// Retorna las primeras `n` palabras del extracto
    pub fn primeras_palabras(&self, n: usize) -> String {
        todo!("tu código va aquí")
    }

    /// Retorna el largo del contenido
    pub fn largo(&self) -> usize {
        todo!("tu código va aquí")
    }
}

/// Struct con multiples lifetimes (aunque raro, posible)
pub struct Comparador<'a, 'b> {
    pub izquierda: &'a str,
    pub derecha: &'b str,
}

impl<'a, 'b> Comparador<'a, 'b> {
    pub fn son_iguales(&self) -> bool {
        todo!("tu código va aquí")
    }

    pub fn cual_es_mayor(&self) -> &str {
        todo!("tu código va aquí")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mas_largo() {
        assert_eq!(mas_largo("hola", "mundo!"), "mundo!");
        assert_eq!(mas_largo("largo", "si"), "largo");
        assert_eq!(mas_largo("ab", "cd"), "ab"); // empate -> primero
    }

    #[test]
    fn test_antes_de() {
        assert_eq!(antes_de("hola@mundo.com", '@'), "hola");
        assert_eq!(antes_de("sin separador", '@'), "sin separador");
        assert_eq!(antes_de("a:b:c", ':'), "a");
    }

    #[test]
    fn test_extracto_primeras_palabras() {
        let texto = String::from("Rust es un lenguaje de programacion");
        let ext = Extracto::new(&texto);
        assert_eq!(ext.primeras_palabras(3), "Rust es un");
        assert_eq!(ext.largo(), 35);
    }

    #[test]
    fn test_extracto_lifetime() {
        // Demuestra que el extracto vive tanto como el texto original
        let ext;
        {
            let texto = String::from("hola mundo");
            ext = Extracto::new(&texto);
            assert_eq!(ext.contenido, "hola mundo");
        }
        // ext ya no es valido fuera de este scope porque texto fue dropeado
        // (este test demuestra el uso correcto dentro del scope)
    }

    #[test]
    fn test_comparador_iguales() {
        let c = Comparador {
            izquierda: "hola",
            derecha: "hola",
        };
        assert!(c.son_iguales());
    }

    #[test]
    fn test_comparador_diferentes() {
        let c = Comparador {
            izquierda: "rust",
            derecha: "python",
        };
        assert!(!c.son_iguales());
        assert_eq!(c.cual_es_mayor(), "python"); // 6 > 4 chars
    }
}
