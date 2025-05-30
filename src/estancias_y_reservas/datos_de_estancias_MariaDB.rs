use sqlx::{MySql, Pool};

use super::datos_de_estancias::DatosDeEstancias;

pub struct DatosDeEstanciasMariaDB<'a> {
    conexion_con_la_bd: &'a Pool<MySql>,
}

impl<'a> DatosDeEstancias for DatosDeEstanciasMariaDB<'a> {
    fn crear_estancia(
        &mut self,
        habitaciones: Vec<crate::habitaciones::Habitacion>,
        huespedes: Vec<crate::huespedes::Huesped>,
        salida_prevista: chrono::DateTime<chrono::Local>,
    ) -> Result<String, String> {
        todo!()
    }

    fn get_estancias(&self) -> Vec<super::Estancia> {
        todo!()
    }
}

impl<'a> DatosDeEstanciasMariaDB<'a> {
    pub fn new(conexion_con_la_bd: &'a Pool<MySql>) -> Self {
        Self { conexion_con_la_bd }
    }
}
