// Capitulo 4: Structs y Enums

// Definir un struct
#[derive(Debug)] // permite imprimir con {:?}
struct Usuario {
    nombre: String,
    email: String,
    edad: u32,
    activo: bool,
}

// Struct para demostrar methods
#[derive(Debug)]
struct Rectangulo {
    ancho: f64,
    alto: f64,
}

// Bloque impl: aqui definimos los methods
impl Rectangulo {
    // Method: recibe &self como primer parametro
    fn area(&self) -> f64 {
        self.ancho * self.alto
    }

    fn perimetro(&self) -> f64 {
        2.0 * (self.ancho + self.alto)
    }

    fn es_cuadrado(&self) -> bool {
        (self.ancho - self.alto).abs() < f64::EPSILON
    }

    // Method que recibe otro Rectangulo
    fn puede_contener(&self, otro: &Rectangulo) -> bool {
        self.ancho >= otro.ancho && self.alto >= otro.alto
    }

    // Associated function (sin self) - funciona como constructor
    fn cuadrado(lado: f64) -> Self {
        Self {
            ancho: lado,
            alto: lado,
        }
    }

    fn new(ancho: f64, alto: f64) -> Self {
        Self { ancho, alto }
    }
}

// Tuple struct
#[derive(Debug)]
struct Color(u8, u8, u8);

// Enum basico
#[derive(Debug)]
enum Direccion {
    Norte,
    Sur,
    Este,
    Oeste,
}

// Enum con datos asociados
#[derive(Debug)]
enum Mensaje {
    Salir,
    Mover { x: i32, y: i32 },
    Texto(String),
    Color(u8, u8, u8),
}

impl Mensaje {
    fn describir(&self) {
        match self {
            Mensaje::Salir => println!("  -> Comando: Salir del programa"),
            Mensaje::Mover { x, y } => println!("  -> Comando: Mover a ({x}, {y})"),
            Mensaje::Texto(t) => println!("  -> Comando: Enviar texto '{t}'"),
            Mensaje::Color(r, g, b) => println!("  -> Comando: Cambiar color a ({r}, {g}, {b})"),
        }
    }
}

// Enum para modelar figuras geometricas
#[derive(Debug)]
enum Figura {
    Circulo(f64),
    Rectangulo(f64, f64),
}

impl Figura {
    fn area(&self) -> f64 {
        match self {
            Figura::Circulo(radio) => std::f64::consts::PI * radio * radio,
            Figura::Rectangulo(ancho, alto) => ancho * alto,
        }
    }

    fn nombre(&self) -> &str {
        match self {
            Figura::Circulo(_) => "Circulo",
            Figura::Rectangulo(_, _) => "Rectangulo",
        }
    }
}

fn main() {
    // =============================================
    // STRUCTS
    // =============================================
    println!("=== STRUCTS ===\n");

    // Crear un struct
    let usuario1 = Usuario {
        nombre: String::from("Ana"),
        email: String::from("ana@mail.com"),
        edad: 28,
        activo: true,
    };
    println!("Usuario: {} ({}, {})", usuario1.nombre, usuario1.email, usuario1.edad);

    // Struct update syntax
    let usuario2 = Usuario {
        nombre: String::from("Carlos"),
        email: String::from("carlos@mail.com"),
        ..usuario1 // toma edad y activo de usuario1
    };
    println!("Usuario2: {} (edad: {})", usuario2.nombre, usuario2.edad);

    // Tuple struct
    let rojo = Color(255, 0, 0);
    println!("Color rojo: ({}, {}, {})", rojo.0, rojo.1, rojo.2);

    // =============================================
    // METHODS
    // =============================================
    println!("\n=== METHODS ===\n");

    let rect = Rectangulo::new(10.0, 5.0);
    println!("Rectangulo: {:?}", rect);
    println!("Area: {}", rect.area());
    println!("Perimetro: {}", rect.perimetro());
    println!("Es cuadrado? {}", rect.es_cuadrado());

    let cuadrado = Rectangulo::cuadrado(5.0);
    println!("\nCuadrado: {:?}", cuadrado);
    println!("Es cuadrado? {}", cuadrado.es_cuadrado());

    println!("\nrect puede contener cuadrado? {}", rect.puede_contener(&cuadrado));

    // =============================================
    // ENUMS
    // =============================================
    println!("\n=== ENUMS ===\n");

    let dir = Direccion::Norte;
    println!("Direccion: {:?}", dir);

    // Enum con datos
    let mensajes = vec![
        Mensaje::Texto(String::from("Hola Rust")),
        Mensaje::Mover { x: 10, y: 20 },
        Mensaje::Color(255, 128, 0),
        Mensaje::Salir,
    ];

    println!("Procesando mensajes:");
    for msg in &mensajes {
        msg.describir();
    }

    // =============================================
    // OPTION<T>
    // =============================================
    println!("\n=== OPTION<T> ===\n");

    let algun_numero: Option<i32> = Some(42);
    let sin_numero: Option<i32> = None;

    // match para manejar Option
    match algun_numero {
        Some(n) => println!("Tenemos un numero: {n}"),
        None => println!("No hay numero"),
    }

    match sin_numero {
        Some(n) => println!("Tenemos un numero: {n}"),
        None => println!("No hay numero"),
    }

    // if let: forma concisa cuando solo te interesa un caso
    if let Some(n) = algun_numero {
        println!("if let: el numero es {n}");
    }

    // Methods utiles de Option
    let valor = algun_numero.unwrap_or(0); // retorna 0 si es None
    println!("unwrap_or: {valor}");

    let valor = sin_numero.unwrap_or(0);
    println!("unwrap_or (None): {valor}");

    // Option con struct
    let nombre_completo = buscar_segundo_nombre("Juan Carlos Perez");
    match nombre_completo {
        Some(segundo) => println!("Segundo nombre: {segundo}"),
        None => println!("No tiene segundo nombre"),
    }

    // =============================================
    // ENUM COMO MODELO DE DATOS
    // =============================================
    println!("\n=== FIGURAS CON ENUM ===\n");

    let figuras: Vec<Figura> = vec![
        Figura::Circulo(5.0),
        Figura::Rectangulo(10.0, 3.0),
        Figura::Circulo(2.5),
    ];

    for fig in &figuras {
        println!("{}: area = {:.2}", fig.nombre(), fig.area());
    }
}

fn buscar_segundo_nombre(nombre_completo: &str) -> Option<&str> {
    let partes: Vec<&str> = nombre_completo.split_whitespace().collect();
    if partes.len() >= 3 {
        Some(partes[1])
    } else {
        None
    }
}
