use super::datos_de_huespedes::DatosDeHuespedes;

pub struct Huespedes<T: DatosDeHuespedes> {
    pub datos: T,
}
