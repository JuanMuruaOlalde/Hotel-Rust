use super::datos_de_estancias::DatosDeEstancias;
use super::datos_de_reservas::DatosDeReservas;

pub struct Estancias_y_Reservas<E, R>
where
    E: DatosDeEstancias,
    R: DatosDeReservas,
{
    pub estancias: E,
    pub reservas: R,
}

impl<E, R> Estancias_y_Reservas<E, R>
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
            false
        } else {
            //TODO, pendiente comprobar las reservas antes de dar la habitacion por libre.
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Local};

    use super::*;
    use crate::habitaciones::Habitaciones;
    use crate::habitaciones::datos_de_habitaciones::DatosDeHabitaciones;
    use crate::habitaciones::datos_de_habitaciones_pruebas::{
        DatosDeHabitacionesPruebas, ID_DE_OTRA_HABITACION_DE_PRUEBAS,
        ID_DE_UNA_HABITACION_DE_PRUEBAS,
    };
    use crate::huespedes::Huespedes;
    use crate::huespedes::datos_de_huespedes::DatosDeHuespedes;
    use crate::huespedes::datos_de_huespedes_pruebas::{
        DatosDeHuespedesPruebas, ID_DE_OTRO_HUESPED_DE_PRUEBAS, ID_DE_UN_HUESPED_DE_PRUEBAS,
    };
    use crate::util::DocumentoDeIdentidad;

    use crate::estancias_y_reservas::datos_de_estancias_pruebas::DatosDeEstanciasPruebas;
    use crate::estancias_y_reservas::datos_de_reservas_pruebas::DatosDeReservasPruebas;

    #[test]
    fn al_asignar_habitaciones_a_una_estancia_estas_quedan_ocupadas() {
        let mut datos = Estancias_y_Reservas {
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
