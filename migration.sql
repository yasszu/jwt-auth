CREATE TABLE accounts (
	id serial PRIMARY KEY,
    email VARCHAR ( 255 ) UNIQUE NOT NULL,
	password VARCHAR ( 50 ) NOT NULL,
	created_on TIMESTAMP NOT NULL DEFAULT current_timestamp
);