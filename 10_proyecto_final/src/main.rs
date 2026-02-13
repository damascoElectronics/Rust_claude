// main.rs - Entry point de minigrep
// Responsabilidad: parsear arguments, llamar a run(), manejar errores

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Recoger los arguments de la linea de comandos
    let args: Vec<String> = env::args().collect();

    // Parsear la configuracion (manejar error con closure)
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Error en los argumentos: {err}");
        eprintln!("Uso: minigrep <patron> <archivo>");
        process::exit(1);
    });

    println!("Buscando '{}' en '{}'...\n", config.consulta, config.archivo);

    // Ejecutar la logica principal
    if let Err(e) = minigrep::run(&config) {
        eprintln!("Error de ejecucion: {e}");
        process::exit(1);
    }
}
