use crate::linked_list::LinkedList::*;

enum LinkedList {
    Pair(i64, Box<LinkedList>),
    Nil
}

impl LinkedList {
    pub fn new() -> Self{
        Nil
    }

    pub fn prepend(self, v:i64) -> LinkedList{
        Pair(v, Box::new(self))
    }

    pub fn stringify(&self) -> String {
        match *self {
            Pair(head, ref tail) => {
                format!("{} {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            }
        }
    }

}


pub fn hello() {
    let mut ls = LinkedList::new();
    ls = ls.prepend(1);
    ls = ls.prepend(2);
    ls = ls.prepend(3);
    ls = ls.prepend(4);

    println!("{}", ls.stringify());
}
