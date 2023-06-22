/*
  Warnings:

  - Made the column `name` on table `Blog` required. This step will fail if there are existing NULL values in that column.
  - Made the column `title` on table `Post` required. This step will fail if there are existing NULL values in that column.
  - Made the column `markdown` on table `Post` required. This step will fail if there are existing NULL values in that column.

*/
-- CreateEnum
CREATE TYPE "Order" AS ENUM ('NONE', 'ASC', 'DESC');

-- AlterTable
ALTER TABLE "Blog" ALTER COLUMN "name" SET NOT NULL;

-- AlterTable
ALTER TABLE "Post" ALTER COLUMN "title" SET NOT NULL,
ALTER COLUMN "markdown" SET NOT NULL;

-- CreateTable
CREATE TABLE "Feed" (
    "id" SERIAL NOT NULL,
    "slug" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "publishOrder" "Order" NOT NULL DEFAULT 'NONE',
    "filterTags" BOOLEAN NOT NULL DEFAULT false,

    CONSTRAINT "Feed_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "_FeedToTag" (
    "A" INTEGER NOT NULL,
    "B" INTEGER NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "Feed_slug_key" ON "Feed"("slug");

-- CreateIndex
CREATE UNIQUE INDEX "_FeedToTag_AB_unique" ON "_FeedToTag"("A", "B");

-- CreateIndex
CREATE INDEX "_FeedToTag_B_index" ON "_FeedToTag"("B");

-- AddForeignKey
ALTER TABLE "_FeedToTag" ADD CONSTRAINT "_FeedToTag_A_fkey" FOREIGN KEY ("A") REFERENCES "Feed"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_FeedToTag" ADD CONSTRAINT "_FeedToTag_B_fkey" FOREIGN KEY ("B") REFERENCES "Tag"("id") ON DELETE CASCADE ON UPDATE CASCADE;
