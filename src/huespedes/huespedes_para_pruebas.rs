use super::datos_de_huespedes::DatosDeHuespedes;
use super::huespedes::Huesped;

pub struct HuespedesParaPruebas {
    datos: Vec<Huesped>,
}

impl HuespedesParaPruebas {
    pub fn new() -> HuespedesParaPruebas {
        HuespedesParaPruebas {
            datos: vec![
                Huesped::new(
                    "Benzirpi Mirvento",
                    crate::util::Nacionalidad::IT_Italy,
                    "99199199199",
                    "666777999",
                    "benzirpi@example.com",
                ),
                Huesped::new(
                    "Julliane Zirteni",
                    crate::util::Nacionalidad::IT_Italy,
                    "88188188188",
                    "666777888",
                    "julliane@example.com",
                ),
            ],
        }
    }
}

pub const ID_DE_UN_HUESPED_DE_PRUEBAS: &str = "99199199199";
pub const ID_DE_OTRO_HUESPED_DE_PRUEBAS: &str = "88188188188";

impl DatosDeHuespedes for HuespedesParaPruebas {
    fn get_huesped_con_id_interno(&self, id: uuid::Uuid) -> Result<Huesped, String> {
        let huesped = self.datos.iter().find(|x| x.get_id_interno() == id);
        match huesped {
            Some(h) => Ok(h.clone()),
            None => Err(format!("No existe huesped con id_interno {id}")),
        }
    }

    fn get_huesped(&self, numero_documento_id: &str) -> Result<Huesped, String> {
        let huesped = self
            .datos
            .iter()
            .find(|x| x.numero_documento_id == numero_documento_id);
        match huesped {
            Some(h) => Ok(h.clone()),
            None => Err(format!(
                "No existe huesped con documento_id {numero_documento_id}"
            )),
        }
    }
}
