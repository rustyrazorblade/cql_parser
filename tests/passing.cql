select field1,field2 from test
select field1, field2 from test
select * from tab where term > ? order by bacon
select * from tab where term > ? order by bacon desc
select * from tab where term > ?
select * from tab where term > ? and x = ?
select * from test
select * from blah LIMIT 10
select * from blah WHERE a = ? and b = ? LIMIT 1
SELECT * FROM test WHERE pk=0 AND t=0;
SELECT name, occupation FROM users WHERE userid IN (199, 200, 207);
SELECT name AS user_name, occupation AS user_occupation FROM users;
SELECT time, value FROM events WHERE event_type = 'myEvent' AND time > '2011-02-03' AND time <= '2012-01-01'
SELECT COUNT(*) FROM users;
SELECT COUNT(*) AS user_count FROM users;
SELECT entry_title, content FROM posts WHERE userid='john doe' AND blog_title='John''s Blog' AND posted_at >= '2012-01-01' AND posted_at < '2012-01-31'
SELECT entry_title, content FROM posts WHERE userid='john doe' AND posted_at >= '2012-01-01' AND posted_at < '2012-01-31'
SELECT * FROM posts WHERE token(userid) > token('tom') AND token(userid) < token('bob')
SELECT * FROM posts WHERE userid='john doe' AND (blog_title, posted_at) > ('John''s Blog', '2012-01-01')
SELECT * FROM posts WHERE userid='john doe' AND blog_title > 'John''s Blog' AND posted_at > '2012-01-01'
SELECT * FROM posts WHERE userid='john doe' AND (blog_title, posted_at) IN (('John''s Blog', '2012-01-01), ('Extreme Chess', '2014-06-01'))
SELECT firstname, lastname FROM users WHERE birth_year = 1981 AND country = 'FR';
SELECT firstname, lastname FROM users WHERE birth_year = 1981 AND country = 'FR' ALLOW FILTERING;
SELECT * FROM myTable WHERE t > maxTimeuuid('2013-01-01 00:05+0000') AND t < minTimeuuid('2013-02-02 10:00+0000')

// inserts
insert into users (name, age) values (?, ?) using timestamp 60
insert into users (name, age) values (?, ?) using ttl 60
insert into ts (id, t, val) values (1, 1, 1)
insert into blah (name, value) values (?, ?)
insert into ts (id, t, val) values (-1, -1, -1);
INSERT INTO users (id, given, surname, favs) VALUES ('jsmith', 'John', 'Smith', { 'fruit' : 'apple', 'band' : 'Beatles' })
INSERT INTO images (name, owner, date, tags) VALUES ('cat.jpg', 'jsmith', 'now', { 'kitten', 'cat', 'pet' });
INSERT INTO plays (id, game, players, scores) VALUES ('123-afde', 'quake', 3, [17, 4, 2]);
INSERT INTO test(pk, t, v, s) VALUES (0, 0, 'val0', 'static0');
INSERT INTO test(pk, t, v, s) VALUES (0, 1, 'val1', 'static1');
INSERT INTO NerdMovies (movie, director, main_actor, year) VALUES ('Serenity', 'Joss Whedon', 'Nathan Fillion', 2005) USING TTL 86400;
INSERT INTO users (userid, password, name) VALUES ('user2', 'ch@ngem3b', 'second user');
INSERT INTO users (userid, password) VALUES ('user4', 'ch@ngem3c');

// LWT
insert into users (name, age) values (?, ?) if not exists

// UDT
INSERT INTO user_profiles(login, first_name, last_name, email, addresses) VALUES ('tsmith', 'Tom', 'Smith', 'tsmith@gmail.com',{ 'home': { street: '1021 West 4th St. #202',city: 'San Fransisco', zip: 94110 }});


// updates
update blah using ttl 60 set bal = ? where k = ?
update men set bal = ? where k = ? if bal = ?
update men set bal = ? where k = ? and v = ?
update men set bal = ? where k = ?
UPDATE users USING TTL 10 SET favs['color'] = 'green' WHERE id = 'jsmith'
UPDATE users SET favs['author'] = 'Ed Poe' WHERE id = 'jsmith'
UPDATE users SET favs = favs +  { 'movie' : 'Cassablanca' } WHERE id = 'jsmith'
UPDATE users USING TTL 10 SET favs['color'] = 'green' WHERE id = 'jsmith'
UPDATE images SET tags = tags + { 'cute', 'cuddly' } WHERE name = 'cat.jpg';
UPDATE images SET tags = tags - { 'lame' } WHERE name = 'cat.jpg';
UPDATE plays SET players = 5, scores = scores + [ 14, 21 ] WHERE id = '123-afde';
UPDATE plays SET players = 5, scores = [ 12 ] + scores WHERE id = '123-afde';
UPDATE plays SET scores[1] = 7 WHERE id = '123-afde';
UPDATE plays SET scores = scores - [ 12, 21 ] WHERE id = '123-afde';
UPDATE NerdMovies USING TTL 400 SET director = 'Joss Whedon', main_actor = 'Nathan Fillion', year = 2005 WHERE movie = 'Serenity';
UPDATE UserActions SET total = total + 2 WHERE user = B70DE1D0-9908-4AE3-BE34-5573E5B09F14 AND action = 'click';
UPDATE users SET password = 'ps22dhds' WHERE userid = 'user3';


// counters
update whatever set k = k + 1 where bacon = ?

// deletes
delete from blah where x = ?
delete from blah where x = 1
DELETE favs['author'] FROM users WHERE id = 'jsmith'
DELETE scores[1] FROM plays WHERE id = '123-afde';
DELETE FROM NerdMovies USING TIMESTAMP 1240003134 WHERE movie = 'Serenity';
DELETE name FROM users WHERE userid = 'user1';

DELETE phone FROM Users WHERE userid IN (C73DE1D3-AF08-40F3-B124-3FF3E5109F22, B70DE1D0-9908-4AE3-BE34-5573E5B09F14);
