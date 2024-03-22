CREATE TABLE `Usuarios` (
  `id_usuario` int PRIMARY KEY AUTO_INCREMENT,
  `nombre` varchar(255),
  `correo_electronico` varchar(255),
  `contrasena` varchar(255)
);

CREATE TABLE `Peliculas` (
  `id_pelicula` int PRIMARY KEY AUTO_INCREMENT,
  `titulo` varchar(255),
  `ano_lanzamiento` int,
  `genero` varchar(255),
  `clasificacion` varchar(255),
  `id_director` int
);

CREATE TABLE `Directores` (
  `id_director` int PRIMARY KEY AUTO_INCREMENT,
  `nombre` varchar(255),
  `fecha_nacimiento` date,
  `nacionalidad` varchar(255)
);

CREATE TABLE `Actores` (
  `id_actor` int PRIMARY KEY AUTO_INCREMENT,
  `nombre` varchar(255),
  `fecha_nacimiento` date,
  `nacionalidad` varchar(255),
  `id_pelicula` int
);

CREATE TABLE `Comentarios` (
  `id_comentario` int PRIMARY KEY AUTO_INCREMENT,
  `id_usuario` int,
  `id_pelicula` int,
  `contenido` text,
  `fecha_creacion` datetime
);

ALTER TABLE `Peliculas` ADD FOREIGN KEY (`id_director`) REFERENCES `Directores` (`id_director`);

ALTER TABLE `Actores` ADD FOREIGN KEY (`id_pelicula`) REFERENCES `Peliculas` (`id_pelicula`);

ALTER TABLE `Comentarios` ADD FOREIGN KEY (`id_usuario`) REFERENCES `Usuarios` (`id_usuario`);

ALTER TABLE `Comentarios` ADD FOREIGN KEY (`id_pelicula`) REFERENCES `Peliculas` (`id_pelicula`);



-- Insert data into Usuarios
INSERT INTO Usuarios (nombre, correo_electronico, contrasena)
VALUES ('John Doe', 'johndoe@example.com', 'password123'),
       ('Jane Smith', 'janesmith@example.com', 'securepassword');

-- Insert data into Directores
INSERT INTO Directores (nombre, fecha_nacimiento, nacionalidad)
VALUES ('Steven Spielberg', '1946-12-18', 'Estadounidense'),
       ('Quentin Tarantino', '1963-03-27', 'Estadounidense');

-- Insert data into Peliculas
INSERT INTO Peliculas (titulo, ano_lanzamiento, genero, clasificacion, id_director)
VALUES ('Jurassic Park', 1993, 'Acción', 'PG-13', 1),
       ('Pulp Fiction', 1994, 'Crimen', 'R', 2);

-- Insert data into Actores
INSERT INTO Actores (nombre, fecha_nacimiento, nacionalidad, id_pelicula)
VALUES ('Sam Neill', '1947-09-14', 'Estadounidense', 1),
       ('John Travolta', '1954-02-18', 'Estadounidense', 2);

-- Insert data into Comentarios
INSERT INTO Comentarios (id_usuario, id_pelicula, contenido, fecha_creacion)
VALUES (1, 1, '¡Increíble! Me encantó la película.', NOW()),
       (2, 2, 'Muy buena película, me gustó mucho.', NOW());