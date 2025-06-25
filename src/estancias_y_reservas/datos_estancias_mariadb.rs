use chrono::Local;
use sqlx::{MySql, Pool, Row};

use crate::estancias_y_reservas::modelo::HabitacionOcupada;

use super::datos_estancias::DatosDeEstancias;
use super::modelo::Estancia;

pub struct DatosDeEstanciasMariaDB<'a> {
    conexion_con_la_bd: &'a Pool<MySql>,
}

impl<'a> DatosDeEstanciasMariaDB<'a> {
    pub fn new(conexion_con_la_bd: &'a Pool<MySql>) -> Self {
        Self { conexion_con_la_bd }
    }
}

impl<'a> DatosDeEstancias for DatosDeEstanciasMariaDB<'a> {
    async fn guardar(&mut self, estancia: Estancia) -> Result<(), String> {
        let resultado = match estancia.salida_real {
            Some(fecha) => {
                sqlx::query(
                    "INSERT INTO estancias(id, entrada, salida_prevista, salida_real) VALUES (?, ?, ?, ?)",
                )
                .bind(sqlx::types::Uuid::from_u128(
                    estancia.get_id_interno().as_u128(),
                ))
                .bind(estancia.entrada_real)
                .bind(estancia.salida_prevista)
                .bind(fecha)
                .execute(self.conexion_con_la_bd)
                .await
            },
            None => {
                sqlx::query(
                    "INSERT INTO estancias(id, entrada, salida_prevista) VALUES (?, ?, ?)",
                )
                .bind(sqlx::types::Uuid::from_u128(
                    estancia.get_id_interno().as_u128(),
                ))
                .bind(estancia.entrada_real)
                .bind(estancia.salida_prevista)
                .execute(self.conexion_con_la_bd)
                .await
            }
        };
        match resultado {
            Ok(_) => {
                for habitacion in estancia.get_habitaciones() {
                    let resultado = sqlx::query(
                        "INSERT INTO estancias_habitaciones(id_estancia,id_habitacion) VALUES (?, ?)",
                    )
                    .bind(sqlx::types::Uuid::from_u128(estancia.get_id_interno().as_u128()))
                    .bind(sqlx::types::Uuid::from_u128(habitacion.get_id_interno().as_u128()))
                    .execute(self.conexion_con_la_bd)
                    .await;
                    match resultado {
                        Ok(_) => (),
                        Err(e) => return Err(e.to_string()),
                    }
                }
                for huesped in estancia.get_huespedes() {
                    let resultado = sqlx::query(
                        "INSERT INTO estancias_huespedes(id_estancia,id_huesped) VALUES (?, ?)",
                    )
                    .bind(sqlx::types::Uuid::from_u128(
                        estancia.get_id_interno().as_u128(),
                    ))
                    .bind(sqlx::types::Uuid::from_u128(
                        huesped.get_id_interno().as_u128(),
                    ))
                    .execute(self.conexion_con_la_bd)
                    .await;
                    match resultado {
                        Ok(_) => (),
                        Err(e) => return Err(e.to_string()),
                    }
                }
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }

    async fn get_habitaciones_ocupadas(&self) -> Result<Vec<HabitacionOcupada>, String> {
        let mut habitaciones_ocupadas: Vec<HabitacionOcupada> = Vec::new();
        let resultado = sqlx::query(
            "SELECT e.id, e.salida_prevista, eha.id_habitacion, h.nombre, e.entrada, e.salida_real
               FROM estancias e
               JOIN (estancias_habitaciones eha, habitaciones h)
                 ON (eha.id_estancia = e.id AND h.id = eha.id_habitacion)
              WHERE salida_real IS NULL",
        )
        .fetch_all(self.conexion_con_la_bd)
        .await;
        match resultado {
            Ok(datos) => {
                for registro in datos {
                    // MariaDB devuelve los UUID como BINARY (una ristra de bytes)
                    // pero en lugar de 16 bytes (un entero u128)
                    // es un string codificado en el formato que tenga la base de datos (utf8)
                    // Esto impide a sqlx reconocer correctamente lo que le devuelve .get
                    // y obliga a hacer estas conversiones manualmente.
                    // [Fix UUID parsing issue due to hyphenated format in MariaDB #2485](https://github.com/SeaQL/sea-orm/pull/2485)
                    let id_texto: String = match registro.try_get("id") {
                        Ok(bytes) => match String::from_utf8(bytes) {
                            Ok(texto) => texto,
                            Err(e) => String::new(),
                        },
                        Err(e) => String::new(),
                    };
                    let id_estancia = match uuid::Uuid::parse_str(&id_texto) {
                        Ok(id) => id,
                        Err(e) => uuid::Uuid::nil(),
                    };
                    let salida_prevista: sqlx::types::chrono::DateTime<Local> =
                        registro.get("salida_prevista");
                    let id_texto: String = match registro.try_get("id_habitacion") {
                        Ok(bytes) => match String::from_utf8(bytes) {
                            Ok(texto) => texto,
                            Err(e) => String::new(),
                        },
                        Err(e) => String::new(),
                    };
                    let id_habitacion = match uuid::Uuid::parse_str(&id_texto) {
                        Ok(id) => id,
                        Err(e) => uuid::Uuid::nil(),
                    };
                    let nombre_habitacion = registro.get("nombre");
                    let una_habitacion_ocupada = HabitacionOcupada {
                        id_interno_habitacion: id_habitacion,
                        nombre_habitacion,
                        id_interno_estancia: id_estancia,
                        salida_prevista,
                    };
                    habitaciones_ocupadas.push(una_habitacion_ocupada);
                }
            }
            Err(e) => return Err(e.to_string()),
        }
        Ok(habitaciones_ocupadas)
    }
}
