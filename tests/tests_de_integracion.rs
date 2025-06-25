mod common;

use chrono::{Duration, Local};

use hotel_rust::{
    comun::documento_de_identidad::DocumentoDeIdentidad,
    estancias_y_reservas::{
        datos_estancias_mariadb::DatosDeEstanciasMariaDB,
        datos_reservas_mariadb::DatosDeReservasMariaDB, modelo::EstanciasYReservas,
    },
};
use sqlx::{MySql, Pool};

#[sqlx::test]
async fn al_asignar_habitaciones_a_una_estancia_estas_quedan_ocupadas_prueba_con_maria_db_(
    conexion: Pool<MySql>,
) {
    let habitaciones = common::preparar_habitaciones_para_pruebas(&conexion).await;
    let habitacion01 = habitaciones
        .get_habitacion(common::ID_DE_UNA_HABITACION_DE_PRUEBAS)
        .await
        .unwrap();
    let habitacion02 = habitaciones
        .get_habitacion(common::ID_DE_OTRA_HABITACION_DE_PRUEBAS)
        .await
        .unwrap();

    let huespedes = common::preparar_huespedes_para_pruebas(&conexion).await;
    let un_huesped = huespedes
        .get_huesped(DocumentoDeIdentidad::new(
            common::ID_DE_UN_HUESPED_DE_PRUEBAS,
        ))
        .await
        .unwrap();
    let otro_huesped = huespedes
        .get_huesped(DocumentoDeIdentidad::new(
            common::ID_DE_OTRO_HUESPED_DE_PRUEBAS,
        ))
        .await
        .unwrap();

    let mut estancias_y_reservas = EstanciasYReservas::new(
        DatosDeEstanciasMariaDB::new(&conexion),
        DatosDeReservasMariaDB::new(&conexion),
    );
    assert_eq!(
        estancias_y_reservas
            .la_habitacion_esta_libre(common::ID_DE_UNA_HABITACION_DE_PRUEBAS)
            .await,
        true
    );
    assert_eq!(
        estancias_y_reservas
            .la_habitacion_esta_libre(common::ID_DE_OTRA_HABITACION_DE_PRUEBAS)
            .await,
        true
    );

    let habitaciones_a_ocupar = vec![habitacion01, habitacion02];
    let huespedes_a_alojar = vec![un_huesped, otro_huesped];
    let salida_prevista = Local::now() + Duration::days(2);
    estancias_y_reservas
        .crear_estancia(habitaciones_a_ocupar, huespedes_a_alojar, salida_prevista)
        .await
        .unwrap();

    assert_eq!(
        estancias_y_reservas
            .la_habitacion_esta_libre(common::ID_DE_UNA_HABITACION_DE_PRUEBAS)
            .await,
        false
    );
    assert_eq!(
        estancias_y_reservas
            .la_habitacion_esta_libre(common::ID_DE_OTRA_HABITACION_DE_PRUEBAS)
            .await,
        false
    );
}
