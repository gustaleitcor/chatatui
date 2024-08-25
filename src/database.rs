use crud_bd::crud::{chat::Chat, establish_connection, user::User};
use diesel::{PgConnection, QueryResult};

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
    pub fn load_users(&mut self) {
        crud_bd::crud::populate_users(&mut self.pg_conn, 1000);
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
    pub fn load_chats(&mut self) {
        crud_bd::crud::populate_chats(&mut self.pg_conn, 50);
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
}
