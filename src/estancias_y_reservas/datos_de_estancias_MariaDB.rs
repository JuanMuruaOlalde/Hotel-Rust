use super::DatosDeEstancias;

pub struct DatosDeEstanciasMariaDB {}

impl DatosDeEstancias for DatosDeEstanciasMariaDB {
    fn crear_estancia(
        &mut self,
        habitaciones: Vec<crate::habitaciones::Habitacion>,
        huespedes: Vec<crate::huespedes::Huesped>,
        salida_prevista: chrono::DateTime<chrono::Local>,
    ) -> Result<String, String> {
        todo!()
    }

    fn la_habitacion_esta_libre(&self, id_habitacion: &str) -> bool {
        todo!()
    }
}

impl Default for DatosDeEstanciasMariaDB {
    fn default() -> Self {
        Self {}
    }
}

impl DatosDeEstanciasMariaDB {
    pub fn new() -> Self {
        Self {}
    }
}
