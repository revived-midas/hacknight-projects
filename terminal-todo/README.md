# Terminal Todo list
Your job is to create a Todo list that runs in the terminal.

When you build the project with `cargo run` it should print the current Todo list (which is probably empty on init).

The use should be able to use the Todo list in the following way:

## Adding todos

`add some kind of todo item`

Should add "some kind of todo item" as a todo item in the list, and automatically assign it an id.

After every command the tool should print out the current todo list, which should now look something like this:

```
Todo List:  
-------------------
[ ] 1 some kind of todo item
```

## Marking todos as done

`done 1`

Should mark the todo item with id 1 as done.

```
Todo List:  
-------------------
[✔] 1 some kind of todo item
```

## Removing todos

`remove 1`

Should remove the todo item with id 1.

```
Todo List:  
-------------------
```


# Additional ideas
If this wasn't challenging enough here are some other features to implement:
* Filtering todos to show only checked, or unchecked items
* Persist todo list to file or database
* Handling multiple todo lists `load list shopping`?
