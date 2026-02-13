// Capitulo 6: Manejo de Errors (Error Handling)

use std::fs;
use std::io;
use std::num::ParseIntError;

// Custom error type
#[derive(Debug)]
enum AppError {
    IoError(io::Error),
    ParseError(ParseIntError),
    DivisionPorCero,
    Personalizado(String),
}

// Implementar Display para nuestro error
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "Error de IO: {e}"),
            AppError::ParseError(e) => write!(f, "Error de parseo: {e}"),
            AppError::DivisionPorCero => write!(f, "Division por cero"),
            AppError::Personalizado(msg) => write!(f, "{msg}"),
        }
    }
}

// Conversiones automaticas con From trait
impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::IoError(e)
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::ParseError(e)
    }
}

fn main() {
    // =============================================
    // RESULT<T, E> BASICO
    // =============================================
    println!("=== RESULT<T, E> ===\n");

    // Parsear un string a numero retorna Result
    let numero: Result<i32, _> = "42".parse();
    match numero {
        Ok(n) => println!("Parseado exitosamente: {n}"),
        Err(e) => println!("Error al parsear: {e}"),
    }

    let invalido: Result<i32, _> = "abc".parse();
    match invalido {
        Ok(n) => println!("Parseado: {n}"),
        Err(e) => println!("Error al parsear 'abc': {e}"),
    }

    // =============================================
    // UNWRAP Y EXPECT
    // =============================================
    println!("\n=== UNWRAP Y EXPECT ===\n");

    // unwrap_or: valor por defecto si hay error
    let valor: i32 = "no_numero".parse().unwrap_or(0);
    println!("unwrap_or: {valor}");

    // unwrap_or_else: closure que genera el valor por defecto
    let valor: i32 = "no_numero".parse().unwrap_or_else(|_| {
        println!("  (usando valor por defecto)");
        -1
    });
    println!("unwrap_or_else: {valor}");

    // =============================================
    // FUNCION QUE RETORNA RESULT
    // =============================================
    println!("\n=== FUNCIONES CON RESULT ===\n");

    // Division segura
    match dividir(10.0, 3.0) {
        Ok(resultado) => println!("10 / 3 = {resultado:.4}"),
        Err(e) => println!("Error: {e}"),
    }

    match dividir(10.0, 0.0) {
        Ok(resultado) => println!("10 / 0 = {resultado}"),
        Err(e) => println!("10 / 0 = Error: {e}"),
    }

    // Parsear y calcular
    println!("\nParsear y calcular:");
    for input in ["42", "abc", "100", "", "999"] {
        match parsear_y_duplicar(input) {
            Ok(n) => println!("  '{input}' -> {n}"),
            Err(e) => println!("  '{input}' -> Error: {e}"),
        }
    }

    // =============================================
    // OPERADOR ? (PROPAGATION)
    // =============================================
    println!("\n=== OPERADOR ? ===\n");

    // Intentar leer un archivo (probablemente no exista)
    match leer_archivo("datos.txt") {
        Ok(contenido) => println!("Contenido: {contenido}"),
        Err(e) => println!("No se pudo leer: {e}"),
    }

    // Encadenar operaciones con ?
    match procesar_datos("  42  ") {
        Ok(resultado) => println!("Procesado: {resultado}"),
        Err(e) => println!("Error: {e}"),
    }

    match procesar_datos("abc") {
        Ok(resultado) => println!("Procesado: {resultado}"),
        Err(e) => println!("Error procesando 'abc': {e}"),
    }

    // =============================================
    // METODOS UTILES DE RESULT
    // =============================================
    println!("\n=== METODOS DE RESULT ===\n");

    let ok_val: Result<i32, String> = Ok(10);
    let err_val: Result<i32, String> = Err("error".to_string());

    // is_ok() / is_err()
    println!("ok_val.is_ok(): {}", ok_val.is_ok());
    println!("err_val.is_err(): {}", err_val.is_err());

    // map: transformar el valor Ok
    let duplicado = ok_val.map(|n| n * 2);
    println!("ok_val.map(|n| n*2): {:?}", duplicado);

    // map_err: transformar el error
    let con_prefijo = err_val.map_err(|e| format!("FATAL: {e}"));
    println!("err_val.map_err: {:?}", con_prefijo);

    // and_then: encadenar operaciones que retornan Result
    let resultado = ok_val.and_then(|n| {
        if n > 5 {
            Ok(n * 100)
        } else {
            Err("muy pequeno".to_string())
        }
    });
    println!("and_then: {:?}", resultado);

    // =============================================
    // CUSTOM ERRORS EN ACCION
    // =============================================
    println!("\n=== CUSTOM ERRORS ===\n");

    match operacion_compleja("42") {
        Ok(resultado) => println!("Resultado: {resultado}"),
        Err(e) => println!("Error: {e}"),
    }

    match operacion_compleja("abc") {
        Ok(resultado) => println!("Resultado: {resultado}"),
        Err(e) => println!("Error: {e}"),
    }

    match operacion_compleja("0") {
        Ok(resultado) => println!("Resultado: {resultado}"),
        Err(e) => println!("Error: {e}"),
    }
}

// Funcion que retorna Result con division segura
fn dividir(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("No se puede dividir por cero".to_string())
    } else {
        Ok(a / b)
    }
}

// Parsear un string y duplicar el valor
fn parsear_y_duplicar(input: &str) -> Result<i32, String> {
    let numero: i32 = input
        .trim()
        .parse()
        .map_err(|e: ParseIntError| e.to_string())?;
    Ok(numero * 2)
}

// Leer archivo usando el operador ?
fn leer_archivo(ruta: &str) -> Result<String, io::Error> {
    let contenido = fs::read_to_string(ruta)?; // ? propaga el error
    Ok(contenido)
}

// Encadenar multiples operaciones con ?
fn procesar_datos(input: &str) -> Result<i32, AppError> {
    let limpio = input.trim();

    if limpio.is_empty() {
        return Err(AppError::Personalizado("Input vacio".to_string()));
    }

    let numero: i32 = limpio.parse()?; // usa From<ParseIntError>
    Ok(numero * 2)
}

// Funcion con custom errors que encadena operaciones
fn operacion_compleja(input: &str) -> Result<f64, AppError> {
    let numero: i32 = input.trim().parse()?;

    if numero == 0 {
        return Err(AppError::DivisionPorCero);
    }

    let resultado = 100.0 / numero as f64;
    Ok(resultado)
}
