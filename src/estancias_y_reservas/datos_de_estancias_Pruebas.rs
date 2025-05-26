use chrono::{DateTime, Local};

use super::Estancia;
use super::datos_de_estancias::DatosDeEstancias;
use crate::habitaciones::Habitacion;
use crate::huespedes::Huesped;

pub struct DatosDeEstancias_Pruebas {
    pub datos: Vec<Estancia>,
}

impl Default for DatosDeEstancias_Pruebas {
    fn default() -> Self {
        Self {
            datos: Default::default(),
        }
    }
}

impl DatosDeEstancias for DatosDeEstancias_Pruebas {
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
