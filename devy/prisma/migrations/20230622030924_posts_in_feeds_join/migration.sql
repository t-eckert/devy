/*
  Warnings:

  - You are about to drop the `_FeedToPost` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropForeignKey
ALTER TABLE "_FeedToPost" DROP CONSTRAINT "_FeedToPost_A_fkey";

-- DropForeignKey
ALTER TABLE "_FeedToPost" DROP CONSTRAINT "_FeedToPost_B_fkey";

-- DropTable
DROP TABLE "_FeedToPost";

-- CreateTable
CREATE TABLE "PostInFeed" (
    "id" SERIAL NOT NULL,
    "feedId" INTEGER NOT NULL,
    "postId" INTEGER NOT NULL,

    CONSTRAINT "PostInFeed_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "PostInFeed" ADD CONSTRAINT "PostInFeed_feedId_fkey" FOREIGN KEY ("feedId") REFERENCES "Feed"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "PostInFeed" ADD CONSTRAINT "PostInFeed_postId_fkey" FOREIGN KEY ("postId") REFERENCES "Post"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
