use sqlx::{MySql, Pool};

use super::DatosDeHuespedes;

pub struct DatosDeHuespedesMariaDB<'a> {
    conexion_con_la_bd: &'a Pool<MySql>,
}

impl<'a> DatosDeHuespedes for DatosDeHuespedesMariaDB<'a> {
    fn get_huesped_con_id_interno(&self, id: uuid::Uuid) -> Result<super::Huesped, String> {
        todo!()
    }

    fn get_huesped(&self, id: crate::util::DocumentoDeIdentidad) -> Result<super::Huesped, String> {
        todo!()
    }
}

impl<'a> DatosDeHuespedesMariaDB<'a> {
    pub fn new(conexion_con_la_bd: &'a Pool<MySql>) -> Self {
        Self { conexion_con_la_bd }
    }
}
