use uuid::Uuid;

use super::datos_de_habitaciones::DatosDeHabitaciones;
use super::habitaciones::Habitacion;

use super::habitaciones::TipoDeBaño;
use super::habitaciones::TipoDeHabitacion;

pub struct HabitacionesParaPruebas {
    lista_de_habitaciones: Vec<Habitacion>,
}

impl HabitacionesParaPruebas {
    pub fn new() -> HabitacionesParaPruebas {
        HabitacionesParaPruebas {
            lista_de_habitaciones: vec![
                Habitacion::new("101", TipoDeHabitacion::SENCILLA, TipoDeBaño::ConDUCHA),
                Habitacion::new("102", TipoDeHabitacion::DOBLE, TipoDeBaño::ConBAÑERA),
            ],
        }
    }
}

pub const ID_DE_UNA_HABITACION_DE_PRUEBAS: &str = "101";
pub const ID_DE_OTRA_HABITACION_DE_PRUEBAS: &str = "102";

impl DatosDeHabitaciones for HabitacionesParaPruebas {
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
