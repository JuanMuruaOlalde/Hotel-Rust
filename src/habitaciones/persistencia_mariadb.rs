use sqlx::{MySql, Pool};

use super::modelo::Habitacion;
use super::persistencia::DatosDeHabitaciones;
pub struct DatosDeHabitacionesMariaDB<'a> {
    conexion_con_la_bd: &'a Pool<MySql>,
}

impl<'a> DatosDeHabitaciones for DatosDeHabitacionesMariaDB<'a> {
    fn get_habitacion(&self, nombre: &str) -> Result<Habitacion, String> {
        todo!()
    }

    fn put_habitacion(&self, habitacion: Habitacion) -> Result<Habitacion, String> {
        todo!()
    }
}

impl<'a> DatosDeHabitacionesMariaDB<'a> {
    pub fn new(conexion_con_la_bd: &'a Pool<MySql>) -> Self {
        Self { conexion_con_la_bd }
    }
}
