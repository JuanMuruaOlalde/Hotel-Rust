use uuid::Uuid;

use super::datos_de_huespedes::DatosDeHuespedes;
use crate::util::Nacionalidad;

pub struct Huespedes<'a> {
    pub datos: &'a dyn DatosDeHuespedes,
}

#[derive(Clone)]
pub struct Huesped {
    id_interno: uuid::Uuid,
    pub nombre_y_apellidos: String,
    pub nacionalidad: Nacionalidad,
    pub numero_documento_id: String,
    pub telefono_de_contacto: String,
    pub correo_electronico: String,
}

impl Huesped {
    pub fn new(
        nombre_y_apellidos: &str,
        nacionalidad: Nacionalidad,
        numero_documento_id: &str,
        telefono_de_contacto: &str,
        correo_electronico: &str,
    ) -> Huesped {
        Huesped {
            id_interno: Uuid::now_v7(),
            nombre_y_apellidos: nombre_y_apellidos.to_string(),
            nacionalidad,
            numero_documento_id: numero_documento_id.to_string(),
            telefono_de_contacto: telefono_de_contacto.to_string(),
            correo_electronico: correo_electronico.to_string(),
        }
    }

    pub fn get_id_interno(&self) -> Uuid {
        self.id_interno.clone()
    }
}
