use sqlx::{Pool, Row, Sqlite};

use super::habitaciones::Habitacion;
use super::persistencia::DatosDeHabitaciones;

pub struct DatosDeHabitacionesSQLite<'a> {
    conexion_con_la_bd: &'a Pool<Sqlite>,
}

impl<'a> DatosDeHabitaciones for DatosDeHabitacionesSQLite<'a> {
    async fn get_habitacion(&self, nombre: &str) -> Result<Habitacion, String> {
        let resultado = sqlx::query(
            "SELECT id, nombre, tipo_de_habitacion, \"tipo_de_baño\" FROM habitaciones WHERE nombre = ?",
        )
        .bind(nombre)
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
                    let nombre: String = registro.get("nombre");
                    let tipo_de_habitacion: String = registro.get("tipo_de_habitacion");
                    let tipo_de_baño: String = registro.get("tipo_de_baño");
                    let habitacion = Habitacion::from_persistencia(
                        id,
                        &nombre,
                        &tipo_de_habitacion,
                        &tipo_de_baño,
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
            "INSERT INTO habitaciones(id, nombre, tipo_de_habitacion, \"tipo_de_baño\") VALUES (?, ?, ?, ?)",
        )
        .bind(habitacion.get_id_interno().to_string())
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

impl<'a> DatosDeHabitacionesSQLite<'a> {
    pub fn new(conexion_con_la_bd: &'a Pool<Sqlite>) -> Self {
        Self { conexion_con_la_bd }
    }
}
