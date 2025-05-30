use dotenv::dotenv;
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

    println!("Se va a realizar una prueba...");
    let algo = sqlx::query("SELECT ?")
        .bind(150_i64)
        .fetch_all(&conexion_con_la_bd)
        .await;
    match algo {
        Ok(resultado) => println!("Resultado: {:?}", resultado),
        Err(e) => println!("Error: {:?}", e),
    }

    println!();
    println!("Hello, hotel!");
}
