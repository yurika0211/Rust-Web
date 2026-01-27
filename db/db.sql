drop table if exists course;

create table course(
    id serial primary key,
    teacher_id INT not null,   -- 修正：从 nill 改为 null
    name varchar(140) not null,
    time TIMESTAMP default now()
);

-- 修正：时间字符串改用单引号 ' '
insert into course
    (id, teacher_id, name, time)
values(1, 1, 'First Course', '2018-01-01 00:00:00');

insert into course
    (id, teacher_id, name, time)
values(2, 1, 'Second Course', '2018-01-02 00:00:00');