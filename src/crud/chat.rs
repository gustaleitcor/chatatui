use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::chats::dsl::*;
use crate::{crud::participants, schema::chats};
use diesel::{Insertable, Queryable};

#[derive(Queryable, Debug)]
pub struct Chat {
    pub id: i32,
    pub title: String,
    pub is_public: bool,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = chats)]
struct NewChat<'a> {
    pub title: &'a str,
    pub is_public: bool,
}

pub fn number_of_chats_with_title(conn: &mut PgConnection, title_to_check: &str) -> i64 {
    use diesel::dsl::count_star;

    let count: i64 = chats
        .filter(title.eq(title_to_check))
        .select(count_star())
        .first(conn)
        .expect("Error loading chat count");

    count
}

pub fn create_chat(
    conn: &mut PgConnection,
    chat_title: &str,
    chat_is_public: bool,
) -> QueryResult<Chat> {
    let new_chat = NewChat {
        title: chat_title,
        is_public: chat_is_public,
    };

    diesel::insert_into(chats)
        .values(&new_chat)
        .get_result(conn)
}

pub fn get_chat_by_id(conn: &mut PgConnection, chat_id: i32) -> QueryResult<Chat> {
    chats.find(chat_id).first(conn)
}

pub fn get_chats_of_user(conn: &mut PgConnection, user_id: i32) -> QueryResult<Vec<Chat>> {
    use crate::schema::participants;
    use crate::schema::participants::dsl as participants_dsl;

    chats
        .inner_join(participants_dsl::participants.on(participants::chat_id.eq(id)))
        .filter(participants::user_id.eq(user_id))
        .select((id, title, is_public))
        .load(conn)
}

// general purpose function
pub fn get_chats_with_pagination(
    conn: &mut PgConnection,
    offset: i64,
    limit: i64,
    filter_id: Option<i32>,
    filter_title: Option<String>,
) -> QueryResult<Vec<Chat>> {
    let mut query = chats.into_boxed();

    if let Some(filter_id) = filter_id {
        query = query.filter(id.eq(filter_id));
    }

    if let Some(filter_title) = filter_title {
        let filter_title: String = format!("%{filter_title}%");
        query = query.filter(title.ilike(filter_title));
    }

    query.offset(offset).limit(limit).load::<Chat>(conn)
}

// maybe not needed anymore
pub fn get_chat_by_title(conn: &mut PgConnection, chat_title: &str) -> QueryResult<Vec<Chat>> {
    chats.filter(title.eq(chat_title)).load::<Chat>(conn)
}

pub fn update_chat_title(
    conn: &mut PgConnection,
    chat_id: i32,
    new_title: &str,
) -> QueryResult<Chat> {
    diesel::update(chats.find(chat_id))
        .set(title.eq(new_title))
        .get_result(conn)
}

pub fn update_chat_privacy(
    conn: &mut PgConnection,
    chat_id: i32,
    new_privacy: bool,
) -> QueryResult<Chat> {
    diesel::update(chats.find(chat_id))
        .set(is_public.eq(new_privacy))
        .get_result(conn)
}

pub fn delete_chat(conn: &mut PgConnection, chat_id: i32) -> QueryResult<Chat> {
    diesel::delete(chats.find(chat_id)).get_result(conn)
}
