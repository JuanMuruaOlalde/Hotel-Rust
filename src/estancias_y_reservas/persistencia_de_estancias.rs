use crate::estancias_y_reservas::estancias_y_reservas::HabitacionOcupada;

use super::estancias_y_reservas::Estancia;

pub trait DatosDeEstancias {
    async fn guardar(&mut self, estancia: Estancia) -> Result<(), String>;

    async fn get_habitaciones_ocupadas(&self) -> Result<Vec<HabitacionOcupada>, String>;
}
