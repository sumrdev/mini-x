-- For followers table
ALTER TABLE follower RENAME TO followers;

CREATE TABLE IF NOT EXISTS new_followers (
  who_id INTEGER,
  whom_id INTEGER
);

INSERT INTO new_followers (who_id, whom_id) 
SELECT who_id, whom_id
FROM followers;

DROP TABLE followers;

ALTER TABLE new_followers RENAME TO followers;

-- For messages table
ALTER TABLE message RENAME TO messages;

CREATE TABLE IF NOT EXISTS new_messages (
  message_id INTEGER PRIMARY KEY AUTOINCREMENT,
  author_id INTEGER NOT NULL,
  text TEXT NOT NULL,
  pub_date TEXT,
  flagged INTEGER
);

INSERT INTO new_messages (message_id, author_id, text, pub_date, flagged) 
SELECT message_id, author_id, text, pub_date, flagged
FROM messages;

DROP TABLE messages;

ALTER TABLE new_messages RENAME TO messages;

-- For users table
ALTER TABLE user RENAME TO users;

CREATE TABLE IF NOT EXISTS new_users (
  user_id INTEGER PRIMARY KEY AUTOINCREMENT,
  username TEXT NOT NULL,
  email TEXT NOT NULL,
  pw_hash TEXT NOT NULL
);

INSERT INTO new_users (user_id, username, email, pw_hash) 
SELECT user_id, username, email, pw_hash
FROM users;

DROP TABLE users;

ALTER TABLE new_users RENAME TO users;