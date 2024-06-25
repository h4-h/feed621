ALTER TABLE topics RENAME TO topics_old;

CREATE TABLE IF NOT EXISTS topics (
    id             BIGINT     PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name           TEXT       NOT NULL,
    query          TEXT       NOT NULL
);

INSERT INTO topics (id, name, query)
    OVERRIDING SYSTEM VALUE
    SELECT id, 'PLEASE SET NAME', query FROM topics_old;

DROP TABLE topics_old CASCADE;
