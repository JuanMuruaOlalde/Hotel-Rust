use sqlx::{MySql, Pool};

use super::datos_de_reservas::DatosDeReservas;

pub struct DatosDeReservasMariaDB<'a> {
    conexion_con_la_bd: &'a Pool<MySql>,
}

impl<'a> DatosDeReservas for DatosDeReservasMariaDB<'a> {}

impl<'a> DatosDeReservasMariaDB<'a> {
    pub fn new(conexion_con_la_bd: &'a Pool<MySql>) -> Self {
        Self { conexion_con_la_bd }
    }
}
