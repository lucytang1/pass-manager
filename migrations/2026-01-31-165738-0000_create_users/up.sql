-- Your SQL goes here
CREATE TABLE "users"(
	"id" UUID NOT NULL PRIMARY KEY,
	"email" TEXT NOT NULL,
	"user_key" TEXT NOT NULL,
	"salt" TEXT NOT NULL
);

