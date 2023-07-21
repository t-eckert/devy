-- CreateEnum
CREATE TYPE "Status" AS ENUM ('NEW', 'QUEUED', 'UPLOADING', 'DONE', 'FAILED', 'REQUEUED');

-- CreateTable
CREATE TABLE "Upload" (
    "id" TEXT NOT NULL,
    "user" TEXT NOT NULL,
    "status" "Status" NOT NULL DEFAULT 'NEW',
    "repo" TEXT NOT NULL,

    CONSTRAINT "Upload_pkey" PRIMARY KEY ("id")
);
