use super::modelo::Habitacion;

pub trait DatosDeHabitaciones {
    fn get_habitacion(&self, nombre: &str) -> Result<Habitacion, String>;
    fn put_habitacion(&self, habitacion: Habitacion) -> Result<Habitacion, String>;
}
