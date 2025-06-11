use uuid::Uuid;

use super::persistencia::DatosDeHuespedes;
use crate::util::CorreoElectronico;
use crate::util::{DocumentoDeIdentidad, Nacionalidad, Telefono};

pub struct Huespedes<T: DatosDeHuespedes> {
    datos: T,
}

impl<T: DatosDeHuespedes> Huespedes<T> {
    pub fn new(datos: T) -> Self {
        Self { datos }
    }

    pub fn añadir_una_persona_nueva(
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
        match self.datos.guardar(huesped) {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn get_huesped(&self, id: DocumentoDeIdentidad) -> Result<Huesped, String> {
        self.datos.get_huesped(id)
    }
}

// Esta es la información básica imprescindible de una persona huesped,
// si hubiera más información irá en otro/s struct secundario/s.
#[derive(Clone)]
pub struct Huesped {
    pub id_interno: uuid::Uuid,
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
}
