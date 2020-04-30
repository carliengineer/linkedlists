use std::mem;
pub struct List {
    head: Link,
}


type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List{
    pub fn new() -> Self{
        List { head: None}
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, None),
        });
    
        self.head = Some(new_node);
    }
/*
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(node)  => {
                self.head = node.next;
                Some(node.elem)
            }
       } //if there is a ; at the end, won't compile as with it it does not take match as return statement 
    }
*/

//match option { None => None, Some(x) => Some(y) } is such an incredibly common idiom that it was called map. 
//map takes a function to execute on the x in the Some(x) to produce the y in Some(y). 
//We could write a proper fn and pass it to map, but we'd much rather write what to do inline.The way to do this is with a closure. 
//Closures are anonymous functions with an extra super-power: they can refer to local variables outside the closure! 
//This makes them super useful for doing all sorts of conditional logic. 

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
        }
    }
}




//pub mod first;

mod tests {
    use super::List;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}

