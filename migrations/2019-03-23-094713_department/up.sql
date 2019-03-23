-- Your SQL goes here
create table if not exists department
(
  id           int auto_increment primary key,
  name         varchar(50) unique,
  create_date  datetime default current_timestamp,
  last_updated datetime default current_timestamp on update current_timestamp
) default charset = utf8
