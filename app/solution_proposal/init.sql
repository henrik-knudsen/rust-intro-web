SELECT 'CREATE DATABASE app'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'app')\gexec

DROP TABLE IF EXISTS Movie;

CREATE TABLE IF NOT EXISTS Movie(

	id serial PRIMARY KEY,
	title VARCHAR(200) NOT NULL,
	release_date Date NOT NULL
);

INSERT INTO movie(title, release_date) VALUES('The Lord of the Rings: The Fellowship of the Ring', '2001-12-10');
INSERT INTO movie(title, release_date) VALUES('The Lord of the Rings: The Two Towers', '2002-12-05');
INSERT INTO movie(title, release_date) VALUES('The Lord of the Rings: The Return of the King', '2003-12-01');


