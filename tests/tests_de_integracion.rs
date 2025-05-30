use chrono::{Duration, Local};
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;

use hotel_rust::estancias_y_reservas::datos_de_estancias::DatosDeEstancias;
use hotel_rust::estancias_y_reservas::datos_de_estancias_mariadb::DatosDeEstanciasMariaDB;
use hotel_rust::estancias_y_reservas::datos_de_reservas_mariadb::DatosDeReservasMariaDB;
use hotel_rust::estancias_y_reservas::manejo_de_estancias_y_reservas::Estancias_y_Reservas;
use hotel_rust::habitaciones::Habitaciones;
use hotel_rust::habitaciones::datos_de_habitaciones::DatosDeHabitaciones;
use hotel_rust::habitaciones::datos_de_habitaciones_mariadb::DatosDeHabitacionesMariaDB;
use hotel_rust::huespedes::Huespedes;
use hotel_rust::huespedes::datos_de_huespedes::DatosDeHuespedes;
use hotel_rust::huespedes::datos_de_huespedes_mariadb::DatosDeHuespedesMariaDB;
use hotel_rust::util::DocumentoDeIdentidad;

#[test]
fn al_asignar_habitaciones_a_una_estancia_estas_quedan_ocupadas_prueba_con_maria_db_() {
    todo!();
}
async fn esto_esta_sin_terminar() {
    dotenv().ok();
    let conexion_con_la_bd =
        MySqlPoolOptions::new()
            .max_connections(1)
            .connect(&std::env::var("DATABASE_DE_PRUEBAS_URL").expect(
                "No se han encontrado las llaves de entrada a la base de datos para pruebas.",
            ))
            .await
            .expect("No se ha podido establecer conexión con la base de datos de pruebas.");

    let mut datos = Estancias_y_Reservas {
        estancias: DatosDeEstanciasMariaDB::new(&conexion_con_la_bd),
        reservas: DatosDeReservasMariaDB::new(&conexion_con_la_bd),
    };

    let habitaciones = Habitaciones {
        datos: DatosDeHabitacionesMariaDB::new(&conexion_con_la_bd),
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
        datos: DatosDeHuespedesMariaDB::new(&conexion_con_la_bd),
    };
    let un_huesped = huespedes
        .datos
        .get_huesped(DocumentoDeIdentidad::new("ID_DE_UN_HUESPED_DE_PRUEBAS"))
        .unwrap();
    let otro_huesped = huespedes
        .datos
        .get_huesped(DocumentoDeIdentidad::new("ID_DE_OTRO_HUESPED_DE_PRUEBAS"))
        .unwrap();

    assert_eq!(
        datos.la_habitacion_esta_libre("ID_DE_UNA_HABITACION_DE_PRUEBAS"),
        true
    );
    assert_eq!(
        datos.la_habitacion_esta_libre("ID_DE_OTRA_HABITACION_DE_PRUEBAS"),
        true
    );

    let habitaciones_a_ocupar = vec![habitacion01, habitacion02];
    let huespedes_a_alojar = vec![un_huesped, otro_huesped];
    let salida_prevista = Local::now() + Duration::days(2);
    let _ =
        datos
            .estancias
            .crear_estancia(habitaciones_a_ocupar, huespedes_a_alojar, salida_prevista);

    assert_eq!(
        datos.la_habitacion_esta_libre("ID_DE_UNA_HABITACION_DE_PRUEBAS"),
        false
    );
    assert_eq!(
        datos.la_habitacion_esta_libre("ID_DE_OTRA_HABITACION_DE_PRUEBAS"),
        false
    );
}
