// lib.rs - Logica principal de minigrep
// Separamos la logica del main para poder escribir tests

use std::error::Error;
use std::fs;
use std::env;

/// Configuracion del programa, parseada desde los arguments
pub struct Config {
    pub consulta: String,
    pub archivo: String,
    pub case_insensitive: bool,
}

impl Config {
    /// Crea Config desde los arguments de la linea de comandos.
    /// Retorna Err si faltan arguments.
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Uso: minigrep <patron> <archivo>");
        }

        let consulta = args[1].clone();
        let archivo = args[2].clone();

        // Leer variable de entorno para case-insensitive
        let case_insensitive = env::var("CASE_INSENSITIVE").is_ok();

        Ok(Config {
            consulta,
            archivo,
            case_insensitive,
        })
    }
}

/// Ejecuta la logica principal del programa
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // Leer el archivo (? propaga el error si falla)
    let contenido = fs::read_to_string(&config.archivo)?;

    // Elegir tipo de busqueda
    let resultados = if config.case_insensitive {
        buscar_insensitive(&config.consulta, &contenido)
    } else {
        buscar(&config.consulta, &contenido)
    };

    // Mostrar resultados
    if resultados.is_empty() {
        println!("No se encontraron coincidencias para '{}'", config.consulta);
    } else {
        println!(
            "Se encontraron {} coincidencias para '{}':\n",
            resultados.len(),
            config.consulta
        );
        for (num_linea, linea) in &resultados {
            println!("  {num_linea}: {linea}");
        }
    }

    Ok(())
}

/// Busca lineas que contengan la consulta (case-sensitive)
/// Retorna un Vec de tuplas (numero_de_linea, linea)
pub fn buscar<'a>(consulta: &str, contenido: &'a str) -> Vec<(usize, &'a str)> {
    contenido
        .lines()
        .enumerate()
        .filter(|(_, linea)| linea.contains(consulta))
        .map(|(i, linea)| (i + 1, linea))
        .collect()
}

/// Busca lineas que contengan la consulta (case-insensitive)
pub fn buscar_insensitive<'a>(consulta: &str, contenido: &'a str) -> Vec<(usize, &'a str)> {
    let consulta_lower = consulta.to_lowercase();

    contenido
        .lines()
        .enumerate()
        .filter(|(_, linea)| linea.to_lowercase().contains(&consulta_lower))
        .map(|(i, linea)| (i + 1, linea))
        .collect()
}

// =============================================
// TESTS
// =============================================
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENIDO_PRUEBA: &str = "\
Rust es seguro.
Rust es rapido.
Seguridad, velocidad, concurrencia.
Elige los tres con Rust.";

    #[test]
    fn busqueda_case_sensitive() {
        let resultados = buscar("Rust", CONTENIDO_PRUEBA);
        assert_eq!(resultados.len(), 3);
        assert_eq!(resultados[0].1, "Rust es seguro.");
        assert_eq!(resultados[1].1, "Rust es rapido.");
        assert_eq!(resultados[2].1, "Elige los tres con Rust.");
    }

    #[test]
    fn busqueda_case_insensitive() {
        let resultados = buscar_insensitive("rust", CONTENIDO_PRUEBA);
        assert_eq!(resultados.len(), 3);
    }

    #[test]
    fn busqueda_sin_resultados() {
        let resultados = buscar("Python", CONTENIDO_PRUEBA);
        assert!(resultados.is_empty());
    }

    #[test]
    fn busqueda_parcial() {
        let resultados = buscar("seguro", CONTENIDO_PRUEBA);
        assert_eq!(resultados.len(), 1);
        assert_eq!(resultados[0].0, 1); // linea 1
    }

    #[test]
    fn config_sin_argumentos() {
        let args: Vec<String> = vec!["programa".to_string()];
        assert!(Config::build(&args).is_err());
    }

    #[test]
    fn config_con_argumentos() {
        let args: Vec<String> = vec![
            "programa".to_string(),
            "rust".to_string(),
            "archivo.txt".to_string(),
        ];
        let config = Config::build(&args).unwrap();
        assert_eq!(config.consulta, "rust");
        assert_eq!(config.archivo, "archivo.txt");
    }

    #[test]
    fn numeros_de_linea_correctos() {
        let resultados = buscar("es", CONTENIDO_PRUEBA);
        // "Rust es seguro." esta en linea 1
        // "Rust es rapido." esta en linea 2
        assert_eq!(resultados[0].0, 1);
        assert_eq!(resultados[1].0, 2);
    }
}
