use chrono::{DateTime, Duration, Local};

use crate::habitaciones::habitaciones::Habitacion;
use crate::huespedes::huespedes::Huesped;
use crate::util::DocumentoDeIdentidad;

use super::datos_de_estancias::DatosDeEstancias;

pub struct Estancias {
    datos: Box<dyn DatosDeEstancias>,
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
    use crate::habitaciones::habitaciones::Habitaciones;
    use crate::habitaciones::habitaciones_para_pruebas::{
        HabitacionesParaPruebas, ID_DE_OTRA_HABITACION_DE_PRUEBAS, ID_DE_UNA_HABITACION_DE_PRUEBAS,
    };
    use crate::huespedes::huespedes::Huespedes;
    use crate::huespedes::huespedes_para_pruebas::{
        HuespedesParaPruebas, ID_DE_OTRO_HUESPED_DE_PRUEBAS, ID_DE_UN_HUESPED_DE_PRUEBAS,
    };

    use crate::estancias_y_reservas::estancias_para_pruebas::EstanciasParaPruebas;

    #[test]
    fn al_asignar_habitaciones_a_una_estancia_estas_quedan_ocupadas() {
        let mut estancias = Estancias {
            datos: Box::new(EstanciasParaPruebas::default()),
        };

        let habitaciones = Habitaciones {
            datos: &HabitacionesParaPruebas::new(),
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
            datos: &HuespedesParaPruebas::new(),
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
