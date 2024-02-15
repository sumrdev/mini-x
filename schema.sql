drop table if exists user;
create table user (
  user_id integer primary key autoincrement,
  username TEXT not null,
  email TEXT not null,
  pw_hash TEXT not null
);

drop table if exists follower;
create table follower (
  who_id integer,
  whom_id integer
);

drop table if exists message;
create table message (
  message_id integer primary key autoincrement,
  author_id integer not null,
  text TEXT not null,
  pub_date integer,
  flagged integer
);
