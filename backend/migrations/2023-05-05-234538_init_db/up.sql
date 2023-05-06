-- Your SQL goes here
CREATE TABLE users (
    user_id INT NOT NULL AUTO_INCREMENT,
    username VARCHAR(16) NOT NULL UNIQUE,
    email VARCHAR(128) NOT NULL UNIQUE,
    password VARCHAR(1024) NOT NULL,
    PRIMARY KEY (user_id)
);

CREATE TABLE group_chats (
    chat_id INT NOT NULL AUTO_INCREMENT,
    chat_name VARCHAR(128) NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (chat_id)
);

CREATE TABLE group_chat_members (
    chat_id INT NOT NULL,
    user_id INT NOT NULL,
    PRIMARY KEY (chat_id, user_id),
    FOREIGN KEY (chat_id) REFERENCES group_chats(chat_id),
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);

CREATE TABLE messages (
    message_id INT NOT NULL AUTO_INCREMENT,
    chat_id INT NOT NULL,
    user_id INT NOT NULL,
    message_text TEXT NOT NULL,
    sent_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (message_id),
    FOREIGN KEY (chat_id) REFERENCES group_chats(chat_id),
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);

