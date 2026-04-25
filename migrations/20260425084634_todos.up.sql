-- Add up migration script here
create table if not exists todos (
    "id" uuid primary key,
    "title" text not null,
    "isCompleted" boolean not null default false,
    "createdAt" timestamptz not null default now(),
    "updatedAt" timestamptz not null default now()
);

create index if not exists "todos_isCompleted" on todos ("isCompleted");
create index if not exists "todos_createdAt" on todos ("createdAt");
create index if not exists "todos_updatedAt" on todos ("updatedAt");
