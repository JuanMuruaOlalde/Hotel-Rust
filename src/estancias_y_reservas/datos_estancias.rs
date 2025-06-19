use crate::estancias_y_reservas::modelo::HabitacionOcupada;

use super::modelo::Estancia;

pub trait DatosDeEstancias {
    async fn guardar(&mut self, estancia: Estancia) -> Result<(), String>;

    async fn get_habitaciones_ocupadas(&self) -> Result<Vec<HabitacionOcupada>, String>;
}
