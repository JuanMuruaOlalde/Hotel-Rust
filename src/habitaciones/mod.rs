pub mod datos_de_habitaciones;
pub mod datos_de_habitaciones_MariaDB;
pub mod datos_de_habitaciones_Pruebas;

use uuid::Uuid;

use datos_de_habitaciones::DatosDeHabitaciones;

pub struct Habitaciones<'a> {
    pub datos: &'a dyn DatosDeHabitaciones,
}

#[derive(Clone)]
pub struct Habitacion {
    id_interno: uuid::Uuid,
    pub nombre: String,
    pub tipo_habitacion: TipoDeHabitacion,
    pub tipo_baño: TipoDeBaño,
}

impl Habitacion {
    pub fn new(
        nombre: &str,
        tipo_habitacion: TipoDeHabitacion,
        tipo_baño: TipoDeBaño,
    ) -> Habitacion {
        Habitacion {
            id_interno: Uuid::now_v7(),
            nombre: nombre.to_string(),
            tipo_habitacion,
            tipo_baño,
        }
    }
}

#[derive(Clone)]
pub enum TipoDeHabitacion {
    SENCILLA,
    DOBLE,
    FAMILIAR,
    SUITE,
}

#[derive(Clone)]
pub enum TipoDeBaño {
    ConDUCHA,
    ConBAÑERA,
}
