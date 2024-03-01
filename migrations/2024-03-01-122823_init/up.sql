-- Your SQL goes here
create table user (
  user_id integer not null primary key autoincrement,
  username TEXT not null,
  email TEXT not null,
  pw_hash TEXT not null
);

create table follower (
  who_id integer not null,
  whom_id integer not null,
  PRIMARY KEY (who_id, whom_id)
);

create table message (
  message_id integer primary key autoincrement,
  author_id integer not null,
  text TEXT not null,
  pub_date integer,
  flagged integer
);
