-- Your SQL goes here

CREATE TABLE qrcode (
  id SERIAL,
  identifier VARCHAR(5) NOT NULL,
  link TEXT NOT NULL,
  PRIMARY KEY(id, identifier)
)