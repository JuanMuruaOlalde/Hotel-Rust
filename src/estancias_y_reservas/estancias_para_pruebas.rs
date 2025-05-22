use chrono::{DateTime, Local};

use super::datos_de_estancias::DatosDeEstancias;
use super::estancias_y_reservas::Estancia;
use crate::habitaciones::habitaciones::Habitacion;
use crate::huespedes::huespedes::Huesped;

pub struct EstanciasParaPruebas {
    pub datos: Vec<Estancia>,
}

impl Default for EstanciasParaPruebas {
    fn default() -> Self {
        Self {
            datos: Default::default(),
        }
    }
}

impl DatosDeEstancias for EstanciasParaPruebas {
    fn crear_estancia(
        &mut self,
        habitaciones: Vec<Habitacion>,
        huespedes: Vec<Huesped>,
        salida_prevista: DateTime<Local>,
    ) -> Result<String, String> {
        let estancia = Estancia {
            habitaciones: habitaciones,
            huespedes: huespedes,
            entrada_real: Local::now(),
            salida_prevista: salida_prevista,
            salida_real: None,
        };
        self.datos.push(estancia);
        Ok(String::from("Estancia creada correctamente"))
    }

    fn la_habitacion_esta_libre(&self, nombre: &str) -> bool {
        if (self.datos.iter().any(|estancia| {
            estancia
                .get_habitaciones_ocupadas()
                .iter()
                .any(|habitacion| habitacion.nombre == nombre)
        })) {
            false
        } else {
            true
        }
    }
}
