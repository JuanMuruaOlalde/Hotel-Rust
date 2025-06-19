use std::str::FromStr;

use uuid::Uuid;

use super::datos::DatosDeHuespedes;
use crate::util::CorreoElectronico;
use crate::util::{DocumentoDeIdentidad, Nacionalidad, Telefono};

pub struct Huespedes<T: DatosDeHuespedes> {
    datos: T,
}

impl<T: DatosDeHuespedes> Huespedes<T> {
    pub fn new(datos: T) -> Self {
        Self { datos }
    }

    pub async fn añadir_una_persona_nueva(
        &mut self,
        nombre_y_apellidos: &str,
        nacionalidad: Nacionalidad,
        numero_documento_id: DocumentoDeIdentidad,
        telefono_de_contacto: Telefono,
        correo_electronico: CorreoElectronico,
    ) -> Result<(), String> {
        let huesped = Huesped::new(
            nombre_y_apellidos,
            nacionalidad,
            numero_documento_id,
            telefono_de_contacto,
            correo_electronico,
        );
        match self.datos.guardar(huesped).await {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn get_huesped(&self, id: DocumentoDeIdentidad) -> Result<Huesped, String> {
        self.datos.get_huesped(id).await
    }
}

// Esta es la información básica imprescindible de una persona huesped,
// si hubiera más información irá en otro/s struct secundario/s.
#[derive(Clone)]
pub struct Huesped {
    id_interno: uuid::Uuid,
    pub nombre_y_apellidos: String,
    pub nacionalidad: Nacionalidad,
    pub numero_documento_id: DocumentoDeIdentidad,
    pub telefono_de_contacto: Telefono,
    pub correo_electronico: CorreoElectronico,
}

impl Huesped {
    pub fn new(
        nombre_y_apellidos: &str,
        nacionalidad: Nacionalidad,
        numero_documento_id: DocumentoDeIdentidad,
        telefono_de_contacto: Telefono,
        correo_electronico: CorreoElectronico,
    ) -> Self {
        Self {
            id_interno: Uuid::now_v7(),
            nombre_y_apellidos: String::from(nombre_y_apellidos),
            nacionalidad,
            numero_documento_id,
            telefono_de_contacto,
            correo_electronico,
        }
    }

    pub fn get_id_interno(&self) -> uuid::Uuid {
        self.id_interno
    }

    pub fn from_persistencia(
        id_interno: uuid::Uuid,
        nombre_y_apellidos: &str,
        nacionalidad: &str,
        documento_de_id: &str,
        telefono: &str,
        correo_e: &str,
    ) -> Result<Huesped, String> {
        let nombre_y_apellidos = nombre_y_apellidos.to_string();
        let nacionalidad = match Nacionalidad::from_str(nacionalidad) {
            Ok(tipo) => tipo,
            Err(e) => return Err(e),
        };
        let numero_documento_id = DocumentoDeIdentidad::new(documento_de_id);
        let telefono_de_contacto = Telefono::new(telefono);
        let correo_electronico = match CorreoElectronico::new(correo_e) {
            Ok(c) => c,
            Err(e) => return Err(e.to_string()),
        };
        Ok(Huesped {
            id_interno,
            nombre_y_apellidos,
            nacionalidad,
            numero_documento_id,
            telefono_de_contacto,
            correo_electronico,
        })
    }
}
