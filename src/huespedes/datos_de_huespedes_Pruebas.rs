use super::Huesped;
use super::datos_de_huespedes::DatosDeHuespedes;
use crate::util::{CorreoElectronico, DocumentoDeIdentidad, Telefono};

pub struct DatosDeHuespedes_Pruebas {
    datos: Vec<Huesped>,
}

impl DatosDeHuespedes_Pruebas {
    pub fn new() -> DatosDeHuespedes_Pruebas {
        DatosDeHuespedes_Pruebas {
            datos: vec![
                Huesped::new(
                    "Benzirpi Mirvento",
                    crate::util::Nacionalidad::IT_Italy,
                    DocumentoDeIdentidad::new(ID_DE_UN_HUESPED_DE_PRUEBAS),
                    Telefono::new("666777999"),
                    CorreoElectronico::new("benzirpi@example.com").unwrap(),
                ),
                Huesped::new(
                    "Julliane Zirteni",
                    crate::util::Nacionalidad::IT_Italy,
                    DocumentoDeIdentidad::new(ID_DE_OTRO_HUESPED_DE_PRUEBAS),
                    Telefono::new("666777888"),
                    CorreoElectronico::new("julliane@example.com").unwrap(),
                ),
            ],
        }
    }
}

pub const ID_DE_UN_HUESPED_DE_PRUEBAS: &str = "99199199199";
pub const ID_DE_OTRO_HUESPED_DE_PRUEBAS: &str = "88188188188";

impl DatosDeHuespedes for DatosDeHuespedes_Pruebas {
    fn get_huesped_con_id_interno(&self, id: uuid::Uuid) -> Result<Huesped, String> {
        let huesped = self.datos.iter().find(|x| x.get_id_interno() == id);
        match huesped {
            Some(h) => Ok(h.clone()),
            None => Err(format!("No existe huesped con id_interno {id}")),
        }
    }

    fn get_huesped(&self, id: DocumentoDeIdentidad) -> Result<Huesped, String> {
        let huesped = self.datos.iter().find(|x| x.numero_documento_id == id);
        match huesped {
            Some(h) => Ok(h.clone()),
            None => Err(format!("No existe huesped con documento_id {id}")),
        }
    }
}
