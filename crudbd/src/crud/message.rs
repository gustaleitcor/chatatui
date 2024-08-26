use crate::schema;
use diesel::prelude::*;

use self::schema::messages::dsl::*;
use crate::schema::messages;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{Insertable, Queryable};

#[derive(Queryable, Debug)]
pub struct Message {
    pub id: i32,
    pub content: String,
    pub chat_id: i32,
    pub user_id: Option<i32>,
    pub date: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = messages)]
struct NewMessage<'a> {
    pub content: &'a str,
    pub user_id: i32,
    pub chat_id: i32,
    pub date: Option<NaiveDateTime>,
}

pub fn create_message(
    conn: &mut PgConnection,
    cont: &str,
    id_user: i32,
    id_chat: i32,
    time: Option<NaiveDateTime>,
) -> QueryResult<Message> {
    let new_message = NewMessage {
        content: cont,
        user_id: id_user,
        chat_id: id_chat,
        date: time,
    };

    diesel::insert_into(messages)
        .values(&new_message)
        .get_result(conn)
}

// general purpose function
pub fn get_messages_with_pagination(
    conn: &mut PgConnection,
    offset: i64,
    limit: i64,
    filter_msg_id: Option<i32>,
    filter_user_id: Option<i32>,
    filter_chat_id: Option<i32>,
    filter_msg_date: Option<NaiveDate>,
) -> QueryResult<Vec<Message>> {
    let mut query = messages.into_boxed();

    if let Some(filter_msg_id) = filter_msg_id {
        query = query.filter(id.eq(filter_msg_id));
    }

    if let Some(filter_user_id) = filter_user_id {
        query = query.filter(user_id.eq(filter_user_id));
    }

    if let Some(filter_chat_id) = filter_chat_id {
        query = query.filter(chat_id.eq(filter_chat_id));
    }

    if let Some(filter_msg_date) = filter_msg_date {
        let day_begin: NaiveDateTime = filter_msg_date.and_hms_opt(0, 0, 0).unwrap();
        let day_end: NaiveDateTime = filter_msg_date.and_hms_opt(23, 59, 59).unwrap();
        query = query.filter(date.ge(day_begin)).filter(date.le(day_end));
    }

    query.offset(offset).limit(limit).load::<Message>(conn)
}

pub fn get_message_by_id(conn: &mut PgConnection, id_message: i32) -> QueryResult<Message> {
    messages.find(id_message).first(conn)
}

// maybe not needed anymore
pub fn get_messages_by_user_id(conn: &mut PgConnection, id_user: i32) -> QueryResult<Vec<Message>> {
    messages.filter(user_id.eq(id_user)).load::<Message>(conn)
}

// maybe not needed anymore
pub fn get_messages_by_chat_id(conn: &mut PgConnection, id_chat: i32) -> QueryResult<Vec<Message>> {
    messages.filter(chat_id.eq(id_chat)).load::<Message>(conn)
}

// maybe not needed anymore
pub fn get_messages_by_day(conn: &mut PgConnection, day: NaiveDate) -> QueryResult<Vec<Message>> {
    let day_begin: NaiveDateTime = day.and_hms_opt(0, 0, 0).unwrap();
    let day_end: NaiveDateTime = day.and_hms_opt(23, 59, 59).unwrap();

    messages
        .filter(date.ge(day_begin))
        .filter(date.le(day_end))
        .load::<Message>(conn)
}

pub fn delete_message(conn: &mut PgConnection, id_message: i32) -> QueryResult<Message> {
    diesel::delete(messages.find(id_message)).get_result(conn)
}
