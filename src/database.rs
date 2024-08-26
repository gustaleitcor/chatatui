use std::io::ErrorKind;

use chrono::{NaiveDate, NaiveDateTime};
use crud_bd::crud::{chat::Chat, establish_connection, message::Message, user::User};
use diesel::{result::Error, PgConnection, QueryResult};

pub struct Database {
    pg_conn: PgConnection,
}

impl Database {
    pub fn new() -> Database {
        Database {
            pg_conn: establish_connection(),
        }
    }

    pub fn create_user(&mut self, user_username: &str, user_password: &str) -> QueryResult<User> {
        crud_bd::crud::user::create_user(&mut self.pg_conn, user_username, user_password)
    }

    pub fn delete_user(&mut self, user_id: i32) -> QueryResult<User> {
        crud_bd::crud::user::delete_user(&mut self.pg_conn, user_id)
    }

    pub fn update_user_username(&mut self, user_id: i32, new_username: &str) -> QueryResult<User> {
        crud_bd::crud::user::update_user_username(&mut self.pg_conn, user_id, new_username)
    }

    pub fn update_user_password(&mut self, user_id: i32, new_password: &str) -> QueryResult<User> {
        crud_bd::crud::user::update_user_password(&mut self.pg_conn, user_id, new_password)
    }

    pub fn fetch_users(
        &mut self,
        limit: i64,
        cursor: i64,
        username_filter: Option<String>,
        id_filter: Option<String>,
    ) -> QueryResult<Vec<User>> {
        let id_filter = id_filter
            .map(|e| {
                if e.is_empty() {
                    None
                } else {
                    Some(match e.parse::<i32>() {
                        Ok(e) => Some(e),
                        Err(_) => None,
                    })
                }
            })
            .flatten()
            .flatten();

        crud_bd::crud::user::get_users_with_pagination(
            &mut self.pg_conn,
            cursor,
            limit,
            username_filter,
            id_filter,
        )
    }

    // TODO: Remove cursor changes from here change the cursor
    pub fn next_users_page(
        &mut self,
        limit: i64,
        db_cursor: &mut i64,
        username_filter: Option<String>,
        id_filter: Option<String>,
    ) -> QueryResult<Vec<User>> {
        *db_cursor = db_cursor.saturating_add(limit);

        let users = self.fetch_users(limit, *db_cursor, username_filter, id_filter)?;

        *db_cursor = *db_cursor + users.len() as i64 - limit;

        Ok(users)
    }

    pub fn prev_users_page(
        &mut self,
        limit: i64,
        db_cursor: &mut i64,
        username_filter: Option<String>,
        id_filter: Option<String>,
    ) -> QueryResult<Vec<User>> {
        if *db_cursor - limit < 0 {
            *db_cursor = 0;
        } else {
            *db_cursor -= limit;
        }

        let users = self.fetch_users(limit, *db_cursor, username_filter, id_filter)?;

        if *db_cursor - limit < 0 {
            *db_cursor = 0;
        } else {
            *db_cursor = *db_cursor - users.len() as i64 + limit;
        }

        Ok(users)
    }

    #[allow(dead_code)]
    pub fn load_users(&mut self, limit: usize) {
        crud_bd::crud::populate_users(&mut self.pg_conn, limit);
    }

    pub fn fetch_chats(
        &mut self,
        limit: i64,
        cursor: i64,
        title_filter: Option<String>,
        id_filter: Option<String>,
    ) -> QueryResult<Vec<Chat>> {
        let id_filter = id_filter
            .map(|e| {
                if e.is_empty() {
                    None
                } else {
                    Some(match e.parse::<i32>() {
                        Ok(e) => Some(e),
                        Err(_) => None,
                    })
                }
            })
            .flatten()
            .flatten();

        crud_bd::crud::chat::get_chats_with_pagination(
            &mut self.pg_conn,
            cursor,
            limit,
            id_filter,
            title_filter,
        )
    }

    // TODO: Remove cursor changes from here change the cursor
    pub fn next_chats_page(
        &mut self,
        limit: i64,
        db_cursor: &mut i64,
        username_filter: Option<String>,
        id_filter: Option<String>,
    ) -> QueryResult<Vec<Chat>> {
        *db_cursor = db_cursor.saturating_add(limit);

        let chats = self.fetch_chats(limit, *db_cursor, username_filter, id_filter)?;

        *db_cursor = *db_cursor + chats.len() as i64 - limit;

        Ok(chats)
    }

    pub fn prev_chats_page(
        &mut self,
        limit: i64,
        db_cursor: &mut i64,
        username_filter: Option<String>,
        id_filter: Option<String>,
    ) -> QueryResult<Vec<Chat>> {
        if *db_cursor - limit < 0 {
            *db_cursor = 0;
        } else {
            *db_cursor -= limit;
        }

        let chats = self.fetch_chats(limit, *db_cursor, username_filter, id_filter)?;

        if *db_cursor - limit < 0 {
            *db_cursor = 0;
        } else {
            *db_cursor = *db_cursor - chats.len() as i64 + limit;
        }

        Ok(chats)
    }

    #[allow(dead_code)]
    pub fn load_chats(&mut self, limit: usize) {
        crud_bd::crud::populate_chats(&mut self.pg_conn, limit);
    }

    pub fn create_chat(&mut self, chat_name: &str, is_public: bool) -> QueryResult<Chat> {
        crud_bd::crud::chat::create_chat(&mut self.pg_conn, chat_name, is_public)
    }

    pub fn delete_chat(&mut self, chat_id: i32) -> QueryResult<Chat> {
        crud_bd::crud::chat::delete_chat(&mut self.pg_conn, chat_id)
    }

    pub fn update_chat_title(&mut self, chat_id: i32, new_title: &str) -> QueryResult<Chat> {
        crud_bd::crud::chat::update_chat_title(&mut self.pg_conn, chat_id, new_title)
    }

    pub fn update_chat_privacy(&mut self, chat_id: i32, is_public: bool) -> QueryResult<Chat> {
        crud_bd::crud::chat::update_chat_privacy(&mut self.pg_conn, chat_id, is_public)
    }

    pub fn fetch_messages(
        &mut self,
        limit: i64,
        cursor: i64,
        message_id: Option<String>,
        user_id: Option<String>,
        chat_id: Option<String>,
        message_date: Option<String>,
    ) -> QueryResult<Vec<Message>> {
        let message_id = message_id
            .map(|e| {
                if e.is_empty() {
                    None
                } else {
                    Some(match e.parse::<i32>() {
                        Ok(e) => Some(e),
                        Err(_) => None,
                    })
                }
            })
            .flatten()
            .flatten();

        let user_id = user_id
            .map(|e| {
                if e.is_empty() {
                    None
                } else {
                    Some(match e.parse::<i32>() {
                        Ok(e) => Some(e),
                        Err(_) => None,
                    })
                }
            })
            .flatten()
            .flatten();

        let chat_id = chat_id
            .map(|e| {
                if e.is_empty() {
                    None
                } else {
                    Some(match e.parse::<i32>() {
                        Ok(e) => Some(e),
                        Err(_) => None,
                    })
                }
            })
            .flatten()
            .flatten();

        let message_date = message_date
            .map(|e| {
                if e.is_empty() {
                    None
                } else {
                    Some(match e.parse::<NaiveDate>() {
                        Ok(e) => Some(e),
                        Err(_) => None,
                    })
                }
            })
            .flatten()
            .flatten();

        crud_bd::crud::message::get_messages_with_pagination(
            &mut self.pg_conn,
            cursor,
            limit,
            message_id,
            user_id,
            chat_id,
            message_date,
        )
    }

    pub fn next_messages_page(
        &mut self,
        limit: i64,
        db_cursor: &mut i64,
        message_id: Option<String>,
        user_id: Option<String>,
        chat_id: Option<String>,
        message_date: Option<String>,
    ) -> QueryResult<Vec<Message>> {
        *db_cursor = db_cursor.saturating_add(limit);

        let messages = self.fetch_messages(
            limit,
            *db_cursor,
            message_id,
            user_id,
            chat_id,
            message_date,
        )?;

        *db_cursor = *db_cursor + messages.len() as i64 - limit;

        Ok(messages)
    }

    pub fn prev_messages_page(
        &mut self,
        limit: i64,
        db_cursor: &mut i64,
        message_id: Option<String>,
        user_id: Option<String>,
        chat_id: Option<String>,
        message_date: Option<String>,
    ) -> QueryResult<Vec<Message>> {
        if *db_cursor - limit < 0 {
            *db_cursor = 0;
        } else {
            *db_cursor -= limit;
        }

        let messages = self.fetch_messages(
            limit,
            *db_cursor,
            message_id,
            user_id,
            chat_id,
            message_date,
        )?;

        if *db_cursor - limit < 0 {
            *db_cursor = 0;
        } else {
            *db_cursor = *db_cursor - messages.len() as i64 + limit;
        }

        Ok(messages)
    }

    pub fn create_message(
        &mut self,
        user_id: &str,
        chat_id: &str,
        message_text: &str,
    ) -> QueryResult<Message> {
        let user_id = match user_id.parse::<i32>() {
            Ok(e) => e,
            Err(_) => return Err(Error::NotFound),
        };

        let chat_id = match chat_id.parse::<i32>() {
            Ok(e) => e,
            Err(_) => return Err(Error::NotFound),
        };

        crud_bd::crud::message::create_message(
            &mut self.pg_conn,
            message_text,
            user_id,
            chat_id,
            None,
        )
    }

    pub fn delete_message(&mut self, message_id: i32) -> QueryResult<Message> {
        crud_bd::crud::message::delete_message(&mut self.pg_conn, message_id)
    }

    pub fn load_messages(&mut self, limit: usize) {
        crud_bd::crud::populate_message(&mut self.pg_conn, limit);
    }
}
