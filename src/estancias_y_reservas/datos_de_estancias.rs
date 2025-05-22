use chrono::{DateTime, Local};

use super::estancias_y_reservas::Estancia;
use crate::habitaciones::habitaciones::Habitacion;
use crate::huespedes::huespedes::Huesped;

pub trait DatosDeEstancias {
    fn crear_estancia(
        &mut self,
        habitaciones: Vec<Habitacion>,
        huespedes: Vec<Huesped>,
        salida_prevista: DateTime<Local>,
    ) -> Result<String, String>;

    fn la_habitacion_esta_libre(&self, id_habitacion: &str) -> bool;
}
