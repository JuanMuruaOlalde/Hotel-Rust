use hotel_rust::{
    estancias_y_reservas::{
        modelo::EstanciasYReservas, datos_estancias_mariadb::DatosDeEstanciasMariaDB,
        datos_reservas_mariadb::DatosDeReservasMariaDB,
    },
    habitaciones::{modelo::Habitaciones, datos_mariadb::DatosDeHabitacionesMariaDB},
    huespedes::{modelo::Huespedes, datos_mariadb::DatosDeHuespedesMariaDB},
};
use sqlx::mysql::MySqlPoolOptions;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!();

    println!("Estableciendo conexión con la base de datos...");
    dotenvy::dotenv().ok();
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

    let mut estancias_y_reservas = EstanciasYReservas::new(
        DatosDeEstanciasMariaDB::new(&conexion_con_la_bd),
        DatosDeReservasMariaDB::new(&conexion_con_la_bd),
    );
    let mut habitaciones = Habitaciones::new(DatosDeHabitacionesMariaDB::new(&conexion_con_la_bd));
    let mut huespedes = Huespedes::new(DatosDeHuespedesMariaDB::new(&conexion_con_la_bd));
    //etc, etc
    // aquí irá el resto del código que inicializa y lanza la aplicación

    //como todavia no se lanza la aplicación...
    //...esto es simplemente por ver algo si la ejecutamos.
    println!();
    println!("Hello, hotel!");
    println!();

    conexion_con_la_bd.close().await;
}
