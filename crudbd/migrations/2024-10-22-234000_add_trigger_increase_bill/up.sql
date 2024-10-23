-- Your SQL goes here
CREATE OR REPLACE FUNCTION trigger_increase_bill()
RETURNS TRIGGER AS $$
DECLARE
    user_id_var INT;
BEGIN
    SELECT user_id INTO user_id_var
    FROM participants
    WHERE id = NEW.participant_id;

    PERFORM increase_user_bill(user_id_var);

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_increase_bill
AFTER INSERT ON messages
FOR EACH ROW
EXECUTE FUNCTION trigger_increase_bill();
