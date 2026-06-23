use sqlx::{Pool, Sqlite};

use super::persistencia_de_reservas::DatosDeReservas;

pub struct DatosDeReservasSQLite<'a> {
    conexion_con_la_bd: &'a Pool<Sqlite>,
}

impl<'a> DatosDeReservas for DatosDeReservasSQLite<'a> {}

impl<'a> DatosDeReservasSQLite<'a> {
    pub fn new(conexion_con_la_bd: &'a Pool<Sqlite>) -> Self {
        Self { conexion_con_la_bd }
    }
}
