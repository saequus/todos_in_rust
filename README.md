# ToDo(s) in Rust

Manage ToDo items from cli. 
Data is preserved within `state.json`.  

### Installation
~~~
$ cargo install
~~~

### Running

~~~
$ cargo run create washing
> washing is being created

$ cargo run create washing
> Item washing
> Status "PENDING"

$ cargo run create washing
> washing is set to done

$ cargo run delete washing
> washing is being delete
~~~

