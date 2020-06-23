//RC; Reference Counter: placeholder for garbage collectors
//Rc is just like Box, but we can duplicate it, and its memory will only be freed when all the Rc's derived from it are dropped. 
//Unfortunately, this flexibility comes at a serious cost: we can only take a shared reference to its internals. 
//This means we can't ever really get data out of one of our lists, nor can we mutate them.

use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new()-> Self {
        List { head: None }
    }
    //appending: takes a list and element, returns a List
    //Rc in particular uses Clone as the way to increment the reference count. So rather than moving a Box to be in the sublist, we just clone the head of the old list.
     pub fn append(&self, elem:T) -> List<T> {
        List {
            head : Some (Rc::new(Node {
                                elem : elem ,  
                //Clone is implemented by almost every type, and provides a generic way to get "another one like this one" that is logically disjoint given only a shared reference.
                                next : self.head.clone(),
                            })
                        )
          }
    } 
    //tail is the logical inverse of this operation. It takes a list and returns the whole list with the first element removed.
    pub fn tail(&self) -> List<T> {
        List {
            //if we do .map(|node| node.next.clone()) won't compile, as map expects return of Y,
            //but we return option with clone() so we need to use and_then() here. 
            head :  self.head.as_ref().and_then(|node| node.next.clone()) 
        }
    }
    
    //return reference to first element with peek
    pub fn head(&self) -> Option<&T> {
        //as_ref returns Option<&T>
        self.head.as_ref().map(|node| &node.elem)
    }
}

//define iter struct
pub struct Iter<'a, T> {
    next : Option<&'a Node<T>>,
}

//implement iter constructor function on the defined struct
impl<T> List<T> {
    pub fn iter(&self)-> Iter<'_, T> {
        //turbofish says I am sure and I only care specific type of the result of option as result of map 
        Iter{ next: self.head.as_ref().map::<&'_ Node<T>, _>(|node| &node) } 
        // Iter { next: self.head.as_ref().map(|node| &**node) } 
    }
}

//implement next function of the Iter
/*
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}*/


impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    //that is the sig of the method in the doc
    fn next(&mut self) -> Option<Self::Item>{
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

// we can;t implement IntoIter and IterMut, because we only have shared access to this.


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.append(1).append(2).append(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);

    }
}


#[test]
fn iter() {
    let list = List::new().append(1).append(2).append(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
}





















