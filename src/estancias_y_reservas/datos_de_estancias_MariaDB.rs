use super::DatosDeEstancias;

pub struct DatosDeEstancias_MariaDB {}

impl DatosDeEstancias for DatosDeEstancias_MariaDB {
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

impl Default for DatosDeEstancias_MariaDB {
    fn default() -> Self {
        Self {}
    }
}

impl DatosDeEstancias_MariaDB {
    pub fn new() -> Self {
        Self {}
    }

}
