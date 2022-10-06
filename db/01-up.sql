-- Keywords

CREATE TYPE KTYPE AS ENUM ('text','credential','domain','email','ip','phone','url','username'); 

CREATE TABLE IF NOT EXISTS "keyword" (
  "id"              SERIAL PRIMARY KEY,
  "type"            KTYPE NOT NULL DEFAULT 'text',
  "value"           VARCHAR(255) NOT NULL,
  "created_at"      TIMESTAMP NOT NULL DEFAULT NOW(),
  "last_consulted"  TIMESTAMP NOT NULL DEFAULT NOW(),

  CONSTRAINT "keyword_uk_link" UNIQUE ("value")
);

-- Events

CREATE TYPE ETYPE AS ENUM ('paste'); 

CREATE TABLE IF NOT EXISTS "event" (
  "id"              SERIAL,
  "template"        VARCHAR(255) NOT NULL DEFAULT '::unknown',
  "type"            ETYPE NOT NULL DEFAULT 'paste',
  "source"          TEXT NOT NULL,
  "data"            TEXT NOT NULL,
  "created_at"      TIMESTAMP NOT NULL DEFAULT NOW(),

  CONSTRAINT "event_pk" PRIMARY KEY ("id"),
  CONSTRAINT "event_uk_link" UNIQUE ("source","data")
);