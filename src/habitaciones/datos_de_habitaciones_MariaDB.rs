use super::DatosDeHabitaciones;
pub struct DatosDeHabitacionesMariaDB {}

impl DatosDeHabitaciones for DatosDeHabitacionesMariaDB {
    fn get_habitacion(&self, nombre: &str) -> Result<super::Habitacion, String> {
        todo!()
    }
}

impl DatosDeHabitacionesMariaDB {
    pub fn new() -> Self {
        Self {}
    }
}
