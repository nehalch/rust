create table person (
    person_id int auto_increment,
    person_name varchar(100) null,
    constraint person_pk primary key (person_id)
);
INSERT INTO `db1`.`person` (`person_id`, `person_name`)
VALUES ('1', 'Gary');
INSERT INTO `db1`.`person` (`person_id`, `person_name`)
VALUES ('2', 'Steve');