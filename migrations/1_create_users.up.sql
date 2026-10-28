CREATE TABLE users
(
    id                  BIGSERIAL PRIMARY KEY,
    first_name          VARCHAR(30) NOT NULL,
    last_name           VARCHAR(30) NOT NULL,
    password            VARCHAR(80) NOT NULL,
    email               VARCHAR(50) NOT NULL UNIQUE,
    created             TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated             TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_login          TIMESTAMP,
    date_of_birth       DATE,
    height              DOUBLE PRECISION,
    weight              DOUBLE PRECISION
);

INSERT INTO users (first_name, last_name, password, email, last_login, date_of_birth, height, weight)
VALUES ('John', 'Doe', 'password123', 'john.doe@example.com', NOW() - INTERVAL '2 days', '1990-05-14', 180.5, 78.2),
       ('Anna', 'Smith', 'qwerty456', 'anna.smith@example.com', NULL, '1995-11-02', 165.0, 58.4),
       ('Ivan', 'Petrenko', 'secret789', 'ivan.petrenko@example.com', NOW() - INTERVAL '1 hour', '1987-03-27', 175.3, 82.0);