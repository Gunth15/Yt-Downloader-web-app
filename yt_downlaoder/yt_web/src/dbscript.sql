DROP TABLE IF EXISTS users;

CREATE TABLE users (
  username  VARCHAR(30) NOT NULL,
  password VARCHAR(20) NOT NULL,
  user_id SERIAL PRIMARY KEY
);

GRANT ALL PRIVILEGES on table users to user_mn;
GRANT ALL PRIVILEGES on all sequences in schema public to user_mn;
