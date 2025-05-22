use super::huespedes::Huesped;

pub trait DatosDeHuespedes {
    fn get_huesped_con_id_interno(&self, id: uuid::Uuid) -> Result<Huesped, String>;
    fn get_huesped(&self, numero_documento_id: &str) -> Result<Huesped, String>;
}
