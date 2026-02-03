-- Your SQL goes here
ALTER TABLE "users" ADD COLUMN "vault" TEXT NOT NULL;
ALTER TABLE "users" ADD COLUMN "iterations" INT4 NOT NULL;
ALTER TABLE "users" ADD COLUMN "vaultiv" TEXT NOT NULL;

