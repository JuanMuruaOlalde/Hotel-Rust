use std::str::FromStr;
use uuid::Uuid;

use super::datos::DatosDeHabitaciones;

pub struct Habitaciones<T: DatosDeHabitaciones> {
    datos: T,
}

impl<T: DatosDeHabitaciones> Habitaciones<T> {
    pub fn new(datos: T) -> Self {
        Self { datos }
    }

    pub async fn añadir_una_nueva(
        &mut self,
        id_publico_nombre: &str,
        tipo_de_habitacion: TipoDeHabitacion,
        tipo_de_baño: TipoDeBaño,
    ) -> Result<(), String> {
        let habitacion = Habitacion::new(id_publico_nombre, tipo_de_habitacion, tipo_de_baño);
        match self.datos.guardar(habitacion).await {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn get_habitacion(&self, nombre: &str) -> Result<Habitacion, String> {
        self.datos.get_habitacion(nombre).await
    }
}

// Esta es la información básica imprescindible de una habitación,
// si hubiera más información irá en otro/s struct secundario/s.
#[derive(Clone)]
pub struct Habitacion {
    id_interno: uuid::Uuid,
    pub id_publico_nombre: String,
    pub tipo_de_habitacion: TipoDeHabitacion,
    pub tipo_de_baño: TipoDeBaño,
}

impl Habitacion {
    pub fn new(
        id_publico_nombre: &str,
        tipo_de_habitacion: TipoDeHabitacion,
        tipo_de_baño: TipoDeBaño,
    ) -> Habitacion {
        Habitacion {
            id_interno: Uuid::now_v7(),
            id_publico_nombre: id_publico_nombre.to_string(),
            tipo_de_habitacion,
            tipo_de_baño,
        }
    }

    pub fn get_id_interno(&self) -> uuid::Uuid {
        self.id_interno.clone()
    }

    pub fn from_persistencia(
        id_interno: uuid::Uuid,
        id_publico: &str,
        tipo_de_habitacion: &str,
        tipo_de_baño: &str,
    ) -> Result<Habitacion, String> {
        let nombre = id_publico.to_string();
        let tipo_de_habitacion = match TipoDeHabitacion::from_str(tipo_de_habitacion) {
            Ok(tipo) => tipo,
            Err(e) => return Err(e),
        };
        let tipo_de_baño = match TipoDeBaño::from_str(tipo_de_baño) {
            Ok(tipo) => tipo,
            Err(e) => return Err(e),
        };
        Ok(Habitacion {
            id_interno,
            id_publico_nombre: nombre,
            tipo_de_habitacion,
            tipo_de_baño,
        })
    }
}

#[derive(Clone)]
pub enum TipoDeHabitacion {
    SENCILLA,
    DOBLE,
    FAMILIAR,
    SUITE,
}
impl std::fmt::Display for TipoDeHabitacion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TipoDeHabitacion::SENCILLA => write!(f, "{}", "SENCILLA"),
            TipoDeHabitacion::DOBLE => write!(f, "{}", "DOBLE"),
            TipoDeHabitacion::FAMILIAR => write!(f, "{}", "FAMILIAR"),
            TipoDeHabitacion::SUITE => write!(f, "{}", "SUITE"),
        }
    }
}
impl std::str::FromStr for TipoDeHabitacion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SENCILLA" => Ok(TipoDeHabitacion::SENCILLA),
            "DOBLE" => Ok(TipoDeHabitacion::DOBLE),
            "FAMILIAR" => Ok(TipoDeHabitacion::FAMILIAR),
            "SUITE" => Ok(TipoDeHabitacion::SUITE),
            _ => Err(format!("[{s}] no es un tipo de habitacion válido.")),
        }
    }
}

#[derive(Clone)]
pub enum TipoDeBaño {
    ConDUCHA,
    ConBAÑERA,
}
impl std::fmt::Display for TipoDeBaño {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TipoDeBaño::ConDUCHA => write!(f, "{}", "ConDUCHA"),
            TipoDeBaño::ConBAÑERA => write!(f, "{}", "ConBAÑERA"),
        }
    }
}
impl std::str::FromStr for TipoDeBaño {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ConDUCHA" => Ok(TipoDeBaño::ConDUCHA),
            "ConBAÑERA" => Ok(TipoDeBaño::ConBAÑERA),
            _ => Err(format!("[{s}] no es un tipo de baño válido.")),
        }
    }
}
