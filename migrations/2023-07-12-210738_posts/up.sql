-- Your SQL goes here
CREATE TABLE posts (
  id INT PRIMARY KEY NOT NULL,
  user_id INT NOT NULL,
  stored_on_server BOOLEAN NOT NULL,
  LOCATION TEXT NULL NULL
)