-- Your SQL goes here
drop table if exists user;
create table user (
  user_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL ,
  username TEXT NOT NULL,
  email TEXT NOT NULL,
  pw_hash TEXT NOT NULL
);

drop table if exists follower;
create table follower (
  who_id INTEGER NOT NULL,
  whom_id INTEGER NOT NULL,
  PRIMARY KEY (who_id, whom_id)
);

drop table if exists message;
create table message (
  message_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  author_id INTEGER NOT NULL,
  text TEXT NOT NULL,
  pub_date TEXT NOT NULL,
  flagged INTEGER NOT NULL
);
