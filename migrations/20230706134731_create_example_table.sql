CREATE TABLE IF NOT EXISTS example (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    uid uuid NOT NULL,
    ping TEXT NOT NULL,
    created_at timestamptz NOT NULL
);
