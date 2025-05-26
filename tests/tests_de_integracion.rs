use chrono::{Duration, Local};

use hotel_rust::estancias_y_reservas::Estancias;
use hotel_rust::estancias_y_reservas::datos_de_estancias_MariaDB::DatosDeEstancias_MariaDB;
use hotel_rust::habitaciones::Habitaciones;
use hotel_rust::habitaciones::datos_de_habitaciones_MariaDB::DatosDeHabitaciones_MariaDB;
use hotel_rust::huespedes::Huespedes;
use hotel_rust::huespedes::datos_de_huespedes_MariaDB::DatosDeHuespedes_MariaDB;
use hotel_rust::util::DocumentoDeIdentidad;

#[test]
fn al_asignar_habitaciones_a_una_estancia_estas_quedan_ocupadas_prueba_con_maria_db_() {
    todo!()
    // let mut estancias = Estancias {
    //     datos: Box::new(DatosDeEstancias_MariaDB::default()),
    // };

    // let habitaciones = Habitaciones {
    //     datos: &DatosDeHabitaciones_MariaDB::new(),
    // };
    // let habitacion01 = habitaciones
    //     .datos
    //     .get_habitacion(ID_DE_UNA_HABITACION_DE_PRUEBAS)
    //     .unwrap();
    // let habitacion02 = habitaciones
    //     .datos
    //     .get_habitacion(ID_DE_OTRA_HABITACION_DE_PRUEBAS)
    //     .unwrap();
    // let habitaciones_a_ocupar = vec![habitacion01, habitacion02];

    // let huespedes = Huespedes {
    //     datos: &DatosDeHuespedes_MariaDB::new(),
    // };
    // let un_huesped = huespedes
    //     .datos
    //     .get_huesped(DocumentoDeIdentidad::new(ID_DE_UN_HUESPED_DE_PRUEBAS))
    //     .unwrap();
    // let otro_huesped = huespedes
    //     .datos
    //     .get_huesped(DocumentoDeIdentidad::new(ID_DE_OTRO_HUESPED_DE_PRUEBAS))
    //     .unwrap();
    // let huespedes_a_alojar = vec![un_huesped, otro_huesped];

    // let salida_prevista = Local::now() + Duration::days(2);

    // assert_eq!(
    //     estancias
    //         .datos
    //         .la_habitacion_esta_libre(ID_DE_UNA_HABITACION_DE_PRUEBAS),
    //     true
    // );
    // assert_eq!(
    //     estancias
    //         .datos
    //         .la_habitacion_esta_libre(ID_DE_OTRA_HABITACION_DE_PRUEBAS),
    //     true
    // );

    // let _ =
    //     estancias
    //         .datos
    //         .crear_estancia(habitaciones_a_ocupar, huespedes_a_alojar, salida_prevista);

    // assert_eq!(
    //     estancias
    //         .datos
    //         .la_habitacion_esta_libre(ID_DE_UNA_HABITACION_DE_PRUEBAS),
    //     false
    // );
    // assert_eq!(
    //     estancias
    //         .datos
    //         .la_habitacion_esta_libre(ID_DE_OTRA_HABITACION_DE_PRUEBAS),
    //     false
    // );
}
