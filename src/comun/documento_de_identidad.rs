#[derive(Clone, PartialEq)]
pub struct DocumentoDeIdentidad {
    numero: String,
}
impl DocumentoDeIdentidad {
    pub fn new(id: &str) -> Self {
        Self {
            numero: String::from(id),
        }
    }
}
impl std::fmt::Display for DocumentoDeIdentidad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.numero)
    }
}
