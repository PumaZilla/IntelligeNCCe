
-- Events

CREATE TABLE IF NOT EXISTS "events" (
  "id"              SERIAL,
  "template"        VARCHAR(255) NOT NULL DEFAULT '::unknown',
  "type"            INTEGER NOT NULL DEFAULT 0,
  "source"          TEXT NOT NULL,
  "data"            TEXT NOT NULL,
  "created_at"      TIMESTAMP NOT NULL DEFAULT NOW(),

  CONSTRAINT "events_pk" PRIMARY KEY ("id"),
  CONSTRAINT "events_uk_link" UNIQUE ("source","data"),
  CONSTRAINT "events_ck_type" CHECK ("type" >= 0 AND "type" <= 0)

);

-- Keywords

CREATE TABLE IF NOT EXISTS "keywords" (
  "id"              SERIAL PRIMARY KEY,
  "type"            INTEGER NOT NULL DEFAULT 0,
  "value"           VARCHAR(255) NOT NULL,
  "created_at"      TIMESTAMP NOT NULL DEFAULT NOW(),
  "last_consulted"  TIMESTAMP NOT NULL DEFAULT NOW(),

  CONSTRAINT "keywords_uk_link" UNIQUE ("value"),
  CONSTRAINT "keywords_ck_type" CHECK ("type" >= 0 AND "type" <= 6)
);


-- Relationships

CREATE TABLE IF NOT EXISTS "events_keywords" (
  "event"           INTEGER NOT NULL,
  "keyword"         INTEGER NOT NULL,

  CONSTRAINT "events_keywords_pk" PRIMARY KEY ("event","keyword"),
  CONSTRAINT "events_keywords_fk_event" FOREIGN KEY ("event") REFERENCES "events" ("id") ON UPDATE CASCADE ON DELETE CASCADE,
  CONSTRAINT "events_keywords_fk_keyword" FOREIGN KEY ("keyword") REFERENCES "keywords" ("id") ON UPDATE CASCADE ON DELETE CASCADE
);