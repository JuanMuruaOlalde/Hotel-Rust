use super::modelo::Estancia;

pub trait DatosDeEstancias {
    fn guardar(&mut self, estancia: Estancia) -> Result<(), String>;

    fn get_estancias_activas(&self) -> Vec<Estancia>;
}
