use super::Habitacion;
use super::datos_de_habitaciones::DatosDeHabitaciones;

use super::TipoDeBaño;
use super::TipoDeHabitacion;

pub struct DatosDeHabitacionesPruebas {
    lista_de_habitaciones: Vec<Habitacion>,
}

impl DatosDeHabitacionesPruebas {
    pub fn new() -> DatosDeHabitacionesPruebas {
        DatosDeHabitacionesPruebas {
            lista_de_habitaciones: vec![
                Habitacion::new("101", TipoDeHabitacion::SENCILLA, TipoDeBaño::ConDUCHA),
                Habitacion::new("102", TipoDeHabitacion::DOBLE, TipoDeBaño::ConBAÑERA),
            ],
        }
    }
}

pub const ID_DE_UNA_HABITACION_DE_PRUEBAS: &str = "101";
pub const ID_DE_OTRA_HABITACION_DE_PRUEBAS: &str = "102";

impl DatosDeHabitaciones for DatosDeHabitacionesPruebas {
    fn get_habitacion(&self, nombre: &str) -> Result<Habitacion, String> {
        let habitacion = self
            .lista_de_habitaciones
            .iter()
            .find(|x| x.nombre == nombre);
        match habitacion {
            Some(h) => Ok(h.clone()),
            None => Err(format!("No existe la habitación {nombre}")),
        }
    }
}
