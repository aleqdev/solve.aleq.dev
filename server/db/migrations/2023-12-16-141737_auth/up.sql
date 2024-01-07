CREATE EXTENSION IF NOT EXISTS "uuid-ossp";


CREATE TABLE "user" (
  id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
  username varchar(64) NOT NULL,
  salt bytea NOT NULL,
  password_hash bytea NOT NULL
);
