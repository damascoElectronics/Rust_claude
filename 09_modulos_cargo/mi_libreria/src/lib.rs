// Capitulo 9: Library Crate - Mi Libreria
// Este archivo es el crate root de un library crate

// Re-exportar modules publicos
pub mod matematicas;
pub mod utilidades;

// Re-export para acceso mas facil
pub use matematicas::basica;
pub use matematicas::avanzada;
