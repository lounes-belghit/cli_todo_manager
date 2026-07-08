# the project is a todo list with json save file the basic functions are
   --show the todo list[*]
   --show the status of the todo items[*]
   --add item[*]
   --change a status of item[*]
   --delete item[*]
   --show more info about item[*]
# the item is a struct :
    --id
    --name
    --descreption
    --status
# the status is an enum of 3 states :
    --waiting
    --working on
    --don
# the librerys needed for are
    --std:for input and output  we had use to text_io for the read function it's aoutumated instate of using the defult one with parecer use  cargo add text_io to add it 
    --we use serde to manupulate the jison files so you can add them by using this command on your terminal: cargo add serde --features derive && cargo add serde_json
# notes:
    --as first attemd we will start with simple list [*]
    --json file [*]
    --data base[]
    --make it as api for a web app[]
