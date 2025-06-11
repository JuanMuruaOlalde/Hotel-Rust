use sqlx::{MySql, Pool};

use super::modelo::Estancia;
use super::persistencia_estancias::DatosDeEstancias;

pub struct DatosDeEstanciasMariaDB<'a> {
    conexion_con_la_bd: &'a Pool<MySql>,
}

impl<'a> DatosDeEstanciasMariaDB<'a> {
    pub fn new(conexion_con_la_bd: &'a Pool<MySql>) -> Self {
        Self { conexion_con_la_bd }
    }
}

impl<'a> DatosDeEstancias for DatosDeEstanciasMariaDB<'a> {
    fn guardar(&mut self, estancia: Estancia) -> Result<(), String> {
        todo!()
    }

    fn get_estancias_activas(&self) -> Vec<Estancia> {
        todo!()
    }
}
