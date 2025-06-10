use super::persistencia_reservas::DatosDeReservas;

pub struct DatosDeReservasPruebas {}

impl DatosDeReservas for DatosDeReservasPruebas {}

impl Default for DatosDeReservasPruebas {
    fn default() -> Self {
        Self {}
    }
}
