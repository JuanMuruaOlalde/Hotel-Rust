use chrono::{DateTime, Local};

use super::persistencia_estancias::DatosDeEstancias;
use super::persistencia_reservas::DatosDeReservas;
use crate::habitaciones::modelo::Habitacion;
use crate::huespedes::modelo::Huesped;

pub struct EstanciasYReservas<E, R>
where
    E: DatosDeEstancias,
    R: DatosDeReservas,
{
    estancias: E,
    reservas: R,
}

impl<E, R> EstanciasYReservas<E, R>
where
    E: DatosDeEstancias,
    R: DatosDeReservas,
{
    pub fn new(datos_estancias: E, datos_reservas: R) -> Self {
        Self {
            estancias: datos_estancias,
            reservas: datos_reservas,
        }
    }
    pub fn crear_estancia(
        &mut self,
        habitaciones: Vec<Habitacion>,
        huespedes: Vec<Huesped>,
        salida_prevista: DateTime<Local>,
    ) -> Result<(), String> {
        let estancia = Estancia {
            habitaciones: habitaciones,
            huespedes: huespedes,
            entrada_real: Local::now(),
            salida_prevista: salida_prevista,
            salida_real: None,
        };
        match self.estancias.guardar(estancia) {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn la_habitacion_esta_libre(&self, nombre: &str) -> bool {
        if self
            .estancias
            .get_estancias_activas()
            .iter()
            .any(|estancia| {
                estancia
                    .get_habitaciones()
                    .iter()
                    .any(|habitacion| habitacion.id_publico_nombre == nombre)
            })
        {
            return false;
        } else {
            //TODO, pendiente comprobar las reservas antes de dar la habitacion por libre.
            if false {
                return false;
            }
        }
        return true;
    }
}

// Esta es la información básica imprescindible de una estancia,
// si hubiera más información iria en otro/s struct secundario/s.
#[derive(Clone)]
pub struct Estancia {
    pub habitaciones: Vec<Habitacion>,
    pub huespedes: Vec<Huesped>,
    pub entrada_real: DateTime<Local>,
    pub salida_prevista: DateTime<Local>,
    pub salida_real: Option<DateTime<Local>>,
}

impl Estancia {
    pub fn get_habitaciones(&self) -> Vec<Habitacion> {
        self.habitaciones.clone()
    }
}

// Esta es la información básica imprescindible de una reserva,
// si hubiera más información irá en otro/s struct secundario/s.
#[derive(Clone)]
pub struct Reserva {
    //TODO, pendiente de implementar
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Local};

    use super::*;
    use crate::habitaciones::modelo::{Habitaciones, TipoDeBaño, TipoDeHabitacion};
    use crate::habitaciones::persistencia_mock::DatosDeHabitacionesPruebas;
    use crate::huespedes::modelo::Huespedes;
    use crate::huespedes::persistencia_mock::DatosDeHuespedesPruebas;
    use crate::util::{CorreoElectronico, DocumentoDeIdentidad, Nacionalidad, Telefono};

    use crate::estancias_y_reservas::persistencia_estancias_mock::DatosDeEstanciasPruebas;
    use crate::estancias_y_reservas::persistencia_reservas_mock::DatosDeReservasPruebas;

    #[test]
    fn al_asignar_habitaciones_a_una_estancia_estas_quedan_ocupadas() {
        let mut habitaciones = Habitaciones::new(DatosDeHabitacionesPruebas::new());
        habitaciones
            .añadir_una_nueva("101", TipoDeHabitacion::SENCILLA, TipoDeBaño::ConDUCHA)
            .unwrap();
        habitaciones
            .añadir_una_nueva("102", TipoDeHabitacion::DOBLE, TipoDeBaño::ConBAÑERA)
            .unwrap();
        let habitacion01 = habitaciones.get_habitacion("101").unwrap();
        let habitacion02 = habitaciones.get_habitacion("102").unwrap();

        let mut huespedes = Huespedes::new(DatosDeHuespedesPruebas::new());
        huespedes
            .añadir_una_persona_nueva(
                "Benzirpi Mirvento",
                Nacionalidad::IT_Italy,
                DocumentoDeIdentidad::new("99199199199"),
                Telefono::new("666777999"),
                CorreoElectronico::new("benzirpi@example.com").unwrap(),
            )
            .unwrap();
        huespedes
            .añadir_una_persona_nueva(
                "Julliane Zirteni",
                Nacionalidad::IT_Italy,
                DocumentoDeIdentidad::new("88188188188"),
                Telefono::new("666777888"),
                CorreoElectronico::new("julliane@example.com").unwrap(),
            )
            .unwrap();
        let un_huesped = huespedes
            .get_huesped(DocumentoDeIdentidad::new("99199199199"))
            .unwrap();
        let una_huesped = huespedes
            .get_huesped(DocumentoDeIdentidad::new("88188188188"))
            .unwrap();

        let mut estancias_y_reservas = EstanciasYReservas::new(
            DatosDeEstanciasPruebas::new(),
            DatosDeReservasPruebas::new(),
        );
        assert_eq!(estancias_y_reservas.la_habitacion_esta_libre("101"), true);
        assert_eq!(estancias_y_reservas.la_habitacion_esta_libre("102"), true);

        let habitaciones_a_ocupar = vec![habitacion01, habitacion02];
        let huespedes_a_alojar = vec![un_huesped, una_huesped];
        let salida_prevista = Local::now() + Duration::days(2);
        estancias_y_reservas
            .crear_estancia(habitaciones_a_ocupar, huespedes_a_alojar, salida_prevista)
            .unwrap();

        assert_eq!(estancias_y_reservas.la_habitacion_esta_libre("101"), false);
        assert_eq!(estancias_y_reservas.la_habitacion_esta_libre("102"), false);
    }
}
