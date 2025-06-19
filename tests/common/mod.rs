use hotel_rust::util::DocumentoDeIdentidad;
use sqlx::{MySql, Pool};

use hotel_rust::habitaciones::datos_mariadb::DatosDeHabitacionesMariaDB;
use hotel_rust::habitaciones::modelo::Habitaciones;
use hotel_rust::habitaciones::modelo::TipoDeBaño;
use hotel_rust::habitaciones::modelo::TipoDeHabitacion;
use hotel_rust::huespedes::datos_mariadb::DatosDeHuespedesMariaDB;
use hotel_rust::huespedes::modelo::Huespedes;
use hotel_rust::util::CorreoElectronico;
use hotel_rust::util::Nacionalidad;
use hotel_rust::util::Telefono;

pub const ID_DE_UNA_HABITACION_DE_PRUEBAS: &str = "PRB101";
pub const ID_DE_OTRA_HABITACION_DE_PRUEBAS: &str = "PRB102";
pub const ID_DE_UN_HUESPED_DE_PRUEBAS: &str = "99199199199";
pub const ID_DE_OTRO_HUESPED_DE_PRUEBAS: &str = "88188188188";

pub async fn preparar_habitaciones_para_pruebas(
    conexion: &Pool<MySql>,
) -> Habitaciones<DatosDeHabitacionesMariaDB> {
    let mut habitaciones = Habitaciones::new(DatosDeHabitacionesMariaDB::new(conexion));
    habitaciones
        .añadir_una_nueva(
            ID_DE_UNA_HABITACION_DE_PRUEBAS,
            TipoDeHabitacion::SENCILLA,
            TipoDeBaño::ConDUCHA,
        )
        .await
        .unwrap();
    habitaciones
        .añadir_una_nueva(
            ID_DE_OTRA_HABITACION_DE_PRUEBAS,
            TipoDeHabitacion::DOBLE,
            TipoDeBaño::ConBAÑERA,
        )
        .await
        .unwrap();
    habitaciones
}

pub async fn preparar_huespedes_para_pruebas(
    conexion: &Pool<MySql>,
) -> Huespedes<DatosDeHuespedesMariaDB> {
    let mut huespedes = Huespedes::new(DatosDeHuespedesMariaDB::new(conexion));
    huespedes
        .añadir_una_persona_nueva(
            "Benzirpi Mirvento",
            Nacionalidad::IT_Italy,
            DocumentoDeIdentidad::new(ID_DE_UN_HUESPED_DE_PRUEBAS),
            Telefono::new("666777999"),
            CorreoElectronico::new("benzirpi@example.com").unwrap(),
        )
        .await
        .unwrap();
    huespedes
        .añadir_una_persona_nueva(
            "Julliane Zirteni",
            Nacionalidad::IT_Italy,
            DocumentoDeIdentidad::new(ID_DE_OTRO_HUESPED_DE_PRUEBAS),
            Telefono::new("666777888"),
            CorreoElectronico::new("julliane@example.com").unwrap(),
        )
        .await
        .unwrap();
    huespedes
}
