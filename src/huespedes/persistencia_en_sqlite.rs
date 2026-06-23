use sqlx::{Pool, Row, Sqlite};

use crate::comun::documento_de_identidad::DocumentoDeIdentidad;

use super::huespedes::Huesped;
use super::persistencia::DatosDeHuespedes;

pub struct DatosDeHuespedesSQLite<'a> {
    conexion_con_la_bd: &'a Pool<Sqlite>,
}

impl<'a> DatosDeHuespedesSQLite<'a> {
    async fn ajustar_y_ejecutar_select_para_get_huesped(
        &self,
        con_id: bool,
        id: uuid::Uuid,
        documento: DocumentoDeIdentidad,
    ) -> Result<Huesped, String> {
        let clausula_where: String;
        let dato_a_buscar: String;
        if con_id {
            clausula_where = String::from("WHERE id = ?");
            dato_a_buscar = id.to_string();
        } else {
            clausula_where = String::from("WHERE documento_de_id = ?");
            dato_a_buscar = documento.to_string();
        }
        let resultado = sqlx::query(
            &format!("SELECT id, nombre_y_apellidos, nacionalidad, documento_de_id, telefono, correo_e FROM huespedes {clausula_where}"),
        )
        .bind(dato_a_buscar)
        .fetch_optional(self.conexion_con_la_bd)
        .await;
        match resultado {
            Ok(datos) => match datos {
                Some(registro) => {
                    let id_texto: String = registro.get("id");
                    let id = match uuid::Uuid::parse_str(&id_texto) {
                        Ok(id) => id,
                        Err(e) => return Err(format!("Problemas al recuperar id: {e}")),
                    };
                    let nombre_y_apellidos: String = registro.get("nombre_y_apellidos");
                    let nacionalidad: String = registro.get("nacionalidad");
                    let documento_de_id: String = registro.get("documento_de_id");
                    let telefono: String = registro.get("telefono");
                    let correo_e: String = registro.get("correo_e");
                    let huesped = Huesped::from_persistencia(
                        id,
                        &nombre_y_apellidos,
                        &nacionalidad,
                        &documento_de_id,
                        &telefono,
                        &correo_e,
                    );
                    match huesped {
                        Ok(h) => Ok(h),
                        Err(e) => Err(format!("Problemas al convertir datos: {e}")),
                    }
                }
                None => Err(format!("No se ha encontrado el huesped {id}")),
            },
            Err(e) => Err(format!("Problemas al consultar la base de datos: {e}")),
        }
    }
}

impl<'a> DatosDeHuespedes for DatosDeHuespedesSQLite<'a> {
    async fn get_huesped_con_id_interno(&self, id: uuid::Uuid) -> Result<Huesped, String> {
        self.ajustar_y_ejecutar_select_para_get_huesped(true, id, DocumentoDeIdentidad::new(""))
            .await
    }

    async fn get_huesped(&self, id: DocumentoDeIdentidad) -> Result<Huesped, String> {
        self.ajustar_y_ejecutar_select_para_get_huesped(false, uuid::Uuid::nil(), id)
            .await
    }

    async fn guardar(&mut self, huesped: Huesped) -> Result<(), String> {
        let resultado = sqlx::query(
            "INSERT INTO huespedes(id, nombre_y_apellidos, nacionalidad, documento_de_id, telefono, correo_e) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(huesped.get_id_interno().to_string())
        .bind(huesped.nombre_y_apellidos)
        .bind(huesped.nacionalidad.to_string())
        .bind(huesped.numero_documento_id.to_string())
        .bind(huesped.telefono_de_contacto.to_string())
        .bind(huesped.correo_electronico.to_string())
        .execute(self.conexion_con_la_bd)
        .await;
        match resultado {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl<'a> DatosDeHuespedesSQLite<'a> {
    pub fn new(conexion_con_la_bd: &'a Pool<Sqlite>) -> Self {
        Self { conexion_con_la_bd }
    }
}
