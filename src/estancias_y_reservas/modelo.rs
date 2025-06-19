use chrono::{DateTime, Local};
use uuid::Uuid;

use super::datos_estancias::DatosDeEstancias;
use super::datos_reservas::DatosDeReservas;
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
    pub async fn crear_estancia(
        &mut self,
        habitaciones: Vec<Habitacion>,
        huespedes: Vec<Huesped>,
        salida_prevista: DateTime<Local>,
    ) -> Result<(), String> {
        let estancia = Estancia::new(habitaciones, huespedes, salida_prevista);
        match self.estancias.guardar(estancia).await {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn la_habitacion_esta_libre(&self, nombre: &str) -> bool {
        let habitaciones_ocupadas = match self.estancias.get_habitaciones_ocupadas().await {
            Ok(h) => h,
            Err(e) => {
                println!("Error {e}");
                Vec::new() //TODO, esto no está bien, en caso de error devolvera que la habitacion esta libre; habria que hacer alguna otra cosa, pero no se me ocurre qué...
            }
        };
        if habitaciones_ocupadas
            .iter()
            .any(|habitacion| habitacion.nombre_habitacion == nombre)
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
    id_interno: uuid::Uuid,
    pub habitaciones: Vec<Habitacion>,
    pub huespedes: Vec<Huesped>,
    pub entrada_real: DateTime<Local>,
    pub salida_prevista: DateTime<Local>,
    pub salida_real: Option<DateTime<Local>>,
}

pub struct HabitacionOcupada {
    pub id_interno_habitacion: uuid::Uuid,
    pub nombre_habitacion: String,
    pub id_interno_estancia: uuid::Uuid,
    pub salida_prevista: DateTime<Local>,
}

impl Estancia {
    pub fn get_habitaciones(&self) -> Vec<Habitacion> {
        self.habitaciones.clone()
    }

    pub fn get_huespedes(&self) -> Vec<Huesped> {
        self.huespedes.clone()
    }

    pub fn new(
        habitaciones: Vec<Habitacion>,
        huespedes: Vec<Huesped>,
        salida_prevista: DateTime<Local>,
    ) -> Self {
        Self {
            id_interno: Uuid::now_v7(),
            habitaciones: habitaciones,
            huespedes: huespedes,
            entrada_real: Local::now(),
            salida_prevista: salida_prevista,
            salida_real: None,
        }
    }

    pub fn get_id_interno(&self) -> Uuid {
        self.id_interno
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
    use crate::habitaciones::datos_mock::DatosDeHabitacionesPruebas;
    use crate::habitaciones::modelo::{Habitaciones, TipoDeBaño, TipoDeHabitacion};
    use crate::huespedes::datos_mock::DatosDeHuespedesPruebas;
    use crate::huespedes::modelo::Huespedes;
    use crate::util::{CorreoElectronico, DocumentoDeIdentidad, Nacionalidad, Telefono};

    use crate::estancias_y_reservas::datos_estancias_mock::DatosDeEstanciasPruebas;
    use crate::estancias_y_reservas::datos_reservas_mock::DatosDeReservasPruebas;

    #[tokio::test]
    async fn al_asignar_habitaciones_a_una_estancia_estas_quedan_ocupadas() {
        let mut habitaciones = Habitaciones::new(DatosDeHabitacionesPruebas::new());
        habitaciones
            .añadir_una_nueva("101", TipoDeHabitacion::SENCILLA, TipoDeBaño::ConDUCHA)
            .await
            .unwrap();
        habitaciones
            .añadir_una_nueva("102", TipoDeHabitacion::DOBLE, TipoDeBaño::ConBAÑERA)
            .await
            .unwrap();
        let habitacion01 = habitaciones.get_habitacion("101").await.unwrap();
        let habitacion02 = habitaciones.get_habitacion("102").await.unwrap();

        let mut huespedes = Huespedes::new(DatosDeHuespedesPruebas::new());
        huespedes
            .añadir_una_persona_nueva(
                "Benzirpi Mirvento",
                Nacionalidad::IT_Italy,
                DocumentoDeIdentidad::new("99199199199"),
                Telefono::new("666777999"),
                CorreoElectronico::new("benzirpi@example.com").unwrap(),
            )
            .await
            .unwrap();
        huespedes
            .añadir_una_persona_nueva(
                "Julliane Zirteni",
                Nacionalidad::IT_Italy,
                DocumentoDeIdentidad::new("88188188188"),
                Telefono::new("666777888"),
                CorreoElectronico::new("julliane@example.com").unwrap(),
            )
            .await
            .unwrap();
        let un_huesped = huespedes
            .get_huesped(DocumentoDeIdentidad::new("99199199199"))
            .await
            .unwrap();
        let una_huesped = huespedes
            .get_huesped(DocumentoDeIdentidad::new("88188188188"))
            .await
            .unwrap();

        let mut estancias_y_reservas = EstanciasYReservas::new(
            DatosDeEstanciasPruebas::new(),
            DatosDeReservasPruebas::new(),
        );
        assert_eq!(
            estancias_y_reservas.la_habitacion_esta_libre("101").await,
            true
        );
        assert_eq!(
            estancias_y_reservas.la_habitacion_esta_libre("102").await,
            true
        );

        let habitaciones_a_ocupar = vec![habitacion01, habitacion02];
        let huespedes_a_alojar = vec![un_huesped, una_huesped];
        let salida_prevista = Local::now() + Duration::days(2);
        estancias_y_reservas
            .crear_estancia(habitaciones_a_ocupar, huespedes_a_alojar, salida_prevista)
            .await
            .unwrap();

        assert_eq!(
            estancias_y_reservas.la_habitacion_esta_libre("101").await,
            false
        );
        assert_eq!(
            estancias_y_reservas.la_habitacion_esta_libre("102").await,
            false
        );
    }
}
