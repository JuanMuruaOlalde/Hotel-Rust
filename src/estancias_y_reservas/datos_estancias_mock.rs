use crate::estancias_y_reservas::modelo::HabitacionOcupada;

use super::datos_estancias::DatosDeEstancias;
use super::modelo::Estancia;

pub struct DatosDeEstanciasPruebas {
    lista_de_estancias: Vec<Estancia>,
}

impl DatosDeEstanciasPruebas {
    pub fn new() -> Self {
        Self {
            lista_de_estancias: Vec::new(),
        }
    }
}

impl DatosDeEstancias for DatosDeEstanciasPruebas {
    async fn guardar(&mut self, estancia: Estancia) -> Result<(), String> {
        self.lista_de_estancias.push(estancia);
        Ok(())
    }

    async fn get_habitaciones_ocupadas(&self) -> Result<Vec<HabitacionOcupada>, String> {
        let estancias_activas: Vec<Estancia> = self
            .lista_de_estancias
            .clone()
            .into_iter()
            .filter(|x| x.salida_real == None)
            .collect();
        let mut habitaciones_ocupadas: Vec<HabitacionOcupada> = Vec::new();
        for estancia in estancias_activas {
            for habitacion in estancia.get_habitaciones() {
                habitaciones_ocupadas.push(HabitacionOcupada {
                    id_interno_habitacion: habitacion.get_id_interno(),
                    nombre_habitacion: habitacion.id_publico_nombre,
                    id_interno_estancia: estancia.get_id_interno(),
                    salida_prevista: estancia.salida_prevista,
                })
            }
        }
        Ok(habitaciones_ocupadas)
    }
}
