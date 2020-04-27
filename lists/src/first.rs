use std::mem;

//enums are types that can have different values which may have different types
// pub says we want people outside this module to be able to use List
/*pub enum List {
    Empty,
    Elem(i32, Box<List>),
}
*/
/*
A pointer type for heap allocation.
Box<T>, casually referred to as a 'box', provides the simplest form of heap allocation in Rust. 
Boxes provide ownership for this allocation, and drop their contents when they go out of scope. 
*/


// public list thta can be used
pub struct List {
    head: Link,
}

//  
enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

//To associate actual code with a type, we use impl blocks:
//Self is an alias for "that type I wrote at the top next to impl". Great for not repeating yourself!
//The last expression of a function is implicitly returned. 
//This makes simple functions a little neater. You can still use return to return early like other C-like languages.

impl List{
    pub fn new() -> Self{
        List { head: Link::Empty}
    }

    //Non-static (normal) methods. 
    //Methods are a special case of function in Rust because of the self argument, which doesn't have a declared type.
    /*Self can taake 3 primary forms
        self - Value --> A value represents true ownership. You can do whatever you want with a value: move it, destroy it, mutate it, or loan it out via a reference. When you pass something by value, it's moved to the new location.  The new location now owns the value, and the old location can no longer access it.
        &mut self - mutable reference --> A mutable reference represents temporary exclusive access to a value that you don't own. You're allowed to do absolutely anything you want to a value you have a mutable reference to as long you leave it in a valid state when you're done This means you can actually completely overwrite the value. A really useful special case of this is swapping a value out for another.
        &self - shared reference -->  A shared reference represents temporary shared access to a value that you don't own. Because you have shared access, you're generally not allowed to mutate anything. Think of & as putting the value out on display in a museum. & is great for methods that only want to observe self.
    
    */
    
    //entire old list needs to go as next element. we can't directly reference self.head, as we &mut borrow
    //mem::replace maneuver. This incredibly useful function lets us steal a value out of a borrow by replacing it with another value.
    //Here we replace self.head temporarily with Link::Empty before replacing it with the new head of the list. 
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
    
        self.head = Link::More(new_node);
    }

    //unimplemented!() is a macro (! indicates a macro) that panics the program when we get to it (~crashes it in a controlled manner)
    //what if the list is empty? option is for that
    pub fn pop(&mut self) -> Option<i32> {
        let result;
        //with &self.head we can't change its value. so we need it by value
        //so we can do that with mem::replace
        match mem::replace(&mut self.head, Link::Empty) {
       // match &self.head {
            Link::Empty => {
                result = None;
            }
            Link::More(node) => {
                result = Some(node.elem);
                self.head = node.next;
            }
        };
        //unimplemented!()
        result
    }


}
//Drop if you contain types that implement Drop, and all you'd want to do is call their destructors. 
//In the case of List, all it would want to do is drop its head, which in turn would maybe try to drop a Box<Node>. 
//All that's handled for us automatically... with one hitch. The automatic handling is going to be bad.
// For list -> A -> B -> C gets dropped, it will try to drop A, which will try to drop B, which will try to drop C.
//This is recursive code, and recursive code can blow the stack. it is not a tail-recursive.
// Because We can't drop the contents of the Box after deallocating, so there's no way to drop in a tail-recursive manner

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
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
