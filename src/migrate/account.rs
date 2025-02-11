#[rustfmt::skip]
pub(super) const ACCOUNT_VERSIONS: [&str; 13] = [
  "CREATE TABLE IF NOT EXISTS accounts(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    gid TEXT NOT NULL,
    indx INTEGER NOT NULL,
    lang INTEGER NOT NULL,
    pass TEXT NOT NULL,
    name TEXT NOT NULL,
    lock TEXT NOT NULL,
    secret TEXT NOT NULL,
    encrypt TEXT NOT NULL,
    mnemonic TEXT NOT NULL,
    avatar TEXT NOT NULL,
    wallet TEXT NOT NULL,
    pub_height INTEGER NOT NULL,
    own_height INTEGER NOT NULL,
    event TEXT NOT NULL,
    datetime INTEGER NOT NULL);",
  "CREATE TABLE IF NOT EXISTS migrates(
    db_name TEXT NOT NULL,
    version INTEGER NOT NULL);",
  "INSERT INTO migrates (db_name, version) values ('account.db', 0)",
  "INSERT INTO migrates (db_name, version) values ('consensus.db', 0)",
  "INSERT INTO migrates (db_name, version) values ('session.db', 0)",
  "INSERT INTO migrates (db_name, version) values ('file.db', 0)",
  "INSERT INTO migrates (db_name, version) values ('jarvis.db', 0)",
  "INSERT INTO migrates (db_name, version) values ('group.db', 0)",
  "INSERT INTO migrates (db_name, version) values ('chat.db', 0)",
  "INSERT INTO migrates (db_name, version) values ('domain.db', 0)",
  "INSERT INTO migrates (db_name, version) values ('wallet.db', 0)",
  "INSERT INTO migrates (db_name, version) values ('cloud.db', 0)",
  "INSERT INTO migrates (db_name, version) values ('dao.db', 0)",
];
