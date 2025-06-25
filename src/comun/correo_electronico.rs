
#[derive(Clone)]
pub struct CorreoElectronico {
    direccion_de_correo: String,
}
impl CorreoElectronico {
    pub fn new(correo: &str) -> Result<Self, String> {
        if correo.contains("@") {
            Ok(Self {
                direccion_de_correo: String::from(correo),
            })
        } else {
            Err(format!(
                "[{correo}] no tiene un formato valido para ser un correo electrónico."
            ))
        }
    }
}
impl std::fmt::Display for CorreoElectronico {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.direccion_de_correo)
    }
}

