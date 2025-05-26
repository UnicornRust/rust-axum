
create table sys_user (
  id serial primary key,
  name varchar(255) not null unique,
  gender varchar(255) not null,
  account varchar(255) not null,
  password varchar(255) not null,
  mobile_phone varchar(255) not null,
  birthday date not null,
  enabled bool not null,
  created_at timestamp not null default now(),
  updated_at timestamp not null
);


insert into sys_user (name, gender, account, password, mobile_phone, birthday, enabled, created_at, updated_at) 
values 
  ('admin', 'male', 'admin', 'admin', '12345678', '1999-01-01', true, now(), now()),
  ('user', 'female', 'user', 'user', '12345678', '1999-01-01', true, now(), now());

