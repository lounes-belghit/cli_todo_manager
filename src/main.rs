mod jsonfn;
mod todoitem;
mod utilefn;
use text_io::read;
//use todoitem::Item;
//use jsonfn::*;
use todoitem::*;
//use utilefn::*;

fn main() {
    //let mut todolist: Vec<Item> = vec![];
    let mut todolist: Vec<Item> = jsonfn::read_json("list.json");
    //main menu :
    loop {
        print!(
            "\n1:add item\t2:show taseks\n3:show details about a task\t4:change status of task\n5:delete item\t0:exit\n-->"
        );
        let choise: i32 = read!();
        match choise {
            1 => utilefn::add_item(&mut todolist),
            2 => utilefn::show_items(&todolist, true),
            3 => utilefn::show_items(&todolist, false),
            4 => utilefn::change_status(&mut todolist),
            5 => utilefn::delete_item(&mut todolist),
            _ => break,
        }
        print!("\n");
        //utilefn::show_items(&todolist, true);
    }
    if jsonfn::write_json("list.json", &todolist) {
        println!("the json file was updated with sucess !!");
    } else {
        println!("there was a erore with updating the json file please chek if it is ok !!");
    }
}
