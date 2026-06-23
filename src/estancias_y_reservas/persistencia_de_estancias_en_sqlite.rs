use chrono::{Local, Utc};
use sqlx::{Pool, Row, Sqlite};

use crate::estancias_y_reservas::estancias_y_reservas::HabitacionOcupada;

use super::estancias_y_reservas::Estancia;
use super::persistencia_de_estancias::DatosDeEstancias;

pub struct DatosDeEstanciasSQLite<'a> {
    conexion_con_la_bd: &'a Pool<Sqlite>,
}

impl<'a> DatosDeEstanciasSQLite<'a> {
    pub fn new(conexion_con_la_bd: &'a Pool<Sqlite>) -> Self {
        Self { conexion_con_la_bd }
    }
}

impl<'a> DatosDeEstancias for DatosDeEstanciasSQLite<'a> {
    async fn guardar(&mut self, estancia: Estancia) -> Result<(), String> {
        let resultado = match estancia.salida_real {
            Some(fecha) => {
                sqlx::query(
                    "INSERT INTO estancias(id, entrada, salida_prevista, salida_real) VALUES (?, ?, ?, ?)",
                )
                .bind(estancia.get_id_interno().to_string())
                .bind(estancia.entrada_real.with_timezone(&Utc))
                .bind(estancia.salida_prevista.with_timezone(&Utc))
                .bind(fecha.with_timezone(&Utc))
                .execute(self.conexion_con_la_bd)
                .await
            }
            None => {
                sqlx::query(
                    "INSERT INTO estancias(id, entrada, salida_prevista) VALUES (?, ?, ?)",
                )
                .bind(estancia.get_id_interno().to_string())
                .bind(estancia.entrada_real.with_timezone(&Utc))
                .bind(estancia.salida_prevista.with_timezone(&Utc))
                .execute(self.conexion_con_la_bd)
                .await
            }
        };
        match resultado {
            Ok(_) => {
                for habitacion in estancia.get_habitaciones() {
                    let resultado = sqlx::query(
                        "INSERT INTO estancias_habitaciones(id_estancia, id_habitacion) VALUES (?, ?)",
                    )
                    .bind(estancia.get_id_interno().to_string())
                    .bind(habitacion.get_id_interno().to_string())
                    .execute(self.conexion_con_la_bd)
                    .await;
                    match resultado {
                        Ok(_) => (),
                        Err(e) => return Err(e.to_string()),
                    }
                }
                for huesped in estancia.get_huespedes() {
                    let resultado = sqlx::query(
                        "INSERT INTO estancias_huespedes(id_estancia, id_huesped) VALUES (?, ?)",
                    )
                    .bind(estancia.get_id_interno().to_string())
                    .bind(huesped.get_id_interno().to_string())
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
               JOIN estancias_habitaciones eha ON eha.id_estancia = e.id
               JOIN habitaciones h ON h.id = eha.id_habitacion
              WHERE salida_real IS NULL",
        )
        .fetch_all(self.conexion_con_la_bd)
        .await;
        match resultado {
            Ok(datos) => {
                for registro in datos {
                    let id_texto: String = registro.get("id");
                    let id_estancia = match uuid::Uuid::parse_str(&id_texto) {
                        Ok(id) => id,
                        Err(_) => uuid::Uuid::nil(),
                    };
                    let salida_prevista_utc: sqlx::types::chrono::DateTime<Utc> =
                        registro.get("salida_prevista");
                    let salida_prevista = salida_prevista_utc.with_timezone(&Local);
                    let id_texto: String = registro.get("id_habitacion");
                    let id_habitacion = match uuid::Uuid::parse_str(&id_texto) {
                        Ok(id) => id,
                        Err(_) => uuid::Uuid::nil(),
                    };
                    let nombre_habitacion: String = registro.get("nombre");
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
