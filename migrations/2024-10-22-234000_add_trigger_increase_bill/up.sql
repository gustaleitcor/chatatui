-- Your SQL goes here
CREATE OR REPLACE FUNCTION trigger_increase_bill()
RETURNS TRIGGER AS $$
DECLARE
    user_id_var INT;
BEGIN
    -- Retrieve user_id from participants
    SELECT user_id INTO user_id_var
    FROM participants
    WHERE id = NEW.participant_id;

    -- Call the function to increase the user's bill
    UPDATE users
    SET bill = bill + 0.5
    WHERE id = user_id_var;

    -- Return the new row (even though not used in AFTER triggers)
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_increase_bill
AFTER INSERT ON messages
FOR EACH ROW
EXECUTE FUNCTION trigger_increase_bill();
