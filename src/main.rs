use dotenv::dotenv;
use hotel_rust::{
    estancias_y_reservas::{
        modelo::EstanciasYReservas, persistencia_estancias_mariadb::DatosDeEstanciasMariaDB,
        persistencia_reservas_mariadb::DatosDeReservasMariaDB,
    },
    habitaciones::{modelo::Habitaciones, persistencia_mariadb::DatosDeHabitacionesMariaDB},
    huespedes::{modelo::Huespedes, persistencia_mariadb::DatosDeHuespedesMariaDB},
};
use sqlx::mysql::MySqlPoolOptions;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!();

    println!("Estableciendo conexión con la base de datos...");
    dotenv().ok();
    let conexion_con_la_bd = MySqlPoolOptions::new()
        .max_connections(1)
        .connect(
            &std::env::var("DATABASE_URL")
                .expect("No se han encontrado las llaves de entrada a la base de datos."),
        )
        .await
        .expect("No se ha podido establecer conexión con la base de datos.");
    println!("Conexión establecida.");

    // println!("Se va a realizar una prueba...");
    // let algo = sqlx::query("SELECT ?")
    //     .bind(150_i64)
    //     .fetch_all(&conexion_con_la_bd)
    //     .await;
    // match algo {
    //     Ok(resultado) => println!("Resultado: {:?}", resultado),
    //     Err(e) => println!("Error: {:?}", e),
    // }

    // aquí irá el código que inicializa y lanza la aplicación
    println!();
    println!("Hello, hotel!");
    println!();

    // let mut estancias_y_reservas = Estancias_y_Reservas {
    //     estancias: DatosDeEstanciasMariaDB::new(&conexion_con_la_bd),
    //     reservas: DatosDeReservasMariaDB::new(&conexion_con_la_bd),
    // };

    // let habitaciones = Habitaciones {
    //     datos: DatosDeHabitacionesMariaDB::new(&conexion_con_la_bd),
    // };
    // let huespedes = Huespedes {
    //     datos: DatosDeHuespedesMariaDB::new(&conexion_con_la_bd),
    // };

    //etc, etc

    conexion_con_la_bd.close().await;
}
