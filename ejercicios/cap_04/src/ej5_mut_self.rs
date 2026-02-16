// =============================================================
// Ejercicio 5: Metodos con &mut self y self
// Practica:
// - &self: lee sin modificar
// - &mut self: modifica el struct
// - self: consume el struct (toma ownership)
// =============================================================

pub struct Contador {
    valor: i32,
}

impl Contador {
    pub fn new() -> Self {
        todo!("tu código va aquí")
    }

    pub fn new_con_valor(valor: i32) -> Self {
        todo!("tu código va aquí")
    }

    /// &self - solo lectura
    pub fn valor(&self) -> i32 {
        todo!("tu código va aquí")
    }

    /// &mut self - modifica el contador
    pub fn incrementar(&mut self) {
        todo!("tu código va aquí")
    }

    /// &mut self - modifica el contador por una cantidad
    pub fn incrementar_por(&mut self, cantidad: i32) {
        todo!("tu código va aquí")
    }

    /// &mut self - resetea a cero
    pub fn reset(&mut self) {
        todo!("tu código va aquí")
    }

    /// self - consume el contador y retorna el valor final.
    /// Despues de llamar esto, el Contador ya no existe.
    pub fn consumir(self) -> i32 {
        todo!("tu código va aquí")
    }
}

pub struct CadenaBuilder {
    partes: Vec<String>,
}

impl CadenaBuilder {
    pub fn new() -> Self {
        todo!("tu código va aquí")
    }

    /// &mut self - agrega una parte
    pub fn agregar(&mut self, texto: &str) {
        todo!("tu código va aquí")
    }

    /// &self - cuantas partes tiene
    pub fn cantidad(&self) -> usize {
        todo!("tu código va aquí")
    }

    /// self - consume el builder y retorna el string final unido
    pub fn construir(self, separador: &str) -> String {
        todo!("tu código va aquí")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contador_basico() {
        let mut c = Contador::new();
        assert_eq!(c.valor(), 0);
        c.incrementar();
        c.incrementar();
        c.incrementar();
        assert_eq!(c.valor(), 3);
    }

    #[test]
    fn test_contador_incrementar_por() {
        let mut c = Contador::new_con_valor(10);
        c.incrementar_por(5);
        assert_eq!(c.valor(), 15);
    }

    #[test]
    fn test_contador_reset() {
        let mut c = Contador::new_con_valor(100);
        c.reset();
        assert_eq!(c.valor(), 0);
    }

    #[test]
    fn test_contador_consumir() {
        let mut c = Contador::new();
        c.incrementar_por(42);
        let valor_final = c.consumir();
        assert_eq!(valor_final, 42);
        // c ya no es valido aqui (fue consumido)
    }

    #[test]
    fn test_cadena_builder() {
        let mut b = CadenaBuilder::new();
        b.agregar("hola");
        b.agregar("mundo");
        b.agregar("rust");
        assert_eq!(b.cantidad(), 3);
        let resultado = b.construir(", ");
        assert_eq!(resultado, "hola, mundo, rust");
    }

    #[test]
    fn test_cadena_builder_vacio() {
        let b = CadenaBuilder::new();
        assert_eq!(b.cantidad(), 0);
        assert_eq!(b.construir("-"), "");
    }
}
