use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

use hotel_rust::estancias_y_reservas::datos_de_estancias_mariadb::DatosDeEstanciasMariaDB;
use hotel_rust::estancias_y_reservas::datos_de_reservas_mariadb::DatosDeReservasMariaDB;
use hotel_rust::estancias_y_reservas::manejo_de_estancias_y_reservas::Estancias_y_Reservas;
use hotel_rust::habitaciones::Habitacion;
use hotel_rust::habitaciones::datos_de_habitaciones::DatosDeHabitaciones;
use hotel_rust::habitaciones::datos_de_habitaciones_mariadb::DatosDeHabitacionesMariaDB;
use hotel_rust::habitaciones::manejo_de_habitaciones::Habitaciones;
use hotel_rust::huespedes::Huesped;
use hotel_rust::huespedes::datos_de_huespedes::DatosDeHuespedes;
use hotel_rust::huespedes::datos_de_huespedes_mariadb::DatosDeHuespedesMariaDB;
use hotel_rust::huespedes::manejo_de_huespedes::Huespedes;
use hotel_rust::util::DocumentoDeIdentidad;

pub struct DatosParaLasPruebas<'a> {
    pub estancias_y_reservas:
        Estancias_y_Reservas<DatosDeEstanciasMariaDB<'a>, DatosDeReservasMariaDB<'a>>,
    pub habitacion01: Habitacion,
    pub habitacion02: Habitacion,
    pub un_huesped: Huesped,
    pub otro_huesped: Huesped,
}

impl<'a> DatosParaLasPruebas<'a> {
    pub async fn new(conexion: &'a Pool<MySql>) -> Self {
        let mut estancias_y_reservas = Estancias_y_Reservas {
            estancias: DatosDeEstanciasMariaDB::new(conexion),
            reservas: DatosDeReservasMariaDB::new(conexion),
        };

        let habitaciones = Habitaciones {
            datos: DatosDeHabitacionesMariaDB::new(conexion),
        };
        let habitacion01 = habitaciones
            .datos
            .get_habitacion("ID_DE_UNA_HABITACION_DE_PRUEBAS")
            .unwrap();
        let habitacion02 = habitaciones
            .datos
            .get_habitacion("ID_DE_OTRA_HABITACION_DE_PRUEBAS")
            .unwrap();

        let huespedes = Huespedes {
            datos: DatosDeHuespedesMariaDB::new(conexion),
        };
        let un_huesped = huespedes
            .datos
            .get_huesped(DocumentoDeIdentidad::new("ID_DE_UN_HUESPED_DE_PRUEBAS"))
            .unwrap();
        let otro_huesped = huespedes
            .datos
            .get_huesped(DocumentoDeIdentidad::new("ID_DE_OTRO_HUESPED_DE_PRUEBAS"))
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
