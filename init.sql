CREATE DATABASE IF NOT EXISTS rust_db;
USE rust_db;

CREATE TABLE IF NOT EXISTS users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL
);

-- Insert a default user if the table is empty
INSERT INTO users (first_name, last_name)
SELECT 'John', 'Doe' FROM DUAL
WHERE NOT EXISTS (SELECT 1 FROM users LIMIT 1);