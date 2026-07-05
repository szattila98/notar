CREATE TABLE notes (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    content JSONB not null default '{}'::jsonb,
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
