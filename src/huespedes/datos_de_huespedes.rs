use crate::util::DocumentoDeIdentidad;

use super::huespedes::Huesped;

pub trait DatosDeHuespedes {
    fn get_huesped_con_id_interno(&self, id: uuid::Uuid) -> Result<Huesped, String>;
    fn get_huesped(&self, id: DocumentoDeIdentidad) -> Result<Huesped, String>;
}
