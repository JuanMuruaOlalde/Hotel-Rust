use super::DatosDeHuespedes;

pub struct DatosDeHuespedes_MariaDB {}

impl DatosDeHuespedes for DatosDeHuespedes_MariaDB {
    fn get_huesped_con_id_interno(&self, id: uuid::Uuid) -> Result<super::Huesped, String> {
        todo!()
    }

    fn get_huesped(&self, id: crate::util::DocumentoDeIdentidad) -> Result<super::Huesped, String> {
        todo!()
    }
}

impl DatosDeHuespedes_MariaDB {
    pub fn new() -> Self {
        Self {}
    }
}
