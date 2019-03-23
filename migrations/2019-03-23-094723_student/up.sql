-- Your SQL goes here
create table if not exists student
(
  id           int auto_increment primary key,
  number       char(9),
  name         char(10),
  department   int,
  create_date  datetime default current_timestamp,
  last_updated datetime default current_timestamp on update current_timestamp,
  foreign key (department) references department (id) on update cascade on delete set null
) default charset = utf8