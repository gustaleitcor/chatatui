use diesel::prelude::*;

use crate::schema::participants;
use crate::schema::participants::dsl::*;
use diesel::{Insertable, Queryable};

#[derive(Queryable, Debug)]
pub struct Participant {
    pub id: i32,
    pub chat_id: i32,
    pub user_id: i32,
    pub is_admin: bool,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = participants)]
struct NewParticipant {
    pub chat_id: i32,
    pub user_id: i32,
    pub is_admin: bool,
}

pub fn create_participant(
    conn: &mut PgConnection,
    input_chat_id: i32,
    input_user_id: i32,
    input_is_admin: bool,
) -> QueryResult<Participant> {
    let new_message = NewParticipant {
        chat_id: input_chat_id,
        user_id: input_user_id,
        is_admin: input_is_admin,
    };

    diesel::insert_into(participants)
        .values(&new_message)
        .get_result(conn)
}

pub fn get_participant_by_id(conn: &mut PgConnection, id_message: i32) -> QueryResult<Participant> {
    participants.find(id_message).first(conn)
}

pub fn get_participants_by_chat_and_user(
    conn: &mut PgConnection,
    id_user: i32,
    id_chat: i32,
) -> QueryResult<Participant> {
    participants
        .filter(user_id.eq(id_user))
        .filter(chat_id.eq(id_chat))
        .first(conn)
}

pub fn get_participants_by_user_id(
    conn: &mut PgConnection,
    id_user: i32,
) -> QueryResult<Vec<Participant>> {
    participants
        .filter(user_id.eq(id_user))
        .load::<Participant>(conn)
}

pub fn get_participants_by_chat_id(
    conn: &mut PgConnection,
    id_chat: i32,
) -> QueryResult<Vec<Participant>> {
    participants
        .filter(chat_id.eq(id_chat))
        .load::<Participant>(conn)
}
