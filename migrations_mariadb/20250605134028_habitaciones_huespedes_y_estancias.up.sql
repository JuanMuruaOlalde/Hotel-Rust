-- Add up migration script here
CREATE TABLE IF NOT EXISTS `huespedes` (
	`id` UUID NOT NULL,
	`nombre_y_apellidos` TEXT NOT NULL,
	`nacionalidad` TINYTEXT NULL,
	`documento_de_id` TINYTEXT NOT NULL,
	`telefono` TINYTEXT NOT NULL,
	`correo_e` TINYTEXT NOT NULL,
  PRIMARY KEY (`id`)
)
COLLATE='utf8mb4_general_ci'
;
 
CREATE TABLE IF NOT EXISTS `habitaciones` (
	`id` UUID NOT NULL,
	`nombre` TINYTEXT NOT NULL,
	`tipo_de_habitacion` TINYTEXT NULL,
	`tipo_de_baño` TINYTEXT NULL,
  PRIMARY KEY (`id`)
)
COLLATE='utf8mb4_general_ci'
;

CREATE TABLE IF NOT EXISTS `estancias` (
	`id` UUID NULL,
	`entrada` DATETIME NULL,
	`salida_prevista` DATETIME NULL,
	`salida_real` DATETIME NULL,
  PRIMARY KEY (`id`)
)
COLLATE='utf8mb4_general_ci'
;

CREATE TABLE IF NOT EXISTS `estancias_habitaciones` (
	`id_estancia` UUID NULL,
	`id_habitacion` UUID NULL
)
COLLATE='utf8mb4_general_ci'
;

CREATE TABLE IF NOT EXISTS `estancias_huespedes` (
	`id_estancia` UUID NULL,
	`id_huesped` UUID NULL
)
COLLATE='utf8mb4_general_ci'
;
