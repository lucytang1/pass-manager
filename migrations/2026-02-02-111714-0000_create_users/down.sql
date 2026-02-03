-- This file should undo anything in `up.sql`
ALTER TABLE "users" DROP COLUMN "vault";
ALTER TABLE "users" DROP COLUMN "iterations";
ALTER TABLE "users" DROP COLUMN "vaultiv";

