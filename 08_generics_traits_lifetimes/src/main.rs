// Capitulo 8: Generics, Traits y Lifetimes

use std::fmt;

// =============================================
// TRAITS
// =============================================

// Definir un trait
trait Resumen {
    fn resumir(&self) -> String;

    // Method con implementacion por defecto
    fn tipo(&self) -> &str {
        "contenido"
    }
}

trait Area {
    fn area(&self) -> f64;
    fn descripcion(&self) -> String;
}

// Structs que implementaran los traits
struct Articulo {
    titulo: String,
    autor: String,
}

struct Tweet {
    usuario: String,
    mensaje: String,
}

struct Circulo {
    radio: f64,
}

struct Rectangulo {
    ancho: f64,
    alto: f64,
}

// Implementar trait Resumen para Articulo
impl Resumen for Articulo {
    fn resumir(&self) -> String {
        format!("'{}' por {}", self.titulo, self.autor)
    }

    fn tipo(&self) -> &str {
        "articulo"
    }
}

// Implementar trait Resumen para Tweet
impl Resumen for Tweet {
    fn resumir(&self) -> String {
        format!("@{}: {}", self.usuario, self.mensaje)
    }

    fn tipo(&self) -> &str {
        "tweet"
    }
}

// Implementar trait Area
impl Area for Circulo {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radio * self.radio
    }

    fn descripcion(&self) -> String {
        format!("Circulo con radio {:.2}", self.radio)
    }
}

impl Area for Rectangulo {
    fn area(&self) -> f64 {
        self.ancho * self.alto
    }

    fn descripcion(&self) -> String {
        format!("Rectangulo de {:.2} x {:.2}", self.ancho, self.alto)
    }
}

// Implementar Display para nuestros structs
impl fmt::Display for Circulo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circulo(r={})", self.radio)
    }
}

impl fmt::Display for Rectangulo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rect({}x{})", self.ancho, self.alto)
    }
}

// =============================================
// GENERICS
// =============================================

// Struct generico
#[derive(Debug)]
struct Punto<T> {
    x: T,
    y: T,
}

// Methods genericos
impl<T: fmt::Display> Punto<T> {
    fn mostrar(&self) {
        println!("Punto({}, {})", self.x, self.y);
    }
}

// Method solo para Punto<f64>
impl Punto<f64> {
    fn distancia_al_origen(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    fn distancia_a(&self, otro: &Punto<f64>) -> f64 {
        ((self.x - otro.x).powi(2) + (self.y - otro.y).powi(2)).sqrt()
    }
}

// Struct con dos tipos genericos
#[derive(Debug)]
struct Par<T, U> {
    primero: T,
    segundo: U,
}

impl<T: fmt::Display, U: fmt::Display> Par<T, U> {
    fn mostrar(&self) {
        println!("({}, {})", self.primero, self.segundo);
    }
}

// Funcion generica: encontrar el mayor
fn mayor<T: PartialOrd>(lista: &[T]) -> &T {
    let mut max = &lista[0];
    for item in &lista[1..] {
        if item > max {
            max = item;
        }
    }
    max
}

// Trait como parameter (impl Trait syntax)
fn notificar(item: &impl Resumen) {
    println!("[{}] {}", item.tipo(), item.resumir());
}

// Trait bound syntax + multiples traits
fn imprimir_area<T: Area + fmt::Display>(figura: &T) {
    println!("{}: area = {:.2}", figura, figura.area());
}

// =============================================
// LIFETIMES
// =============================================

// Funcion con lifetime annotations
fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Struct con lifetime (contiene una reference)
#[derive(Debug)]
struct Extracto<'a> {
    texto: &'a str,
    longitud: usize,
}

impl<'a> Extracto<'a> {
    fn new(texto: &'a str) -> Self {
        Extracto {
            texto,
            longitud: texto.len(),
        }
    }

    fn primera_palabra(&self) -> &str {
        self.texto.split_whitespace().next().unwrap_or("")
    }
}

impl<'a> fmt::Display for Extracto<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'{}' ({} chars)", self.texto, self.longitud)
    }
}

fn main() {
    // =============================================
    // GENERICS EN ACCION
    // =============================================
    println!("=== GENERICS ===\n");

    // Punto con diferentes types
    let p_int = Punto { x: 5, y: 10 };
    let p_float = Punto { x: 1.5, y: 4.2 };
    p_int.mostrar();
    p_float.mostrar();

    // Methods especificos para f64
    println!("Distancia al origen: {:.2}", p_float.distancia_al_origen());

    let p2 = Punto { x: 4.0, y: 6.0 };
    println!(
        "Distancia entre puntos: {:.2}",
        p_float.distancia_a(&p2)
    );

    // Par con tipos mixtos
    let par = Par {
        primero: "Rust",
        segundo: 2021,
    };
    print!("Par: ");
    par.mostrar();

    // Funcion generica mayor
    let numeros = vec![34, 50, 25, 100, 65];
    println!("\nMayor de {:?}: {}", numeros, mayor(&numeros));

    let palabras = vec!["manzana", "naranja", "banana"];
    println!("Mayor de {:?}: {}", palabras, mayor(&palabras));

    // =============================================
    // TRAITS EN ACCION
    // =============================================
    println!("\n=== TRAITS ===\n");

    let articulo = Articulo {
        titulo: String::from("Rust en 2024"),
        autor: String::from("Ferris"),
    };

    let tweet = Tweet {
        usuario: String::from("rustlang"),
        mensaje: String::from("Rust 2024 ya esta aqui!"),
    };

    // Usar trait como parameter
    notificar(&articulo);
    notificar(&tweet);

    // Trait Area con diferentes figuras
    println!();
    let circulo = Circulo { radio: 5.0 };
    let rectangulo = Rectangulo {
        ancho: 10.0,
        alto: 3.0,
    };

    imprimir_area(&circulo);
    imprimir_area(&rectangulo);

    // Trait objects (dynamic dispatch con dyn)
    let figuras: Vec<Box<dyn Area>> = vec![
        Box::new(Circulo { radio: 3.0 }),
        Box::new(Rectangulo {
            ancho: 4.0,
            alto: 5.0,
        }),
        Box::new(Circulo { radio: 1.5 }),
    ];

    println!("\nFiguras (dynamic dispatch):");
    let total: f64 = figuras.iter().map(|f| f.area()).sum();
    for fig in &figuras {
        println!("  {} -> area: {:.2}", fig.descripcion(), fig.area());
    }
    println!("  Area total: {total:.2}");

    // =============================================
    // LIFETIMES EN ACCION
    // =============================================
    println!("\n=== LIFETIMES ===\n");

    let string1 = String::from("cadena larga");
    let resultado;
    {
        let string2 = String::from("corta");
        resultado = mas_largo(string1.as_str(), string2.as_str());
        println!("El mas largo es: '{resultado}'");
    }

    // Struct con lifetime
    let texto = String::from("Rust es un lenguaje seguro y rapido");
    let extracto = Extracto::new(&texto);
    println!("Extracto: {extracto}");
    println!("Primera palabra: '{}'", extracto.primera_palabra());

    // Lifetime con 'static
    let estatico: &'static str = "Vivo toda la ejecucion del programa";
    println!("Static: {estatico}");

    // =============================================
    // COMBINANDO TODO
    // =============================================
    println!("\n=== COMBINANDO GENERICS + TRAITS + LIFETIMES ===\n");

    // Funcion que combina los tres conceptos
    fn anunciar_mayor<'a, T: fmt::Display>(
        x: &'a str,
        y: &'a str,
        anuncio: T,
    ) -> &'a str {
        println!("Anuncio: {anuncio}");
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let resultado = anunciar_mayor("Rust", "Python", "Comparando lenguajes...");
    println!("El mas largo: {resultado}");
}
