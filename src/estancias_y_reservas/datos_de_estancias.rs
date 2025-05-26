use chrono::{DateTime, Local};

use super::Estancia;
use crate::habitaciones::Habitacion;
use crate::huespedes::Huesped;

pub trait DatosDeEstancias {
    fn crear_estancia(
        &mut self,
        habitaciones: Vec<Habitacion>,
        huespedes: Vec<Huesped>,
        salida_prevista: DateTime<Local>,
    ) -> Result<String, String>;

    fn la_habitacion_esta_libre(&self, id_habitacion: &str) -> bool;
}
