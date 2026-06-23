-- Add up migration script here
CREATE TABLE IF NOT EXISTS huespedes (
    id TEXT NOT NULL PRIMARY KEY,
    nombre_y_apellidos TEXT NOT NULL,
    nacionalidad TEXT NULL,
    documento_de_id TEXT NOT NULL,
    telefono TEXT NOT NULL,
    correo_e TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS habitaciones (
    id TEXT NOT NULL PRIMARY KEY,
    nombre TEXT NOT NULL,
    tipo_de_habitacion TEXT NULL,
    "tipo_de_baño" TEXT NULL
);

CREATE TABLE IF NOT EXISTS estancias (
    id TEXT NOT NULL PRIMARY KEY,
    entrada TEXT NULL,
    salida_prevista TEXT NULL,
    salida_real TEXT NULL
);

CREATE TABLE IF NOT EXISTS estancias_habitaciones (
    id_estancia TEXT NULL,
    id_habitacion TEXT NULL
);

CREATE TABLE IF NOT EXISTS estancias_huespedes (
    id_estancia TEXT NULL,
    id_huesped TEXT NULL
);
