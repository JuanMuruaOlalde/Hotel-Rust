use super::datos_de_habitaciones::DatosDeHabitaciones;

pub struct Habitaciones<T: DatosDeHabitaciones> {
    pub datos: T,
}
