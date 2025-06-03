mod common;

use chrono::{Duration, Local};

use hotel_rust::estancias_y_reservas::datos_de_estancias::DatosDeEstancias;
use sqlx::{MySql, Pool};

#[sqlx::test]
async fn al_asignar_habitaciones_a_una_estancia_estas_quedan_ocupadas_prueba_con_maria_db_(
    conexion: Pool<MySql>,
) {
    let mut datos = common::DatosParaLasPruebas::new(&conexion).await;

    assert_eq!(
        datos
            .estancias_y_reservas
            .la_habitacion_esta_libre("ID_DE_UNA_HABITACION_DE_PRUEBAS"),
        true
    );
    assert_eq!(
        datos
            .estancias_y_reservas
            .la_habitacion_esta_libre("ID_DE_OTRA_HABITACION_DE_PRUEBAS"),
        true
    );

    let habitaciones_a_ocupar = vec![datos.habitacion01.clone(), datos.habitacion02.clone()];
    let huespedes_a_alojar = vec![datos.un_huesped.clone(), datos.otro_huesped.clone()];
    let salida_prevista = Local::now() + Duration::days(2);
    let _ = datos.estancias_y_reservas.estancias.crear_estancia(
        habitaciones_a_ocupar,
        huespedes_a_alojar,
        salida_prevista,
    );

    assert_eq!(
        datos
            .estancias_y_reservas
            .la_habitacion_esta_libre("ID_DE_UNA_HABITACION_DE_PRUEBAS"),
        false
    );
    assert_eq!(
        datos
            .estancias_y_reservas
            .la_habitacion_esta_libre("ID_DE_OTRA_HABITACION_DE_PRUEBAS"),
        false
    );
}
