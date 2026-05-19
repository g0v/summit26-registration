CREATE TEMP TABLE temp_updates (
    ticket_id TEXT,
    nickname TEXT
);
\copy temp_updates(ticket_id, nickname) FROM '/tmp/data.csv' DELIMITER ',' CSV HEADER;

UPDATE workers
SET 
    ticket_id = temp.ticket_id
FROM temp_updates AS temp
WHERE workers.nickname = temp.nickname;
