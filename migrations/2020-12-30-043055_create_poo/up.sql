CREATE TABLE poo_form(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);
INSERT INTO poo_form (id, name) VALUES (1, 'normal');
INSERT INTO poo_form (id, name) VALUES (2, 'hard');
INSERT INTO poo_form (id, name) VALUES (3, 'soft');
INSERT INTO poo_form (id, name) VALUES (4, 'very hard');
INSERT INTO poo_form (id, name) VALUES (5, 'diarrhea');

CREATE TABLE poo_color(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);
INSERT INTO poo_color (id, name) VALUES (1, 'brown');
INSERT INTO poo_color (id, name) VALUES (2, 'black');
INSERT INTO poo_color (id, name) VALUES (3, 'white');
INSERT INTO poo_color (id, name) VALUES (4, 'red');
INSERT INTO poo_color (id, name) VALUES (5, 'green');

CREATE TABLE poo_bleeding(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);
INSERT INTO poo_bleeding (id, name) VALUES (1, 'none');
INSERT INTO poo_bleeding (id, name) VALUES (2, 'little');
INSERT INTO poo_bleeding (id, name) VALUES (3, 'heavy');

CREATE TABLE poo(
    id SERIAL PRIMARY KEY,
    form SERIAL REFERENCES poo_form(id),
    color SERIAL REFERENCES poo_color(id),
    bleeding SERIAL REFERENCES poo_bleeding(id),
    required_time TIME NOT NULL,
    published_at TIMESTAMP NOT NULL
);
