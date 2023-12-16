create table if not exists answers (
	id serial primary key,
	content text,
	question_id int references questions
)
