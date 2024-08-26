pub mod chat;
pub mod message;
pub mod user;

use chat::create_chat;
use diesel::prelude::QueryableByName;
use diesel::{sql_query, RunQueryDsl};
use fake::faker::lorem::raw::*;
use fake::faker::name::raw::*;
use fake::locales::EN;
use fake::{locales::PT_BR, Fake};

use diesel::{pg::PgConnection, Connection};
use dotenvy::dotenv;
use message::create_message;
use std::env;
use user::create_user;

use crate::schema::{chats, users};

pub fn populate_users(conn: &mut PgConnection, num_users: usize) {
    for _ in 0..num_users {
        let username: String = Name(PT_BR).fake();
        let password: String = Word(EN).fake();

        let _ = create_user(conn, &username, &password);
    }
}

pub fn populate_chats(conn: &mut PgConnection, num_chats: usize) {
    for i in 0..num_chats {
        let title: String = Sentence(EN, 3..6).fake();
        let is_public: bool = i % 2 == 0;

        let _ = create_chat(conn, &title, is_public);
    }
}

pub fn populate_message(conn: &mut PgConnection, num_messages: usize) {

    //HACK: idk what i'm doing
    #[derive(QueryableByName, Debug)]
    #[diesel(table_name = users)]
    struct UserQuery {
        #[diesel(column_name = id)]
        pub id_query: i32,
        #[diesel(column_name = username)]
        pub _username_query: String,
        #[diesel(column_name = password)]
        pub _password_query: String,
    }
    #[derive(QueryableByName, Debug)]
    #[diesel(table_name = chats)]
    struct ChatQuery {
        #[diesel(column_name = id)]
        pub id_query: i32,
        #[diesel(column_name = title)]
        pub _title_query: String,
        #[diesel(column_name = is_public)]
        pub _is_public_query: bool,
    }
    for _ in 0..num_messages {
        let message: String = Sentence(EN, 1..20).fake();
        let user = sql_query("select * from users where id = (select id from users order by random() limit 1);
").get_results::<UserQuery>(conn);
        let user_id = user.unwrap()[0].id_query;

        let chat = sql_query("select * from chats where id = (select id from chats order by random() limit 1);
").get_results::<ChatQuery>(conn);
        let chat_id = chat.unwrap()[0].id_query;


        let _ = create_message(conn, message.as_str(), user_id, chat_id, None).unwrap();
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("You must set DATABASE_URL=postgres://$USER:$PASSWORD@localhost/crud_bd in .env!");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|err| panic!("Error connecting to {database_url}\n{err}"))
}
