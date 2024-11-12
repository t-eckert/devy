DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_catalog.pg_indexes
        WHERE tablename = 'blog' AND indexname = 'idx_blog_slug'
    ) THEN
        CREATE INDEX idx_blog_slug ON "blog" (slug);
    END IF;
END $$;
