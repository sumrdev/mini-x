CREATE TABLE IF NOT EXISTS users (
  user_id SERIAL PRIMARY KEY,
  username TEXT NOT NULL,
  email TEXT NOT NULL,
  pw_hash TEXT NOT NULL
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
  text TEXT NOT NULL,
  pub_date TEXT NOT NULL,
  flagged INTEGER NOT NULL
);
