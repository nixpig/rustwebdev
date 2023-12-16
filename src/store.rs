use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;

use crate::types::question::NewQuestion;
use crate::types::{
    answer::{Answer, AnswerId, NewAnswer},
    question::{Question, QuestionId},
};

#[derive(Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("unable to establish a database connection: {}", e),
        };

        Store {
            connection: db_pool,
        }
    }

    pub async fn get_questions(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<Question>, sqlx::Error> {
        match sqlx::query("select * from questions limit $1 offset $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(questions) => Ok(questions),
            Err(e) => Err(e),
        }
    }

    pub async fn add_question(&self, new_question: NewQuestion) -> Result<Question, sqlx::Error> {
        match sqlx::query("insert into questions (title, content, tags) values ($1, $2, $3) returning id, title, content, tags")
            .bind(new_question.title)
            .bind(new_question.content)
            .bind(new_question.tags)
            .map(|row: PgRow| Question{
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags")
            })
                .fetch_one(&self.connection)
            .await {
                Ok(question) => Ok(question),
                Err(e) => Err(e)
            }
    }

    pub async fn update_question(
        &self,
        question: Question,
        question_id: i32,
    ) -> Result<Question, sqlx::Error> {
        match sqlx::query("update questions set title=$2, content=$3, tags=$4 where id=$1 returning id, title, content, tags")
            .bind(question_id)
            .bind(question.title)
            .bind(question.content)
            .bind(question.tags)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags")
            })
                .fetch_one(&self.connection)
                .await {
                Ok(question) => Ok(question),
                Err(e) => Err(e),
            }
    }

    pub async fn get_question_by_id(&self, question_id: i32) -> Result<Question, sqlx::Error> {
        match sqlx::query("select id, title, content, tags from questions where id=$1")
            .bind(question_id)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(question) => Ok(question),
            Err(e) => Err(e),
        }
    }

    pub async fn delete_question(&self, question_id: i32) -> Result<(), sqlx::Error> {
        match sqlx::query("delete from questions where id=$1")
            .bind(question_id)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn get_answers(&self) -> Result<Vec<Answer>, sqlx::Error> {
        match sqlx::query("select id, content, question_id from answers")
            .map(|row: PgRow| Answer {
                id: AnswerId(row.get("id")),
                content: row.get("content"),
                question_id: QuestionId(row.get("question_id")),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(answers) => Ok(answers),
            Err(e) => Err(e),
        }
    }

    pub async fn get_answer_by_id(&self, answer_id: i32) -> Result<Answer, sqlx::Error> {
        match sqlx::query("select id, content, question_id from answers where id=$1")
            .bind(answer_id)
            .map(|row: PgRow| Answer {
                id: AnswerId(row.get("id")),
                content: row.get("content"),
                question_id: QuestionId(row.get("question_id")),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(answer) => Ok(answer),
            Err(e) => Err(e),
        }
    }

    pub async fn update_answer(
        &self,
        answer: Answer,
        answer_id: i32,
    ) -> Result<Answer, sqlx::Error> {
        match sqlx::query(
            "update answers set content=$2 where id=$1 returning id, content, question_id",
        )
        .bind(answer_id)
        .bind(answer.content)
        .map(|row: PgRow| Answer {
            id: AnswerId(row.get("id")),
            content: row.get("content"),
            question_id: QuestionId(row.get("question_id")),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(answers) => Ok(answers),
            Err(e) => Err(e),
        }
    }

    pub async fn add_answer(
        &self,
        question_id: i32,
        answer: NewAnswer,
    ) -> Result<Answer, sqlx::Error> {
        match sqlx::query("insert into answers (content, question_id) values ($1, $2) returning id, content, question_id")
            .bind(answer.content)
            .bind(question_id)
            .map(|row: PgRow| Answer{
                id: AnswerId(row.get("id")),
                content: row.get(
                    "content"
                ),
                question_id: QuestionId(row.get("question_id")),
            })
                .fetch_one(&self.connection)
            .await {
                Ok(answer) => Ok(answer),
                Err(e) => Err(e),
            }
    }

    pub async fn delete_answer(&self, answer_id: i32) -> Result<(), sqlx::Error> {
        match sqlx::query("delete from answers where id=$1")
            .bind(answer_id)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn get_answers_for_question(
        &self,
        question_id: i32,
    ) -> Result<Vec<Answer>, sqlx::Error> {
        // TODO: offline compile time checking
        // cargo install sqlx-cli && DATABASE_URL=<db_url> cargo sqlx prepare
        // check .sqlx file into vcs
        // cargo sqlx prepare --check
        // match sqlx::query!("select id, content, question_id from answers where question_id=$1")
        match sqlx::query("select id, content, question_id from answers where question_id=$1")
            .bind(question_id)
            .map(|row: PgRow| Answer {
                id: AnswerId(row.get("id")),
                content: row.get("content"),
                question_id: QuestionId(row.get("question_id")),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(answers) => Ok(answers),
            Err(e) => Err(e),
        }
    }
}
