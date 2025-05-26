use super::DatosDeHabitaciones;
pub struct DatosDeHabitaciones_MariaDB {}

impl DatosDeHabitaciones for DatosDeHabitaciones_MariaDB {
    fn get_habitacion(&self, nombre: &str) -> Result<super::Habitacion, String> {
        todo!()
    }
}

impl DatosDeHabitaciones_MariaDB {
    pub fn new() -> Self {
        Self {}
    }
}
