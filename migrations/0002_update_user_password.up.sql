ALTER TABLE users RENAME COLUMN password to password_hash;
ALTER TABLE users ALTER COLUMN password_hash TYPE TEXT;
