pub mod datos_de_estancias;
pub mod datos_de_estancias_mariadb;
mod datos_de_estancias_pruebas;
pub mod datos_de_reservas;
pub mod datos_de_reservas_mariadb;
mod datos_de_reservas_pruebas;
pub mod manejo_de_estancias_y_reservas;

use chrono::{DateTime, Local};

use crate::habitaciones::Habitacion;
use crate::huespedes::Huesped;

#[derive(Clone)]
pub struct Estancia {
    pub habitaciones: Vec<Habitacion>,
    pub huespedes: Vec<Huesped>,
    pub entrada_real: DateTime<Local>,
    pub salida_prevista: DateTime<Local>,
    pub salida_real: Option<DateTime<Local>>,
}

impl Estancia {
    pub fn get_habitaciones(&self) -> Vec<Habitacion> {
        self.habitaciones.clone()
    }
}
