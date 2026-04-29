CREATE TABLE IF NOT EXISTS attendees (
    ticket_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    ticket_type TEXT NOT NULL,
    registered BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO attendees (ticket_id, name, ticket_type, registered)
VALUES
    ('CONF-1027', 'Maya Chen', 'Speaker', TRUE),
    ('CONF-1184', 'Owen Patel', 'VIP', FALSE),
    ('CONF-1266', 'Lina Morales', 'General', FALSE),
    ('CONF-1315', 'Noah Williams', 'Workshop', TRUE),
    ('CONF-1442', 'Ari Tanaka', 'General', FALSE),
    ('CONF-1503', 'Sam Rivera', 'Sponsor', FALSE),
    ('CONF-1638', 'Priya Shah', 'VIP', TRUE),
    ('CONF-1790', 'Theo Brooks', 'General', FALSE)
ON CONFLICT (ticket_id) DO NOTHING;
