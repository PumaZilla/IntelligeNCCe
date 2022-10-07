-- Keywords

-- CREATE TYPE KTYPE AS ENUM ('text','credential','domain','email','ip','phone','url','username'); 

CREATE TABLE IF NOT EXISTS "keywords" (
  "id"              SERIAL PRIMARY KEY,
  "type"            VARCHAR(50) NOT NULL DEFAULT 'text',
  "value"           VARCHAR(255) NOT NULL,
  "created_at"      TIMESTAMP NOT NULL DEFAULT NOW(),
  "last_consulted"  TIMESTAMP NOT NULL DEFAULT NOW(),

  CONSTRAINT "keywords_uk_link" UNIQUE ("value")
);

-- Events

-- CREATE TYPE ETYPE AS ENUM ('paste'); 

CREATE TABLE IF NOT EXISTS "events" (
  "id"              SERIAL,
  "template"        VARCHAR(255) NOT NULL DEFAULT '::unknown',
  "type"            VARCHAR(50) NOT NULL DEFAULT 'paste',
  "source"          TEXT NOT NULL,
  "data"            TEXT NOT NULL,
  "created_at"      TIMESTAMP NOT NULL DEFAULT NOW(),

  CONSTRAINT "events_pk" PRIMARY KEY ("id"),
  CONSTRAINT "events_uk_link" UNIQUE ("source","data")
);

-- Relationships

CREATE TABLE IF NOT EXISTS "events_keywords" (
  "event"           INTEGER NOT NULL,
  "keyword"         INTEGER NOT NULL,

  CONSTRAINT "events_keywords_pk" PRIMARY KEY ("event","keyword"),
  CONSTRAINT "events_keywords_fk_event" FOREIGN KEY ("event") REFERENCES "events" ("id") ON UPDATE CASCADE ON DELETE CASCADE,
  CONSTRAINT "events_keywords_fk_keyword" FOREIGN KEY ("keyword") REFERENCES "keywords" ("id") ON UPDATE CASCADE ON DELETE CASCADE
);