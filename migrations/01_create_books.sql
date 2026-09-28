CREATE TABLE IF NOT EXISTS books (
    id          SERIAL PRIMARY KEY,
    isbn        VARCHAR(13) UNIQUE NOT NULL,
    title       TEXT        NOT NULL,
    description TEXT        NOT NULL DEFAULT '',
    published   INT         NOT NULL
);

INSERT INTO books (isbn, title, description, published) VALUES
    ('9780451419439', 'Les Miserables', 'A sweeping tale of love, loss, valor, and passion', 2013),
    ('9780374533557', 'Thinking, Fast and Slow', 'Why do we make the decisions we do? Nobel Prize winner Daniel Kahneman revolutionised our understanding of human behaviour with Thinking, Fast and Slow. Distilling his life s work, Kahneman showed that there are two ways we make choices: fast, intuitive thinking, and slow, rational thinking. His book reveals how our minds are tripped up by error, bias and prejudice (even when we think we are being logical) and gives practical techniques that enable us all to improve our decision-making. This profound exploration of the marvels and limitations of the human mind has had a lasting impact on how we see ourselves.', 2013);

CREATE TABLE IF NOT EXISTS authors (
    id          SERIAL PRIMARY KEY,
    first_name  TEXT    NOT NULL,
    last_name   TEXT    NOT NULL
);

INSERT INTO authors (first_name, last_name) VALUES
    ('Victor', 'Hugo'),
    ('Daniel', 'Kahneman');

CREATE TABLE IF NOT EXISTS book_authors (
    book_id     INT    NOT NULL REFERENCES books(id) ON DELETE CASCADE,
    author_id   INT    NOT NULL REFERENCES authors(id) ON DELETE CASCADE,
    PRIMARY KEY (book_id, author_id)
);
