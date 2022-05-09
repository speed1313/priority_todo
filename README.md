# Priority_TODO

you can use priority queue TODO.
TODO items is stored to a local file.
The storage file's format is like below
```
watch Hikakine TV, 2
study Rust, 4
buy water, 5
```
first column is TODO item, second column is priority level.


# Usage
- add TODO item
```
$ Priority_TODO add 
please add TODO
> buy watar
please specify the priority level of the TODO from 10 to 1
> 5
accept TODO now!
Latest TODO list is shown below
1. buy water
2. study Rust
3. watch Hikakine TV
```

- complete TODO item
```
$ Priority_TODO complete
please select the number of TODO item from TODO list below
1. buy water
2. study Rust
3. watch Hikakine TV
> 2
accept now!
Latest TODO list is shown below
1. buy water
2. watch Hikakine TV
```

- show TODO item
```
$ Priority_TODO show
1. buy water
2. watch Hikakine TV
```


