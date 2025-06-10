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
    pub estancias: E,
    pub reservas: R,
}

impl<E, R> EstanciasYReservas<E, R>
where
    E: DatosDeEstancias,
    R: DatosDeReservas,
{
    pub fn la_habitacion_esta_libre(&self, nombre: &str) -> bool {
        if self.estancias.get_estancias().iter().any(|estancia| {
            estancia
                .get_habitaciones()
                .iter()
                .any(|habitacion| habitacion.nombre == nombre)
        }) {
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

pub struct Reserva {
    //TODO, pendiente de implementar
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Local};

    use super::*;
    use crate::habitaciones::modelo::Habitaciones;
    use crate::habitaciones::persistencia::DatosDeHabitaciones;
    use crate::habitaciones::persistencia_mock::{
        DatosDeHabitacionesPruebas, ID_DE_OTRA_HABITACION_DE_PRUEBAS,
        ID_DE_UNA_HABITACION_DE_PRUEBAS,
    };
    use crate::huespedes::modelo::Huespedes;
    use crate::huespedes::persistencia::DatosDeHuespedes;
    use crate::huespedes::persistencia_mock::{
        DatosDeHuespedesPruebas, ID_DE_OTRO_HUESPED_DE_PRUEBAS, ID_DE_UN_HUESPED_DE_PRUEBAS,
    };
    use crate::util::DocumentoDeIdentidad;

    use crate::estancias_y_reservas::persistencia_estancias_mock::DatosDeEstanciasPruebas;
    use crate::estancias_y_reservas::persistencia_reservas_mock::DatosDeReservasPruebas;

    #[test]
    fn al_asignar_habitaciones_a_una_estancia_estas_quedan_ocupadas() {
        let mut datos = EstanciasYReservas {
            estancias: DatosDeEstanciasPruebas::default(),
            reservas: DatosDeReservasPruebas::default(),
        };

        let habitaciones = Habitaciones {
            datos: DatosDeHabitacionesPruebas::new(),
        };
        let habitacion01 = habitaciones
            .datos
            .get_habitacion(ID_DE_UNA_HABITACION_DE_PRUEBAS)
            .unwrap();
        let habitacion02 = habitaciones
            .datos
            .get_habitacion(ID_DE_OTRA_HABITACION_DE_PRUEBAS)
            .unwrap();

        let huespedes = Huespedes {
            datos: DatosDeHuespedesPruebas::new(),
        };
        let un_huesped = huespedes
            .datos
            .get_huesped(DocumentoDeIdentidad::new(ID_DE_UN_HUESPED_DE_PRUEBAS))
            .unwrap();
        let otro_huesped = huespedes
            .datos
            .get_huesped(DocumentoDeIdentidad::new(ID_DE_OTRO_HUESPED_DE_PRUEBAS))
            .unwrap();

        assert_eq!(
            datos.la_habitacion_esta_libre(ID_DE_UNA_HABITACION_DE_PRUEBAS),
            true
        );
        assert_eq!(
            datos.la_habitacion_esta_libre(ID_DE_OTRA_HABITACION_DE_PRUEBAS),
            true
        );

        let habitaciones_a_ocupar = vec![habitacion01, habitacion02];
        let huespedes_a_alojar = vec![un_huesped, otro_huesped];
        let salida_prevista = Local::now() + Duration::days(2);
        let _ = datos.estancias.crear_estancia(
            habitaciones_a_ocupar,
            huespedes_a_alojar,
            salida_prevista,
        );

        assert_eq!(
            datos.la_habitacion_esta_libre(ID_DE_UNA_HABITACION_DE_PRUEBAS),
            false
        );
        assert_eq!(
            datos.la_habitacion_esta_libre(ID_DE_OTRA_HABITACION_DE_PRUEBAS),
            false
        );
    }
}
