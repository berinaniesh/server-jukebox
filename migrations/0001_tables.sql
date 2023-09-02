CREATE TABLE `songs` (
  `id` int NOT NULL AUTO_INCREMENT,
  `song` varchar(250) NOT NULL,
  PRIMARY KEY (`id`)
);

CREATE TABLE `dqueue` (
  `id` int NOT NULL AUTO_INCREMENT,
  `song` varchar(250) NOT NULL,
  PRIMARY KEY (`id`)
);

CREATE TABLE `pqueue` (
  `id` int NOT NULL AUTO_INCREMENT,
  `uuid` varchar(50) NOT NULL,
  `song` varchar(250) NOT NULL,
  PRIMARY KEY (`id`)
);

CREATE TABLE `log` (
  `id` int NOT NULL AUTO_INCREMENT,
  `song` varchar(250) NOT NULL,
  `ip` varchar(20) NOT NULL,
  PRIMARY KEY (`id`)
);

CREATE TABLE `deflist` (
  `id` int NOT NULL AUTO_INCREMENT,
  `song` varchar(250) NOT NULL,
  PRIMARY KEY (`id`)
);

CREATE TABLE `admin` (
  `id` int NOT NULL AUTO_INCREMENT,
  `value` varchar(100) DEFAULT NULL,
  PRIMARY KEY (`id`)
);

