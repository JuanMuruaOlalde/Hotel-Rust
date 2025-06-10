use sqlx::{MySql, Pool};

use super::modelo::Estancia;
use super::persistencia_estancias::DatosDeEstancias;
use crate::habitaciones::modelo::Habitacion;
use crate::huespedes::modelo::Huesped;

pub struct DatosDeEstanciasMariaDB<'a> {
    conexion_con_la_bd: &'a Pool<MySql>,
}

impl<'a> DatosDeEstancias for DatosDeEstanciasMariaDB<'a> {
    fn crear_estancia(
        &mut self,
        habitaciones: Vec<Habitacion>,
        huespedes: Vec<Huesped>,
        salida_prevista: chrono::DateTime<chrono::Local>,
    ) -> Result<String, String> {
        todo!()
    }

    fn get_estancias(&self) -> Vec<Estancia> {
        todo!()
    }
}

impl<'a> DatosDeEstanciasMariaDB<'a> {
    pub fn new(conexion_con_la_bd: &'a Pool<MySql>) -> Self {
        Self { conexion_con_la_bd }
    }
}
