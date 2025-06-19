use super::modelo::Habitacion;

pub trait DatosDeHabitaciones {
    async fn get_habitacion(&self, nombre: &str) -> Result<Habitacion, String>;
    async fn guardar(&mut self, habitacion: Habitacion) -> Result<(), String>;
}
