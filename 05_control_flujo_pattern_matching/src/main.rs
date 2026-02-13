// Capitulo 5: Control de Flujo y Pattern Matching

#[derive(Debug)]
enum Moneda {
    Centavo,
    Cinco,
    Diez,
    Veinticinco,
}

impl Moneda {
    fn valor(&self) -> u32 {
        match self {
            Moneda::Centavo => 1,
            Moneda::Cinco => 5,
            Moneda::Diez => 10,
            Moneda::Veinticinco => 25,
        }
    }
}

#[derive(Debug)]
enum Operacion {
    Suma(f64, f64),
    Resta(f64, f64),
    Multiplicacion(f64, f64),
    Division(f64, f64),
}

impl Operacion {
    fn calcular(&self) -> Option<f64> {
        match self {
            Operacion::Suma(a, b) => Some(a + b),
            Operacion::Resta(a, b) => Some(a - b),
            Operacion::Multiplicacion(a, b) => Some(a * b),
            Operacion::Division(a, b) => {
                if *b == 0.0 {
                    None // no se puede dividir por cero
                } else {
                    Some(a / b)
                }
            }
        }
    }
}

struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    // =============================================
    // IF / ELSE
    // =============================================
    println!("=== IF / ELSE ===\n");

    let temperatura = 25;

    if temperatura > 30 {
        println!("Hace mucho calor!");
    } else if temperatura > 20 {
        println!("Temperatura agradable: {temperatura}°C");
    } else if temperatura > 10 {
        println!("Hace un poco de frio");
    } else {
        println!("Hace mucho frio!");
    }

    // if como expression
    let estado = if temperatura > 20 { "calido" } else { "frio" };
    println!("El clima esta {estado}");

    // =============================================
    // LOOPS
    // =============================================
    println!("\n=== LOOPS ===\n");

    // loop con break que retorna valor
    let mut contador = 0;
    let resultado = loop {
        contador += 1;
        if contador == 5 {
            break contador * 10;
        }
    };
    println!("loop result: {resultado}");

    // while loop
    print!("while countdown: ");
    let mut n = 5;
    while n > 0 {
        print!("{n} ");
        n -= 1;
    }
    println!("Despegue!");

    // for con range
    print!("for range: ");
    for i in 1..=5 {
        print!("{i} ");
    }
    println!();

    // for con iterador y enumerate
    let lenguajes = ["Rust", "Python", "Go", "TypeScript"];
    println!("\nLenguajes:");
    for (i, lang) in lenguajes.iter().enumerate() {
        println!("  {i}. {lang}");
    }

    // for en reversa
    print!("\nReversa: ");
    for i in (1..=5).rev() {
        print!("{i} ");
    }
    println!();

    // =============================================
    // MATCH - PATTERN MATCHING
    // =============================================
    println!("\n=== MATCH ===\n");

    // Match basico con numeros
    let dado = 4;
    match dado {
        1 => println!("Sacaste un uno"),
        2 | 3 => println!("Sacaste {dado} (bajo)"),
        4..=6 => println!("Sacaste {dado} (alto)"),
        _ => println!("Dado invalido!"),
    }

    // Match con enum
    let monedas = vec![
        Moneda::Veinticinco,
        Moneda::Diez,
        Moneda::Cinco,
        Moneda::Centavo,
    ];
    let total: u32 = monedas.iter().map(|m| m.valor()).sum();
    println!("Total en monedas: {} centavos", total);

    // Match con Option
    let numeros = vec![Some(10), None, Some(30), None, Some(50)];
    print!("Valores presentes: ");
    for num in &numeros {
        match num {
            Some(n) => print!("{n} "),
            None => print!("- "),
        }
    }
    println!();

    // Match con destructuring de struct
    let puntos = vec![
        Punto { x: 0, y: 5 },
        Punto { x: 3, y: 0 },
        Punto { x: 7, y: 8 },
        Punto { x: 0, y: 0 },
    ];

    println!("\nClasificacion de puntos:");
    for p in &puntos {
        match p {
            Punto { x: 0, y: 0 } => println!("  ({}, {}) -> Origen", p.x, p.y),
            Punto { x: 0, y } => println!("  (0, {y}) -> En eje Y"),
            Punto { x, y: 0 } => println!("  ({x}, 0) -> En eje X"),
            Punto { x, y } => println!("  ({x}, {y}) -> Punto general"),
        }
    }

    // Match con guard
    let numero: Option<i32> = Some(-5);
    match numero {
        Some(n) if n > 0 => println!("\nPositivo: {n}"),
        Some(n) if n < 0 => println!("\nNegativo: {n}"),
        Some(_) => println!("\nCero"),
        None => println!("\nSin valor"),
    }

    // =============================================
    // IF LET y WHILE LET
    // =============================================
    println!("\n=== IF LET / WHILE LET ===\n");

    // if let: forma concisa para un solo patron
    let favorito: Option<&str> = Some("Rust");
    if let Some(lang) = favorito {
        println!("Mi lenguaje favorito es {lang}");
    }

    // while let
    let mut stack = vec![1, 2, 3, 4, 5];
    print!("Stack pop: ");
    while let Some(top) = stack.pop() {
        print!("{top} ");
    }
    println!();

    // =============================================
    // OPERACIONES CON PATTERN MATCHING
    // =============================================
    println!("\n=== CALCULADORA CON ENUM + MATCH ===\n");

    let operaciones = vec![
        Operacion::Suma(10.0, 5.0),
        Operacion::Resta(10.0, 3.0),
        Operacion::Multiplicacion(4.0, 7.0),
        Operacion::Division(15.0, 4.0),
        Operacion::Division(10.0, 0.0),
    ];

    for op in &operaciones {
        match op.calcular() {
            Some(resultado) => println!("  {:?} = {:.2}", op, resultado),
            None => println!("  {:?} = Error (division por cero)", op),
        }
    }

    // =============================================
    // FIZZBUZZ (Ejercicio clasico)
    // =============================================
    println!("\n=== FIZZBUZZ (1-20) ===\n");

    for i in 1..=20 {
        match (i % 3, i % 5) {
            (0, 0) => println!("{i}: FizzBuzz"),
            (0, _) => println!("{i}: Fizz"),
            (_, 0) => println!("{i}: Buzz"),
            _ => println!("{i}"),
        }
    }

    // Clasificar notas
    println!("\n=== CLASIFICAR NOTAS ===\n");
    let notas = [95, 82, 67, 45, 100, 55];
    for nota in notas {
        let clasificacion = match nota {
            90..=100 => "Excelente",
            75..=89 => "Bueno",
            60..=74 => "Suficiente",
            _ => "Reprobado",
        };
        println!("  Nota {nota}: {clasificacion}");
    }
}
