use uuid::Uuid;

use super::persistencia::DatosDeHuespedes;
use crate::util::CorreoElectronico;
use crate::util::{DocumentoDeIdentidad, Nacionalidad, Telefono};

pub struct Huespedes<T: DatosDeHuespedes> {
    pub datos: T,
}

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
    ) -> Huesped {
        Huesped {
            id_interno: Uuid::now_v7(),
            nombre_y_apellidos: String::from(nombre_y_apellidos),
            nacionalidad,
            numero_documento_id,
            telefono_de_contacto,
            correo_electronico,
        }
    }

    pub fn get_id_interno(&self) -> Uuid {
        self.id_interno.clone()
    }
}
