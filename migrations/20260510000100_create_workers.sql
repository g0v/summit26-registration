CREATE TABLE IF NOT EXISTS workers (
    ticket_id TEXT PRIMARY KEY,
    nickname TEXT NOT NULL,
    team TEXT NOT NULL,
    role TEXT NOT NULL,
    email TEXT,
    tshirt TEXT,
    diet TEXT,
    registered BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- INSERT INTO workers (ticket_id, nickname, team, role)
-- VALUES
--     ('CONF-0420', '沒有人', '自然組', '小農'),
--     ('CONF-1337', 'Heisenberg W.', '化學組', '組長'),
--     ('CONF-5489', 'JoJo', '社會組', '組頭')
-- ON CONFLICT (ticket_id) DO NOTHING;
