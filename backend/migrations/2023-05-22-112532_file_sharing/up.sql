-- Your SQL goes here
CREATE TABLE files (
  id INT PRIMARY KEY AUTO_INCREMENT,
  filename VARCHAR(255) NOT NULL,
  path VARCHAR(255) NOT NULL,
  message_id INT NOT NULL,
  FOREIGN KEY (message_id) REFERENCES messages(message_id)
);
ALTER TABLE group_chats
MODIFY COLUMN `key` VARCHAR(256) NOT NULL;
