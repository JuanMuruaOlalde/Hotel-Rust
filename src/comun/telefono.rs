#[derive(Clone)]
pub struct Telefono {
    numero: String,
}
impl Telefono {
    pub fn new(telefono: &str) -> Self {
        Self {
            numero: String::from(telefono),
        }
    }
}
impl std::fmt::Display for Telefono {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.numero)
    }
}
