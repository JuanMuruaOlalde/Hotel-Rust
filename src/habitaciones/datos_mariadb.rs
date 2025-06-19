use sqlx::{MySql, Pool, Row};

use super::datos::DatosDeHabitaciones;
use super::modelo::Habitacion;
pub struct DatosDeHabitacionesMariaDB<'a> {
    conexion_con_la_bd: &'a Pool<MySql>,
}

impl<'a> DatosDeHabitaciones for DatosDeHabitacionesMariaDB<'a> {
    async fn get_habitacion(&self, nombre: &str) -> Result<Habitacion, String> {
        let resultado = sqlx::query(
            "SELECT id, nombre, tipo_de_habitacion, tipo_de_baño FROM habitaciones WHERE nombre = ?",
        )
        .bind(nombre)
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
                    let habitacion = Habitacion::from_persistencia(
                        id,
                        registro.get("nombre"),
                        registro.get("tipo_de_habitacion"),
                        registro.get("tipo_de_baño"),
                    );
                    match habitacion {
                        Ok(h) => Ok(h),
                        Err(e) => Err(format!("Problemas al convertir datos: {e}")),
                    }
                }
                None => Err(format!("No se ha encontrado la habitacion {nombre}")),
            },
            Err(e) => Err(format!("Problemas al consultar la base de datos: {e}")),
        }
    }

    async fn guardar(&mut self, habitacion: Habitacion) -> Result<(), String> {
        let resultado = sqlx::query(
            "INSERT INTO habitaciones(id, nombre, tipo_de_habitacion, tipo_de_baño) VALUES (?, ?, ?, ?)",
        )
        .bind(sqlx::types::Uuid::from_u128(habitacion.get_id_interno().as_u128()))
        .bind(habitacion.id_publico_nombre)
        .bind(habitacion.tipo_de_habitacion.to_string())
        .bind(habitacion.tipo_de_baño.to_string())
        .execute(self.conexion_con_la_bd)
        .await;
        match resultado {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl<'a> DatosDeHabitacionesMariaDB<'a> {
    pub fn new(conexion_con_la_bd: &'a Pool<MySql>) -> Self {
        Self { conexion_con_la_bd }
    }
}
