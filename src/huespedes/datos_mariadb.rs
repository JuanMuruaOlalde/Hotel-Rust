use sqlx::{MySql, Pool, Row};

use crate::comun::documento_de_identidad::DocumentoDeIdentidad;

use super::datos::DatosDeHuespedes;
use super::modelo::Huesped;

pub struct DatosDeHuespedesMariaDB<'a> {
    conexion_con_la_bd: &'a Pool<MySql>,
}

impl<'a> DatosDeHuespedesMariaDB<'a> {
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
                    // MariaDB devuelve los UUID como BINARY (una ristra de bytes)
                    // pero en lugar de 16 bytes (un entero u128)
                    // es un string codificado en el formato que tenga la base de datos (utf8)
                    // Esto impide a sqlx reconocer correctamente lo que le devuelve .get
                    // y obliga a hacer estas conversiones manualmente.
                    // [Fix UUID parsing issue due to hyphenated format in MariaDB #2485](https://github.com/SeaQL/sea-orm/pull/2485)
                    let id_texto: String = match registro.try_get("id") {
                        Ok(bytes) => match String::from_utf8(bytes) {
                            Ok(texto) => texto,
                            Err(e) => return Err(format!("Problemas al recuperar id: {e}")),
                        },
                        Err(e) => return Err(format!("Problemas al recuperar id: {e}")),
                    };
                    let id = match uuid::Uuid::parse_str(&id_texto) {
                        Ok(id) => id,
                        Err(e) => return Err(format!("Problemas al recuperar id: {e}")),
                    };
                    let huesped = Huesped::from_persistencia(
                        id,
                        registro.get("nombre_y_apellidos"),
                        registro.get("nacionalidad"),
                        registro.get("documento_de_id"),
                        registro.get("telefono"),
                        registro.get("correo_e"),
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

impl<'a> DatosDeHuespedes for DatosDeHuespedesMariaDB<'a> {
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
            "INSERT INTO huespedes(id, nombre_y_apellidos, nacionalidad, documento_de_id, telefono, correo_e ) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(sqlx::types::Uuid::from_u128(huesped.get_id_interno().as_u128()))
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

impl<'a> DatosDeHuespedesMariaDB<'a> {
    pub fn new(conexion_con_la_bd: &'a Pool<MySql>) -> Self {
        Self { conexion_con_la_bd }
    }
}
