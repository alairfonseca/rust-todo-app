-- Your SQL goes here
CREATE TABLE tasks (
  id SERIAL NOT NULL PRIMARY KEY,
  board_id INTEGER NOT NULL,
  title VARCHAR NOT NULL,
  description TEXT NOT NULL
);
