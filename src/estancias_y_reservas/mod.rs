pub mod datos_de_estancias;
pub mod datos_de_estancias_mariadb;
mod datos_de_estancias_pruebas;

use chrono::{DateTime, Duration, Local};

use crate::estancias_y_reservas::datos_de_estancias::DatosDeEstancias;
use crate::habitaciones::Habitacion;
use crate::huespedes::Huesped;
use crate::util::DocumentoDeIdentidad;

pub struct Estancias<T: DatosDeEstancias> {
    pub datos: T,
}

pub struct Estancia {
    pub habitaciones: Vec<Habitacion>,
    pub huespedes: Vec<Huesped>,
    pub entrada_real: DateTime<Local>,
    pub salida_prevista: DateTime<Local>,
    pub salida_real: Option<DateTime<Local>>,
}

impl Estancia {
    pub fn get_habitaciones_ocupadas(&self) -> Vec<Habitacion> {
        self.habitaciones.clone()
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;
    use crate::habitaciones::Habitaciones;
    use crate::habitaciones::datos_de_habitaciones::DatosDeHabitaciones;
    use crate::habitaciones::datos_de_habitaciones_pruebas::{
        DatosDeHabitaciones_Pruebas, ID_DE_OTRA_HABITACION_DE_PRUEBAS,
        ID_DE_UNA_HABITACION_DE_PRUEBAS,
    };
    use crate::huespedes::Huespedes;
    use crate::huespedes::datos_de_huespedes::DatosDeHuespedes;
    use crate::huespedes::datos_de_huespedes_pruebas::{
        DatosDeHuespedes_Pruebas, ID_DE_OTRO_HUESPED_DE_PRUEBAS, ID_DE_UN_HUESPED_DE_PRUEBAS,
    };

    use crate::estancias_y_reservas::datos_de_estancias_pruebas::DatosDeEstancias_Pruebas;

    #[test]
    fn al_asignar_habitaciones_a_una_estancia_estas_quedan_ocupadas() {
        let mut estancias = Estancias {
            datos: DatosDeEstancias_Pruebas::default(),
        };

        let habitaciones = Habitaciones {
            datos: DatosDeHabitaciones_Pruebas::new(),
        };
        let habitacion01 = habitaciones
            .datos
            .get_habitacion(ID_DE_UNA_HABITACION_DE_PRUEBAS)
            .unwrap();
        let habitacion02 = habitaciones
            .datos
            .get_habitacion(ID_DE_OTRA_HABITACION_DE_PRUEBAS)
            .unwrap();
        let habitaciones_a_ocupar = vec![habitacion01, habitacion02];

        let huespedes = Huespedes {
            datos: DatosDeHuespedes_Pruebas::new(),
        };
        let un_huesped = huespedes
            .datos
            .get_huesped(DocumentoDeIdentidad::new(ID_DE_UN_HUESPED_DE_PRUEBAS))
            .unwrap();
        let otro_huesped = huespedes
            .datos
            .get_huesped(DocumentoDeIdentidad::new(ID_DE_OTRO_HUESPED_DE_PRUEBAS))
            .unwrap();
        let huespedes_a_alojar = vec![un_huesped, otro_huesped];

        let salida_prevista = Local::now() + Duration::days(2);

        assert_eq!(
            estancias
                .datos
                .la_habitacion_esta_libre(ID_DE_UNA_HABITACION_DE_PRUEBAS),
            true
        );
        assert_eq!(
            estancias
                .datos
                .la_habitacion_esta_libre(ID_DE_OTRA_HABITACION_DE_PRUEBAS),
            true
        );

        let _ = estancias.datos.crear_estancia(
            habitaciones_a_ocupar,
            huespedes_a_alojar,
            salida_prevista,
        );

        assert_eq!(
            estancias
                .datos
                .la_habitacion_esta_libre(ID_DE_UNA_HABITACION_DE_PRUEBAS),
            false
        );
        assert_eq!(
            estancias
                .datos
                .la_habitacion_esta_libre(ID_DE_OTRA_HABITACION_DE_PRUEBAS),
            false
        );
    }
}
