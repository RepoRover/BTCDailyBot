CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE chats (
  chat_id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
  telegram_chat_id varchar(255) NOT NULL,
)