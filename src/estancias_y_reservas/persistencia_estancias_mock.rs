use super::modelo::Estancia;
use super::persistencia_estancias::DatosDeEstancias;

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
    fn guardar(&mut self, estancia: Estancia) -> Result<(), String> {
        self.lista_de_estancias.push(estancia);
        Ok(())
    }

    fn get_estancias_activas(&self) -> Vec<Estancia> {
        self.lista_de_estancias
            .clone()
            .into_iter()
            .filter(|x| x.salida_real == None)
            .collect()
    }
}
