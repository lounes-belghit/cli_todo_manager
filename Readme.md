# the project is a todo list with json save file the basic functions are:
    --show the todo list [*]
    --show the status of the todo items [*]
    --add item [*]
    --change a status of item [*]
    --delete item [*]
    --show more info about item [*]
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
# for the data base part we will use a sqlite:
    **as first idea we will start with one table then w may make other tables for users of colective work **
    --create the archetecture of db (relations ,tables,workflow ) we will use drawdb to make it easier []
    -- work on add function []
  ## notes:
      --we will use the rusqlite layberry for the single thread process 
      --after that for the http server we need to use a multi thread layberry to handel the parallel requisets 
  ## sqlite fn that we will implement for now :
        -- create db fn[*]
        -- add task fn  []
        -- delete task fn[] 
        -- update status []
        -- get task []
        -- get all taskes []
        -- delete all tasks [] 
        -- search for task []
