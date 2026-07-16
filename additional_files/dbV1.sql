CREATE TABLE IF NOT EXISTS "Task" (
	"id" INTEGER NOT NULL UNIQUE,
	"Name" TEXT,
	"descreption" TEXT,
	"status" TEXT,
	PRIMARY KEY("id")
);
