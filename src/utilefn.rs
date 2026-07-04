use text_io::read;
//use todoitem::*;

use crate::todoitem::Item; //crate use to declare the pakage

pub fn add_item(list: &mut Vec<Item>) {
    let id: i32 = list.len() as i32;
    print!("\n enter the name of the new item-->");
    let name: String = read!(); //use read instate of the stdin  it's more ofechen
    print!("\nenter the discreption of your item-->");
    let descreption: String = read!();
    list.push(Item::new(id, name, descreption));
}
pub fn show_items(list: &Vec<Item>, is_ditailsted: bool) {
    if list.len() == 0 {
        return;
    }
    if is_ditailsted {
        print!("\n ----------------------------\n");
        for i in list {
            i.display_litel();
            print!("\n")
        }
    } else {
        print!("what is the id of item-->");
        let id: i32 = read!();
        //let item: Item = list[id];
        //item.display_item();

        list[id as usize].display_item();
    }
}
pub fn change_status(list: &mut Vec<Item>) {
    print!("enter the item that you want to change status -->");
    let id: i32 = read!();
    list[id as usize].change_status();
}
pub fn delete_item(list: &mut Vec<Item>) {
    print!("enter the id for the item that you want to delete -->");
    let id: i32 = read!();
    list.remove(id as usize);
}
