use super::DatosDeHuespedes;

pub struct DatosDeHuespedesMariaDB {}

impl DatosDeHuespedes for DatosDeHuespedesMariaDB {
    fn get_huesped_con_id_interno(&self, id: uuid::Uuid) -> Result<super::Huesped, String> {
        todo!()
    }

    fn get_huesped(&self, id: crate::util::DocumentoDeIdentidad) -> Result<super::Huesped, String> {
        todo!()
    }
}

impl DatosDeHuespedesMariaDB {
    pub fn new() -> Self {
        Self {}
    }
}
