use diesel::result::Error;
use diesel::{Connection, PgConnection};

use crud_bd::crud::{chat::*, *};

const DEFAULT_TITLE: &str = "Ricado Burity Videos Satisfatorios";
const DEFAULT_IS_PUBLIC: bool = true;

fn create_common_chat(conn: &mut PgConnection) -> Chat {
    create_chat(conn, DEFAULT_TITLE, DEFAULT_IS_PUBLIC).unwrap()
}

fn is_common_chat(chat: &Chat) {
    assert!(chat.title.contains(DEFAULT_TITLE));
    assert!(chat.is_public == DEFAULT_IS_PUBLIC);
}

#[test]
fn chat_created() {
    let mut conn = establish_connection();

    conn.test_transaction::<_, Error, _>(|conn| {
        let chat = create_common_chat(conn);

        is_common_chat(&chat);

        Ok(())
    })
}

#[test]
fn find_chat() {
    let mut conn = establish_connection();

    conn.test_transaction::<_, Error, _>(|conn| {
        let created_chat = create_common_chat(conn);

        let found_chat_id = get_chat_by_id(conn, created_chat.id).unwrap();
        is_common_chat(&found_chat_id);

        Ok(())
    })
}

#[test]
fn modify_chat() {
    let mut conn = establish_connection();

    conn.test_transaction::<_, Error, _>(|conn| {
        let created_chat = create_common_chat(conn);

        let alt_title: &str = "Madeirada";
        let modified_chat_title = update_chat_title(conn, created_chat.id, alt_title).unwrap();
        assert!(modified_chat_title.title.contains(alt_title));

        Ok(())
    })
}

#[test]
fn remove_chat() {
    let mut conn = establish_connection();

    conn.test_transaction::<_, Error, _>(|conn| {
        let created_chat = create_common_chat(conn);

        let removed_chat = delete_chat(conn, created_chat.id).unwrap();
        is_common_chat(&removed_chat);

        Ok(())
    })
}
