-- Remove foreign keys from students table
ALTER TABLE students DROP COLUMN IF EXISTS created_by;
ALTER TABLE students DROP COLUMN IF EXISTS updated_by;
ALTER TABLE students DROP COLUMN IF EXISTS created_at;
ALTER TABLE students DROP COLUMN IF EXISTS updated_at;

-- Drop tables in reverse order of creation to avoid foreign key constraint errors
DROP TABLE IF EXISTS audit_logs;
DROP TABLE IF EXISTS user_tokens;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS roles; 