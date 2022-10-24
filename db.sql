CREATE TABLE IF NOT EXISTS db_column_items
(
    ctm_id SERIAL PRIMARY KEY NOT NULL,
    ctm_name VARCHAR(255),
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc')
);

CREATE TABLE IF NOT EXISTS db_items
(
    itm_id SERIAL PRIMARY KEY NOT NULL,
    itm_name VARCHAR(255),
    itm_type VARCHAR(255),
    itm_code VARCHAR(10),
    itm_description TEXT,
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc')
);