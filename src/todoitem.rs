use std::fmt;

//use crate::todoitem::Status::{Don, WorkingOn};
#[derive(Debug, Clone, Copy, PartialEq)]
//use text_io::read;
enum Status {
    Waiting,
    WorkingOn,
    Don,
}
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::WorkingOn => write!(f, "Working On"),
            Status::Waiting => write!(f, "Waiting"),
            Status::Don => write!(f, "Done"),
        }
    }
}
//#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Item {
    id: i32,
    name: String,
    descreption: String,
    status: Status,
}
// if we implement a struct to a class( we dont call it a class ) if we put ->self that will change in the obgect
// the new fn is the public cunstructure
impl Item {
    pub fn new(id: i32, name: String, descreption: String) -> Self {
        Self {
            id,
            name,
            descreption,
            status: Status::Waiting,
        }
    }

    pub fn display_item(&self) {
        print!(
            "id:{}\nname:{}\ndescreption:{}\nstatus:{}\n",
            self.id, self.name, self.descreption, self.status
        );
    }
    pub fn display_litel(&self) {
        print!("|id:{}|name:{}|status:{}|", self.id, self.name, self.status);
    }
    pub fn change_status(&mut self) {
        match self.status {
            Status::Waiting => self.status = Status::WorkingOn,
            Status::WorkingOn => self.status = Status::Don,
            _ => self.status = Status::Don,
        }
    }
}
