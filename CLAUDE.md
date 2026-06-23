# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Propósito del proyecto

Ejercicio de gestión hotelera en Rust, concebido para ilustrar la sección 9.3 "Arquitecturas multi-capa" del libro [Más allá del IF y del WHILE](https://www.susosise.es/documentos/Mas_alla_del_IF_y_del_WHILE.pdf). El objetivo principal es mostrar una organización de código limpia y bien separada, no la funcionalidad completa de un hotel real.

Se sigue la metodología DDD (Domain Driven Design) y la arquitectura Hexagonal
(Ports & Adapters). Por ahora solo están implementados los puertos secundarios
(persistencia). Los puertos primarios (UI/API) están pendientes.

## Reglas

### Instrucciones importantes sobre la persistencia

- No toques estos archivos (están congelados):
  `migrations_mariadbmigrations_mariadb/20250605134028_habitaciones_huespedes_y_estancias.down.sql`
  `migrations_mariadbmigrations_mariadb/20250605134028_habitaciones_huespedes_y_estancias.up.sql`
  `src/habitaciones/persistencia_en_mariadb.rs`,
  `src/huespedes/persistencia_en_mariadb.rs`,
  `src/estancias_y_reservas/persistencia_de_estancias_en_mariadb.rs`,
  `src/estancias_y_reservas/persistencia_de_reservas_en_mariadb.rs`,
  `tests/tests_de_integracion_con_mariadb.rs`

Al comienzo del proyecto, cuando se empezó a trabajar los módulos `habitaciones`, `huespedes`, `estancias_y_reservas`. Se queria ilustrar la separación entre aspectos de dominio y aspectos tecnologicos. Y por eso se implementó la persistencia de tres formas distintas: mock, en MariaDB y en SQLite. Para ver que, con la adecuada separación, eran intercambiables con facilidad. Pero a partir de ahora vamos a seguir trabajando solo con mock y SQLite. 

## Comandos habituales

```bash

cargo build
cargo run                          # requiere .env con DATABASE_URL apuntando a SQLite
cargo test                         # tests unitarios (usan persistencia mock, sin BD)

# Migraciones SQLite
sqlx migrate run --source migrations_sqlite

```

La variable de entorno `DATABASE_URL` se carga desde `.env` con `dotenvy`. Para SQLite, un valor válido es `sqlite://hotel.db`.

## Arquitectura

El proyecto sigue una arquitectura multi-capa del tipo hexagonal. Separando con claridad los aspectos relativos al dominio de los aspectos relativos a las tecnologias de implementación.

Cada módulo de dominio tiene exactamente esta estructura de archivos:

```
<dominio>/
  <dominio>.rs              ← structs del modelo y lógica de negocio
  persistencia.rs           ← trait que define el contrato de persistencia
  persistencia_en_sqlite.rs ← implementación SQLite (sqlx)
  persistencia_mock.rs      ← implementación en memoria para tests
  mod.rs
```

Los módulos de dominio actuales son: `habitaciones`, `huespedes`, `estancias_y_reservas`.

**Regla de dependencias**: el modelo (`<dominio>.rs`) no conoce nada de la persistencia. La capa de persistencia depende del modelo, nunca al revés. El `main.rs` es el único lugar donde se elige qué implementación de persistencia se inyecta.

### Inyección de la persistencia

Cada struct de dominio de alto nivel (p.ej. `Habitaciones<T>`) es genérico sobre `T: DatosDeHabitaciones`. El trait `DatosDeHabitaciones` (en `persistencia.rs`) define los métodos async requeridos. En `main.rs` se instancia con la implementación concreta:

```rust
Habitaciones::new(DatosDeHabitacionesSQLite::new(&conexion))
```

En los tests unitarios se usa `DatosDeHabitacionesPruebas::new()` (mock en memoria).

### Módulo `comun`

Contiene tipos-valor compartidos entre dominios: `CorreoElectronico`, `DocumentoDeIdentidad`, `Nacionalidad`, `Telefono`. Estos tipos encapsulan validación básica y se pasan como argumentos tipados en lugar de strings crudos.

### Identificadores

Cada entidad tiene dos IDs:
- `id_interno`: UUID v7, usado internamente y en la BD como clave primaria (guardado como TEXT en SQLite).
- `id_publico`: string legible por humanos (p.ej. nombre de habitación "101", número de documento del huésped).

### Tests

Los tests unitarios están en el propio archivo de cada modelo (`#[cfg(test)]` al final de `<dominio>.rs`) y usan las implementaciones mock.

Los test de integracion están en la carpeta `./tests` y usan las implementaciones con SQLite. 

