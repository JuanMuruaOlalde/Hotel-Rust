use chrono::{DateTime, Local};

use super::modelo::Estancia;
use super::persistencia_estancias::DatosDeEstancias;
use crate::habitaciones::modelo::Habitacion;
use crate::huespedes::modelo::Huesped;

pub struct DatosDeEstanciasPruebas {
    pub datos: Vec<Estancia>,
}

impl Default for DatosDeEstanciasPruebas {
    fn default() -> Self {
        Self {
            datos: Default::default(),
        }
    }
}

impl DatosDeEstancias for DatosDeEstanciasPruebas {
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

    fn get_estancias(&self) -> Vec<Estancia> {
        self.datos.clone()
    }
}
