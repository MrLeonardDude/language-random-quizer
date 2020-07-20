CREATE TABLE users(
    id int NOT NULL AUTO_INCREMENT,
    nome varchar(50) not null,
    login varchar(50) not null,
    psswd varchar(64) not null,
    date_inserted DATETIME DEFAULT CURRENT_TIMESTAMP,
    date_updated DATETIME ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY(id),
    UNIQUE(login)
) ENGINE=InnoDB DEFAULT CHARSET=utf8