-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS trigger_increase_bill ON messages;
DROP FUNCTION IF EXISTS trigger_increase_bill();
