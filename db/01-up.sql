-- Entities

CREATE TABLE IF NOT EXISTS "keyword" (
  "id"              SERIAL PRIMARY KEY,
  "type"            VARCHAR(255) NOT NULL,
  "value"           VARCHAR(255) NOT NULL,
  "created_at"      TIMESTAMP NOT NULL DEFAULT NOW(),
  "last_consulted"  TIMESTAMP NOT NULL DEFAULT NOW(),

  CONSTRAINT "keyword_uk_link" UNIQUE ("value")
);

CREATE TABLE IF NOT EXISTS "event" (
  "id"              SERIAL,
  "template"        VARCHAR(255) NOT NULL DEFAULT '::unknown',
  "type"            VARCHAR(100) NOT NULL DEFAULT 'source',
  "source"          TEXT NOT NULL,
  "data"            TEXT NOT NULL,
  "created_at"      TIMESTAMP NOT NULL DEFAULT NOW(),

  CONSTRAINT "event_pk" PRIMARY KEY ("id"),
  CONSTRAINT "event_uk_link" UNIQUE ("source","data")
);