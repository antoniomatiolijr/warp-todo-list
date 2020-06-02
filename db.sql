CREATE TABLE IF NOT EXISTS todo (
  id SERIAL PRIMARY KEY NOT NULL,
  name TEXT,
  created_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
  checked boolean DEFAULT false
);
