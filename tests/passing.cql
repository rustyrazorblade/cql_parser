select field1,field2 from test
select field1, field2 from test
select * from tab where term > ? order by bacon
select * from tab where term > ? order by bacon desc
select * from tab where term > ?
select * from tab where term > ? and x = ?
select * from test
select * from blah LIMIT 10
select * from blah WHERE a = ? and b = ? LIMIT 1

update blah using ttl 60 set bal = ? where k = ?
update men set bal = ? where k = ? if bal = ?
update men set bal = ? where k = ? and v = ?
update men set bal = ? where k = ?

insert into users (name, age) values (?, ?) using timestamp 60
insert into users (name, age) values (?, ?) using ttl 60
insert into users (name, age) values (?, ?) if not exists
insert into blah (name, value) values (?, ?)

delete from blah where x = ?
