use std::mem;
pub struct List<T> {
    head: Link<T>,
}


type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T, //i32,
    next: Link<T>,
}

//There are three common methods which can create iterators from a collection:
    //iter(), which iterates over &T.
    //iter_mut(), which iterates over &mut T.
    //into_iter(), which iterates over T.

//Creating an iterator of your own involves two steps: creating a struct to hold the iterator's state, and then implementing Iterator for that struct. 

//tuple structs are useful for trivial wrappers around other types
pub struct IntoIter<T> (List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }   
}

impl<T> Iterator for IntoIter<T> {
    type Item = T; 
    fn next(&mut self) -> Option<Self::Item> { // -> Option<T>
        self.0.pop()
    }
}

//The basic logic we want is to hold a pointer to the current node we want to yield next
//Because that node may not exist (the list is empty or we're otherwise done iterating), 
//we want that reference to be an Option. When we yield an element, we want to proceed to the current node's next node.
pub struct Iter<'a, T> {
    next : Option<&'a Node<T>>
}

impl<T> List<T> {
    //You may be thinking "wow that &** thing is really janky", and you're not wrong. 
    //Normally Rust is very good at doing this kind of conversion implicitly, through a process called 
    //deref coercion, where basically it can insert *'s throughout your code to make it type-check. 
    //It can do this because we have the borrow checker to ensure we never mess up pointers!
    pub fn iter<'a>(&'a self) -> Iter<'a, T>{
        Iter{next : self.head.as_ref().map(|node| &**node)}
        //alternatively, with turbofish ::<> The turbofish, ::<>, that lets us tell the compiler what we think the types of those generics should be
        //map is a generic function. pub fn map<U,F>(self, f: F) -> Option<U>
        //so turbofish in this case says map should return ::<&Node<T>> and I dont care/know about other type
        //this in turn lets compiler know it should apply deref coercion apploed to it, so we don't
        //have to do the **'s ourselves
            //self.next = node.head.as_ref().map::<&Node<T>, _>(|node| &node);
        //upon fixing the refenrecing with as_ref and &**node, we can finally apply lifetime elision
        //so this function needs no lifetimes, or to say there is a lifetime but it is not needed,
        //we can do, as of rust 2018, explicitly elided lifetime: '_ rather than 'a
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    //below we don't need life time a. it shadows the a defined above in impl<'a, T> 
    //self just takes the lifetime define dthere, no need to write to a function with only one
    //input that is self
    // important: for the next function, it establishes no constraint between the lifetime of input
    // and putput, so it can be written as folowing: 
    //fn next<'b>(&'b mut self) -> Option<&'a T >{ 
    //so we can call next over and over unconditionally, which is fine for shared references, like:
    /*
        list.push(1); list.push(2); list.push(3);
        let mut iter = list.iter();
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        let z = iter.next().unwrap();
    */
    //so iter is able to return references to many new variables, as it is shared reference.
    //howver this won't work itermut(),as mutable references can't coexist

    fn next(&mut self) -> Option<Self::Item>{
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
   
    //&mut is not Copy, so you can't give reference to self.next lie .as_ref() 
    //because if you cpoied &mut you would have 2 &muts to the same location in memory
    //insted we should properly take() the value:
    fn next<'b>(&'b mut self) -> Option<&'a mut T> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
    //why it works? 
    //We take the Option<&mut> so we have exclusive access to the mutable reference. No need to worry about someone looking at it again.
    //Rust understands that it's okto shard a mutable reference into the subfields of the pointed-to struct, 
    //because there's no way to "go back up", and they're definitely disjoint. 
}



impl<T> List<T>{
    pub fn new() -> Self{
        //we don't put <T> here, as it is inferred by the return type Self, which has <T> in its
        //definition
        List { head: None}
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            //memreplace is a common idiom, Option made it a method take()
            //next: mem::replace(&mut self.head, None),
            next: self.head.take(),
        });
    
        self.head = Some(new_node);
    }

//match option { None => None, Some(x) => Some(y) } is such an incredibly common idiom that it was called map. 
//map takes a function to execute on the x in the Some(x) to produce the y in Some(y). 
//We could write a proper fn and pass it to map, but we'd much rather write what to do inline.The way to do this is with a closure. 
//Closures are anonymous functions with an extra super-power: they can refer to local variables outside the closure! 
//This makes them super useful for doing all sorts of conditional logic. 

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
    
    //return the element at the head of the list
    pub fn peek(&self) -> Option<&T> {
       self.head.as_ref().map(|node| {
            &node.elem     
       })
    }

    pub fn peek_mut(&mut self) ->Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }        

}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take(); //mem::replace(&mut self.head, None);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take(); //mem::replace(&mut boxed_node.next, None);
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
    #[test]
    fn peek() {
         let mut list = List::new();
         assert_eq!(list.peek(), None);
         assert_eq!(list.peek_mut(), None);
         list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
       // following creates a pattern that will be matched against the argument to the closure; 
       // |&mut value| means "the argument is a mutable reference, but just copy the value it points to into value
       // as peek_mut returns an arg of &mut val as well.
       // list.peek_mut().map(|&mut value| {
       //      value = 42
       // });
        //in the following, it matches argument (&mut value) returned by peek_mut to value arg of
        //the closure such that, the type of the value will be &mut i32 and we can mutate it
        list.peek_mut().map(|value|{
            *value = 42
        });
        

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    } 

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }

}
