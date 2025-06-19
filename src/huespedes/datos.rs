use crate::util::DocumentoDeIdentidad;

use super::modelo::Huesped;

pub trait DatosDeHuespedes {
    async fn get_huesped_con_id_interno(&self, id: uuid::Uuid) -> Result<Huesped, String>;
    async fn get_huesped(&self, id: DocumentoDeIdentidad) -> Result<Huesped, String>;
    async fn guardar(&mut self, huesped: Huesped) -> Result<(), String>;
}
