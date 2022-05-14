# Priority_TODO

you can use priority queue TODO.
TODO items is stored to Sqlite DB.


# Usage
- add todo item
```
$ ./target/debug/priority_todo --method add
input todo text:
buy water
input priority:
6
--TODO LIST--
ID TODO PRIORITY
1, play the guiter, 4
2, buy water, 6
```
- show todo list sorted by priority
```
$ ./target/debug/priority_todo --method show
ID TODO PRIORITY
1, play the guiter, 4
2, buy water, 6
```

- delete todo item from todo list
```
$ ./target/debug/priority_todo --method complete
--TODO LIST--
ID TODO PRIORITY
1, play the guiter, 4
2, buy water, 6

input complete todo ID:
2
--TODO LIST--
ID TODO PRIORITY
1, play the guiter, 4
```

- reset todo list
```
$ ./target/debug/priority_todo --method reset

$ ./target/debug/priority_todo --method show
ID TODO PRIORITY
```
