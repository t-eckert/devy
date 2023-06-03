-- CreateTable
CREATE TABLE "Post" (
    "id" SERIAL NOT NULL,
    "title" TEXT NOT NULL,
    "markdown" TEXT,

    CONSTRAINT "Post_pkey" PRIMARY KEY ("id")
);
