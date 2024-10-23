-- Your SQL goes here
CREATE OR REPLACE FUNCTION increase_user_bill(IN user_id INT)
RETURNS VOID
LANGUAGE plpgsql
AS $$
BEGIN
    UPDATE users
    SET bill = bill + 0.5
    WHERE id = user_id;
END;
$$;