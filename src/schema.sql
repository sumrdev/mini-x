CREATE TABLE IF NOT EXISTS users (
  user_id SERIAL PRIMARY KEY,
  username VARCHAR(100)  NOT NULL,
  email VARCHAR(100)  NOT NULL,
  pw_hash VARCHAR(100) NOT NULL
);

CREATE TABLE IF NOT EXISTS followers (
  who_id INTEGER,
  whom_id INTEGER,
  FOREIGN KEY (who_id) REFERENCES users (user_id),
  FOREIGN KEY (whom_id) REFERENCES users (user_id),
  PRIMARY KEY (who_id, whom_id)
);

CREATE TABLE IF NOT EXISTS messages (
  message_id SERIAL PRIMARY KEY,
  author_id INTEGER NOT NULL,
  text VARCHAR(255) NOT NULL,
  pub_date VARCHAR(255),
  flagged INTEGER
);
