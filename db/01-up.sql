-- Entities

CREATE TABLE IF NOT EXISTS "object" (
  "id"        SERIAL,
  "source"    VARCHAR(255) NOT NULL DEFAULT '::unknown',
  "type"      VARCHAR(100) NOT NULL DEFAULT 'source',
  "location"  TEXT NOT NULL,
  "data"      TEXT NOT NULL,

  CONSTRAINT "object_pk" PRIMARY KEY ("id"),
  CONSTRAINT "object_uk_link" UNIQUE ("location","data")
);