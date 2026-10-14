CREATE TABLE IF NOT EXISTS books (
    id          BIGSERIAL PRIMARY KEY,
    isbn        VARCHAR(13) UNIQUE NOT NULL,
    title       TEXT        NOT NULL,
    description TEXT        NOT NULL DEFAULT '',
    published   INT         NOT NULL
);

