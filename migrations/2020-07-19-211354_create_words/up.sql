create table words(
    id int not null auto_increment,
    nome varchar(50) not null,
    id_language int not null,
    PRIMARY KEY (id),
    FOREIGN KEY (id_language) REFERENCES languages(id)    
) ENGINE=InnoDB DEFAULT CHARSET=utf8