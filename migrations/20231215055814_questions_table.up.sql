create table if not exists questions (
	id serial primary key,
	title varchar(255) not null,
	content text,
	tags text [],
	created_on timestamp not null default now()
);

