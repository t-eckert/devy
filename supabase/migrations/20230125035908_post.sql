create table "public"."post" (
    "id" bigint generated by default as identity not null,
    "created_at" timestamp with time zone not null default now(),
    "updated_at" timestamp with time zone,
    "markdown" text not null default ''::text,
    "title" text not null default ''::text,
    "author" bigint not null,
    "slug" text not null default ''::text
);


alter table "public"."post" enable row level security;

CREATE UNIQUE INDEX post_pkey ON public.post USING btree (id);

alter table "public"."post" add constraint "post_pkey" PRIMARY KEY using index "post_pkey";

alter table "public"."post" add constraint "post_author_fkey" FOREIGN KEY (author) REFERENCES profile(id) not valid;

alter table "public"."post" validate constraint "post_author_fkey";

