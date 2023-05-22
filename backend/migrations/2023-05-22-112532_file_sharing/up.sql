-- Your SQL goes here
CREATE TABLE files (
  id INT PRIMARY KEY AUTO_INCREMENT,
  filename VARCHAR(255),
  path VARCHAR(255),
  message_id INT,
  FOREIGN KEY (message_id) REFERENCES messages(message_id)
);
