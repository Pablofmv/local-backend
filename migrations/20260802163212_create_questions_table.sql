CREATE TABLE questions (
    id SERIAL PRIMARY KEY,

    user_id INTEGER NOT NULL,

    title TEXT NOT NULL,
    body TEXT NOT NULL,
    category TEXT NOT NULL,

    community TEXT NOT NULL DEFAULT 'PERUVIAN',
    region TEXT NOT NULL DEFAULT 'REGION1',
    state TEXT NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
