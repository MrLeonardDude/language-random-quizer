create table translations(
    id int not null auto_increment,
    language_from int not null,
    language_to int not null,
    PRIMARY KEY(id),
    FOREIGN KEY (language_from) REFERENCES words(id),
    FOREIGN KEY (language_from) REFERENCES words(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 