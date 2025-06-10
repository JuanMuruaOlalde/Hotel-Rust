use sqlx::{MySql, Pool};

use hotel_rust::estancias_y_reservas::modelo::EstanciasYReservas;
use hotel_rust::estancias_y_reservas::persistencia_estancias_mariadb::DatosDeEstanciasMariaDB;
use hotel_rust::estancias_y_reservas::persistencia_reservas_mariadb::DatosDeReservasMariaDB;
use hotel_rust::habitaciones::modelo::Habitacion;
use hotel_rust::habitaciones::modelo::Habitaciones;
use hotel_rust::habitaciones::persistencia::DatosDeHabitaciones;
use hotel_rust::habitaciones::persistencia_mariadb::DatosDeHabitacionesMariaDB;
use hotel_rust::huespedes::modelo::Huesped;
use hotel_rust::huespedes::modelo::Huespedes;
use hotel_rust::huespedes::persistencia::DatosDeHuespedes;
use hotel_rust::huespedes::persistencia_mariadb::DatosDeHuespedesMariaDB;
use hotel_rust::util::DocumentoDeIdentidad;

pub const ID_DE_UNA_HABITACION_DE_PRUEBAS: &str = "PRB101";
pub const ID_DE_OTRA_HABITACION_DE_PRUEBAS: &str = "PRB102";
pub const ID_DE_UN_HUESPED_DE_PRUEBAS: &str = "99199199199";
pub const ID_DE_OTRO_HUESPED_DE_PRUEBAS: &str = "88188188188";

pub struct DatosParaPruebasDeEstanciasYReservas<'a> {
    pub estancias_y_reservas:
        EstanciasYReservas<DatosDeEstanciasMariaDB<'a>, DatosDeReservasMariaDB<'a>>,
    pub habitacion01: Habitacion,
    pub habitacion02: Habitacion,
    pub un_huesped: Huesped,
    pub otro_huesped: Huesped,
}

impl<'a> DatosParaPruebasDeEstanciasYReservas<'a> {
    pub async fn new(conexion: &'a Pool<MySql>) -> Self {
        let estancias_y_reservas = EstanciasYReservas {
            estancias: DatosDeEstanciasMariaDB::new(conexion),
            reservas: DatosDeReservasMariaDB::new(conexion),
        };

        let habitaciones = Habitaciones {
            datos: DatosDeHabitacionesMariaDB::new(conexion),
        };
        let habitacion01 = habitaciones
            .datos
            .get_habitacion(ID_DE_UNA_HABITACION_DE_PRUEBAS)
            .unwrap();
        let habitacion02 = habitaciones
            .datos
            .get_habitacion(ID_DE_OTRA_HABITACION_DE_PRUEBAS)
            .unwrap();

        let huespedes = Huespedes {
            datos: DatosDeHuespedesMariaDB::new(conexion),
        };
        let un_huesped = huespedes
            .datos
            .get_huesped(DocumentoDeIdentidad::new(ID_DE_UN_HUESPED_DE_PRUEBAS))
            .unwrap();
        let otro_huesped = huespedes
            .datos
            .get_huesped(DocumentoDeIdentidad::new(ID_DE_OTRO_HUESPED_DE_PRUEBAS))
            .unwrap();
        Self {
            estancias_y_reservas,
            habitacion01,
            habitacion02,
            un_huesped,
            otro_huesped,
        }
    }
}
