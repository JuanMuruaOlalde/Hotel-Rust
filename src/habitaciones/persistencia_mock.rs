use super::habitaciones::Habitacion;
use super::persistencia::DatosDeHabitaciones;

pub struct DatosDeHabitacionesPruebas {
    lista_de_habitaciones: Box<Vec<Habitacion>>,
}

impl DatosDeHabitacionesPruebas {
    pub fn new() -> Self {
        Self {
            lista_de_habitaciones: Box::new(Vec::new()),
        }
    }
}

impl DatosDeHabitaciones for DatosDeHabitacionesPruebas {
    async fn get_habitacion(&self, nombre: &str) -> Result<Habitacion, String> {
        let habitacion = self
            .lista_de_habitaciones
            .iter()
            .find(|x| x.id_publico_nombre == nombre);
        match habitacion {
            Some(h) => Ok(h.clone()),
            None => Err(format!("No existe la habitación {nombre}")),
        }
    }

    async fn guardar(&mut self, habitacion: Habitacion) -> Result<(), String> {
        self.lista_de_habitaciones.push(habitacion.clone());
        Ok(())
    }
}
