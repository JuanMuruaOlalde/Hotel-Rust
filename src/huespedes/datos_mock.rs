use crate::comun::documento_de_identidad::DocumentoDeIdentidad;

use super::datos::DatosDeHuespedes;
use super::modelo::Huesped;

pub struct DatosDeHuespedesPruebas {
    lista_de_huespedes: Box<Vec<Huesped>>,
}

impl DatosDeHuespedesPruebas {
    pub fn new() -> Self {
        Self {
            lista_de_huespedes: Box::new(Vec::new()),
        }
    }
}

impl DatosDeHuespedes for DatosDeHuespedesPruebas {
    async fn get_huesped_con_id_interno(&self, id: uuid::Uuid) -> Result<Huesped, String> {
        let huesped = self
            .lista_de_huespedes
            .iter()
            .find(|x| x.get_id_interno() == id);
        match huesped {
            Some(h) => Ok(h.clone()),
            None => Err(format!("No existe huesped con id_interno {id}")),
        }
    }

    async fn get_huesped(&self, id: DocumentoDeIdentidad) -> Result<Huesped, String> {
        let huesped = self
            .lista_de_huespedes
            .iter()
            .find(|x| x.numero_documento_id == id);
        match huesped {
            Some(h) => Ok(h.clone()),
            None => Err(format!("No existe huesped con documento_id {id}")),
        }
    }

    async fn guardar(&mut self, huesped: Huesped) -> Result<(), String> {
        self.lista_de_huespedes.push(huesped);
        Ok(())
    }
}
