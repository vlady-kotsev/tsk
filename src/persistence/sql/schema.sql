-- SQLite creates the database file itself when a connection is opened
-- (see Connection::open in persister.rs), so there is no CREATE DATABASE
-- statement here. This script just sets up the schema on top of it.

PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS boards (
    id    TEXT PRIMARY KEY,
    title TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tasks (
    id       TEXT PRIMARY KEY,
    content  TEXT NOT NULL,
    board_id TEXT NOT NULL,
    FOREIGN KEY (board_id) REFERENCES boards (id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_tasks_board_id ON tasks (board_id);
