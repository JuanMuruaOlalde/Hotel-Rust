use sqlx::{MySql, Pool};

use super::datos_de_habitaciones::DatosDeHabitaciones;
pub struct DatosDeHabitacionesMariaDB<'a> {
    conexion_con_la_bd: &'a Pool<MySql>,
}

impl<'a> DatosDeHabitaciones for DatosDeHabitacionesMariaDB<'a> {
    fn get_habitacion(&self, nombre: &str) -> Result<super::Habitacion, String> {
        todo!()
    }
}

impl<'a> DatosDeHabitacionesMariaDB<'a> {
    pub fn new(conexion_con_la_bd: &'a Pool<MySql>) -> Self {
        Self { conexion_con_la_bd }
    }
}
