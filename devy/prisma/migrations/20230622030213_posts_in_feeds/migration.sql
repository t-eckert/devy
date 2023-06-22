-- AlterTable
ALTER TABLE "Feed" ADD COLUMN     "lastCachedAt" TIMESTAMP(3);

-- CreateTable
CREATE TABLE "_FeedToPost" (
    "A" INTEGER NOT NULL,
    "B" INTEGER NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "_FeedToPost_AB_unique" ON "_FeedToPost"("A", "B");

-- CreateIndex
CREATE INDEX "_FeedToPost_B_index" ON "_FeedToPost"("B");

-- AddForeignKey
ALTER TABLE "_FeedToPost" ADD CONSTRAINT "_FeedToPost_A_fkey" FOREIGN KEY ("A") REFERENCES "Feed"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_FeedToPost" ADD CONSTRAINT "_FeedToPost_B_fkey" FOREIGN KEY ("B") REFERENCES "Post"("id") ON DELETE CASCADE ON UPDATE CASCADE;
