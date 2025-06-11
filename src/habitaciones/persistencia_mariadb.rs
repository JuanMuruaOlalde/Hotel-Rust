use sqlx::{MySql, Pool};

use super::modelo::Habitacion;
use super::persistencia::DatosDeHabitaciones;
pub struct DatosDeHabitacionesMariaDB<'a> {
    conexion_con_la_bd: &'a Pool<MySql>,
}

impl<'a> DatosDeHabitaciones for DatosDeHabitacionesMariaDB<'a> {
    fn get_habitacion(&self, nombre: &str) -> Result<Habitacion, String> {
        todo!()
        // let resultado = sqlx::query("SELECT * FROM habitaciones WHERE nombre = ?")
        //     .bind(nombre)
        //     .fetch_optional(self.conexion_con_la_bd)
        //     .await;
        // match resultado {
        //     Ok(datos) => match datos {
        //         Some(registro) => {
        //             let habitacion = Habitacion::from_persistencia(
        //                 registro.id,
        //                 registro.nombre,
        //                 registro.tipo_habitacion,
        //                 registro.tipo_baño,
        //             );
        //             match habitacion {
        //                 Ok(h) => Ok(h),
        //                 Err(e) => Err(format!("Problemas al convertir datos: {e}")),
        //             }
        //         }
        //         None => Err(format!("No se ha encontrado la habitacion {nombre}")),
        //     },
        //     Err(e) => Err(format!("Problemas al consultar la base de datos: {e}")),
        // }
    }

    fn guardar(&mut self, habitacion: Habitacion) -> Result<(), String> {
        todo!()
    }
}

impl<'a> DatosDeHabitacionesMariaDB<'a> {
    pub fn new(conexion_con_la_bd: &'a Pool<MySql>) -> Self {
        Self { conexion_con_la_bd }
    }
}
