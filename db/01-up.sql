-- Entities

CREATE TABLE IF NOT EXISTS "event" (
  "id"        SERIAL,
  "source"    VARCHAR(255) NOT NULL DEFAULT '::unknown',
  "type"      VARCHAR(100) NOT NULL DEFAULT 'source',
  "location"  TEXT NOT NULL,
  "data"      TEXT NOT NULL,

  CONSTRAINT "event_pk" PRIMARY KEY ("id"),
  CONSTRAINT "event_uk_link" UNIQUE ("location","data")
);