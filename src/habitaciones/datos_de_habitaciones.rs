use super::Habitacion;

pub trait DatosDeHabitaciones {
    fn get_habitacion(&self, nombre: &str) -> Result<Habitacion, String>;
}
