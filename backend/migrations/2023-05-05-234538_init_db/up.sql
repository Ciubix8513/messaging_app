-- Your SQL goes here
CREATE TABLE Users (
    user_id INT NOT NULL AUTO_INCREMENT,
    username VARCHAR(16) NOT NULL UNIQUE,
    email VARCHAR(128) NOT NULL UNIQUE,
    password VARCHAR(1024) NOT NULL,
    PRIMARY KEY (user_id)
);

CREATE TABLE Group_Chats (
    chat_id INT NOT NULL AUTO_INCREMENT,
    chat_name VARCHAR(128) NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (chat_id)
);

CREATE TABLE Group_Chat_Members (
    chat_id INT NOT NULL,
    user_id INT NOT NULL,
    PRIMARY KEY (chat_id, user_id),
    FOREIGN KEY (chat_id) REFERENCES Group_Chats(chat_id),
    FOREIGN KEY (user_id) REFERENCES Users(user_id)
);

CREATE TABLE Messages (
    message_id INT NOT NULL AUTO_INCREMENT,
    chat_id INT NOT NULL,
    user_id INT NOT NULL,
    message_text TEXT NOT NULL,
    sent_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (message_id),
    FOREIGN KEY (chat_id) REFERENCES Group_Chats(chat_id),
    FOREIGN KEY (user_id) REFERENCES Users(user_id)
);

