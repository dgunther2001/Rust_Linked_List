
use std::fmt::{self, Debug};

pub struct LinkedList<T> 
where T: Debug {
    head: Link<T>,
    length: u32,
}

// CAN HAVE SPECIFIC IMPLEMENTTIONS AS WELL!!!
//impl LinkedList<u32> {}
//impl LinkedList<i32> {}
#[derive(Debug, Clone)] // derives a debug and clone trait
struct ListNode <T>
where T: Debug {
    el: T, // some element of type generic T
    next: Link<T>,
    //NonEmpty(u32 /* 32 bits */, &'a ListNode<'a>), //Box<ListNode> /* 64 bits */),
}

impl <T> ListNode<T> 
where T: Debug {

    /*
        Objective => take a list node, and get out a reference to the one it points to in next
        Input(s) => (1 -> &self) a reference to a ListNode {IMPLICIT}
        Output => None
     */
    fn next(&self) -> &Link<T> { // implementation of a next function
        &self.next // simply returns a reference to the next ListNode, which can be None
    }

    fn iter(&self) {
        todo!(); // MAKE AN ITERATOR!!!
    }
}

type Link<T> = Option<Box<ListNode<T>>>; // EQUIVALENT TO WHAT IS BELOW...
/*
enum Link {
    Empty,
    NonEmpty(Box<ListNode>),
}
*/

impl <T> LinkedList<T> 
where T: Debug + Clone {

    /*
        Objective => initialize the LinkedList (constructor)
        Input(s) => None
        Output => an empty LinkedList
     */
    pub fn init_list() -> Self { // or pub fn empty() -> LinkedList<T>
        // "constructor"
        Self { head: None, length: 0 } // or LinkedList { head: None }
    }

    /*
        Objective => append to the front of a LinkedList and push everything down 1 index
        Input(s) => (1 -> &mut self) a mutable reference to a LinkedList {IMPLICIT}; (2: element -> T) a literal of type T
        Output => None 
     */

    pub fn append_front(&mut self, element: T) {
        // AQUIRE FULL OWNERSHIP OF THE OLD HEAD
        //let old_head = std::mem::replace(&mut self.head, None); // SETS THE OLD HEAD TO NONE BUT STORES IT IN A MEMORY SAFE WAY
        let old_head = self.head.take(); // EQUIVALENT OT THE ABOVE LINE!!!
        let new_head = Box::new(ListNode { //creates a new node
            el: element,
            next: old_head
        });
        self.head = Some(new_head); // sets the head to the new node
        self.length = self.length + 1; // increments the length
    }

    /*
        Objective => append to the end of a linked list
        Input(s) => (1 -> &mut self)  a mutable reference to a LinkedList {IMPLICIT}; (2 -> element: T) a literal of type T
        Output => None
     */
    pub fn append_back(&mut self, element: T) {
        let new_node = Box::new(ListNode {  // intiializes the new node
            el: element,
            next: None,
        });

        match &self.head { // checks whether the head is None
            Some(n) => {
                let mut current_node = self.head.as_mut(); // sets a mutable reference that we iterate over
                while let Some(node) = current_node { // gets whats inside the option (when head is not None)
                    if node.next.is_none() { // if the next node is None, we are at the end of the list
                        node.next = Some(new_node); // set the value of next to the new node
                        break; // break out of the while loop
                    } else { // otherwise, iterate...
                        current_node = node.next.as_mut(); // get next as a mutavle references
                    }
                }
            },
            None => { // if there is no head
                self.head = Some(new_node); // set the head to the new_node
            }
        } 
        self.length += 1; // increment the length...

    }

    /*
        Objective => insert at a specified index of the LinkedList
        Input(s) => (1 -> &mut self)  a mutable reference to a LinkedList {IMPLICIT}; (2 -> element: T) a literal of type T; (3 -> index: u32) the index we want to isnert at in the list
        Output => None
     */
    pub fn insert_at_index(&mut self, element: T, index: u32) {
        match index { // pattern matches based on the input index
            0 => self.append_front(element), // if the index is just 0, we just call the append_front function on self as that behavior is already implemented
            _ => { // otherise...
                if index >= 1 && index < self.length { // if the index is non-zero and within the range of the list...
                    let mut current_node = &mut self.head; // set a mutable Option current_node as a mutable reference t the head of the list
                    for _ in 0..index-1 { // iterate over the specified range in the if statement (0 to the input index)
                        if let Some(n) = current_node { // so long as the next node is not None (should never be based on how everything is defined) create a variable called n which is a mutable ref to current node
                            current_node = &mut n.next; // set current_nude to a mutable reference to the value store in n.next
                        } 
                    }
                    if let Some(n) = current_node { // if current_node is not None (should never be in this scope), declare a variable n equalt to current node, which is a mutable reference to the spot before where we want to insert
                        let new_node = Box::new(ListNode { // we declare a new ListNode on the heap
                            el: element,
                            next: n.next.take(), // sets current_node.next (n) to None, and stores the value that was stored there into the next field of the new node
                        });
                        n.next = Some(new_node); // sets the prior node's next field to the new node, and thus the list has been fully restored with the new node inserted at the proper location
                        self.length += 1; // increment the length of the list
                    }
 
                } else { // if the index specified is greater than the max possible index (LinkedList.length - 1)
                    self.append_back(element) // call the append_back method on self, as it has already been defined
                }
            }

        
        }


    }

    /*
        Objective => to delete the head of the list, and set the head to the next node
        Input(s) => (1 -> &mut self)  a mutable reference to a LinkedList {IMPLICIT}
        Output => None if the head was None, or the value of the element T stored at head
     */
    pub fn delete_front(&mut self) -> Option<T> {
        self.head.take()/* take() TAKES OWNERSHIP of the head of the list, and sets the head of the list itself to None */.map(|n | /* .map(...{ essentially does what it says if n, which now owns what was in head is is not None */{ 
            self.head = n.next; // sets the head to whatever the next node is
            self.length = self.length - 1; // decrements the length
            n.el // returns the value stored at the head, that was deleted
        })
    }

    /*
        Objective => to delete the last element in the list
        Input(s) => (1 -> &mut self)  a mutable reference to a LinkedList {IMPLICIT}
        Output => None if the head was None, or the value of the element T stored at the end of the list
     */
    pub fn delete_back(&mut self) -> Option<T> {
        let mut node = &mut self.head; // we create a mutable variable node, which is initially stored as a mutable reference to the head of the LinkedList
            while let Some(n ) = node.take() { // node is not None, take its value and store it in n, and set the value where it was taken from to None
                if n.next.is_some() { // if the next value is not None (IE -> NOT AT THE END OF THE LIST...)
                    let n = node.get_or_insert(n); // as node is set to none, it puts n back into node, and sets a new n to be a mutable reference to it
                    node = &mut n.next; // increments node to a mutable reference of the next location
                } else { // if the next node is None, then don't insert n back into the list and...
                    self.length -= 1; // decrement length
                    return Some(n.el); // and return an Option of the type T, if there is something there (should always be the case in this scope)
                } 
            } None // if the lsit is empty, return None
    }

    /*
        Objective => delete at a specified index
        Input(s) => (1 -> &mut self)  a mutable reference to a LinkedList {IMPLICIT}; (2 -> index: u32) the desired index to delete at
        Output => an Option of type T (thus it is allowed to be None, if the LinkedList is empty)
     */
    pub fn delete_at_index(&mut self, index: u32) -> Option<T> {
        if index == 0 {
            return self.delete_front(); // calls the predefined delete_front() method if the index is 0
        }
        if index >= self.length - 1 {
            return self.delete_back(); // calls the delete back index if we are at the end of the list, or specifiy an index out of bounds (BEWARE)...
        }
    
        let mut current_node = &mut self.head; // sets a mutable variable called current_node to be a mutable reference to the head of the list
        let mut current_index = 0; // sets the current index to 0, allowing us to pick the right thing to delete

        while let Some(node) = current_node { // node is the node prior to the one we intend to delete
            if current_index == index - 1 { // if we are at the node before the one we want to delete...
                if let Some(mut node_to_delete) = node.next.take() { // sets a mutable variable node_to_delete equal to the prior node.next by setting node.next = None through the Take() method
                    // node_to_delete STORES THE NODE WE INTEND TO DELETE
                    node.next = node_to_delete.next.take(); // set the node prior to node_to_delete.next equal to the node_to_delete.next, thereby transferring ownership to node.next and restoring the list
                    self.length -= 1; // decrement the length
                    return Some(node_to_delete.el); // returns the element (T) stored within the deleted node, and then it goes out of scope and is dropped
                }
            }
            // iteration over the list
            current_node = &mut node.next; // current node becomes a mutable reference to the node that node.next "points to"
            current_index += 1; // increments the current_index variable
        } None
    }

    /*
        Objective => get a reference to the value stored at the head of the list
        Input(s) => (1 -> &self)  a reference to a LinkedList {IMPLICIT}
        Output => None if the head is None, or a reference to the value stored at head
     */
    pub fn get_head(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.el) // returns the value (el) at head as a reference, if head is Some(...), otherwise just returns None
    }

    /*
        Objective => get a mutable reference to the value stored at the head of the list
        Input(s) => (1 -> &self)  a reference to a LinkedList {IMPLICIT}
        Output => None if the head is None, or a mutable reference to the value stored at head
     */
    pub fn get_head_as_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|mut n| &mut n.el) // if the mutable reference to self.head is None, return None, otherwise return a mutable reference to to the value T stored at the head of the list
    }

    /*
        Objective => get an Option, which returns a reference to to the ListNode at head, so long as head is not none
        Input(s) => (1 -> &self)  a reference to a LinkedList {IMPLICIT}
        Ouptut => None if the head is None, or a reference to the ListNode at head
     */
    fn get_head_ref(&self) -> Option<&ListNode<T>> {
        if let Some(head) = &self.head { // if the head is Some(...), we create a variable called head that stores a reference to the head of the list
            return Some(head); // return the reference to self.head (head) as Some(...)
        } None // if the head is None, return None
    }

    /*
        Objective => get a reference to the value stored at the tail of the list
        Input(s) => (1 -> &self)  a reference to a LinkedList {IMPLICIT}
        Output => None if the tail is None, or a reference to the value stored at tail
     */
    pub fn get_tail_ref(&self) -> Option<&T> {
        match self.get_length() { // matches the length of the list
            0 => None, // if its empty, there is no tail
            _ => self.get_at_index(self.get_length() - 1), // if it's non empty, get a reference to an element at the last index
        }
    } 

    /*
        Objective => get a mutable reference to the value stored at the tail of the list
        Input(s) => (1 -> &self)  a reference to a LinkedList {IMPLICIT}
        Output => None if the tail is None, or a mutable reference to the value stored at tail
     */
    pub fn get_tail_ref_mut(&mut self) -> Option<&mut T> {
        match self.get_length() { // matches based on the length of the list
            0 => None, // if the list is empty, return None
            _ => self.get_mutable_ref_at_index(self.get_length() - 1), // otherwise return the tail of the list as a mutable reference
        }
    }

    /*
        Objectives => get a reference to the value stored at a specified index
        Input(s) => (1 -> &self)  a reference to a LinkedList {IMPLICIT}; (2 -> index: u32) the desired index to get a reference to the value
        Output => an Option that either returns a reference to the field T at a particular index, or None, if the index is out of bounds
     */
    pub fn get_at_index(&self, index: u32) -> Option<&T>{
        let mut current_index: u32 = 0; // sets a mutable iterator for the list to 0 
        let mut current_node = &self.head; // sets a mutable variable as a reference to the head of the list
        while let Some(ref n) = current_node { // so long as there are nodes to iterate over, set n s current node...
            if current_index == index {  // if we are at the current_index is what we're looking for..
                return Some(&n.el); // return a reference to the element at the specified index
            } else { // if it's not the index we're looking for...
                current_node = &n.next; // set current_node to the node it points to
                current_index += 1; // increment the iterator
            }
        } None // if we "run out of list" return None
    }

     /*
        Objectives => get a mutable reference to the value stored at a specified index
        Input(s) => (1 -> &mut self) a mutable reference to a LinkedList {IMPLICIT}; (2 -> index: u32) the desired index to get a reference to the value
        Output => an Option that either returns a mutable reference to the field T at a particular index, or None, if the index is out of bounds
     */
    pub fn get_mutable_ref_at_index(&mut self, index: u32) -> Option<&mut T> {
        let mut current_index: u32 = 0; // sets a mutable iterator for the list to 0 
        let mut current_node = &mut self.head; // sets a mutable variable as a mutable reference to the head of the list
        while let Some(ref mut n) = current_node { // so long as there are nodes to iterate over, set n s current node...
            if current_index == index { // if we are at the current_index is what we're looking for..
                return Some(&mut n.el); // return a mutable reference to the element at the specified index
            } else { // if it's not the index we're looking for...
                current_node = &mut n.next; // set current_node to the node it points to
                current_index += 1; // increment the iterator
            }
        } None // if we "run out of list" return None
    }

    /*
        Objective => get the length of the list
        Input(s) => (1 -> &self)  a reference to a LinkedList {IMPLICIT}
        Output => LinkedList length as a u32
     */
    pub fn get_length(&self) -> u32 {
        self.length // quite literally just returns the value stored at length...
    }

    /*
        Objectives => check if the list is empty
        Input(s) => (1 -> &self)  a reference to a LinkedList {IMPLICIT}
        Output => a boolean value of whether the list does or does't have a length of 0
     */
    pub fn is_empty(&self) -> bool {
        match self.length { // pattern matches based on the length of the list
            0 => true, // if the length is 0, return true
            _ => false // if it's anything else, return false
        }
    }

    /*
        Objective => clear the LinkedList
        Input(s) => (1 -> &mut self) a mutable reference to a LinkedList {IMPLICIT}
        Output => None
     */
    pub fn clear(&mut self) {
        self.head = None; // sets the head to none freeing memory because there is no longer a reference to any of it
        self.length = 0; // sets the length to 0
    }

    /*
        Objectives => check whether the LinkedList contains a node with the specified element
        Input(s) => (1 -> &self) a reference to a LinkedList {IMPLICIT}; (2 -> item: T) a value of the generic
        Output => a boolean specifiying whether the value was found in the list
     */
    pub fn contains(&self, item: T) -> bool 
    where  
        T: PartialEq, // STIPULATES THAT "==" IS SPECIFIED FOR THE GENERIC T
    {
        let mut current_node = &self.head; // creates a mutable variable that is intialized as a reference to the head of the linked list
 
        while let Some(ref node) = current_node { // while we can iterate over the list (IE -> there are still nodes unchecked) set a variable node as a reference to current node
            if node.el == item { // if the element at the current_node is the item we are looking for...
                return true; // return true
            }
            else {
                current_node = &node.next; // go to the next node
            }
        } false // if we "run out of list" return false
    }

    /*
        Objectives => find an element in a LinkedList, and return an Option of the index it was found at
        Input(s) => (1 -> &self) a reference to a LinkedList {IMPLICIT}; (2 -> item: T) a value of the generic
        Output => a u32 representing the index a the item was found at (CAN BE NONE)
     */
    pub fn find(&self, item: T) -> Option<u32> 
    where 
        T: PartialEq, // STIPULATES THAT "==" IS SPECIFIED FOR THE GENERIC T
    {
        let mut current_node = &self.head; // creates a mutable variable that is intialized as a reference to the head of the linked list
        let mut inc: u32 = 0; // sets an icrementor so that we are able to store the "current index" as we iterate and return the correct value

        while let Some(ref node) = current_node { // while we can iterate over the list (IE -> there are still nodes unchecked) set a variable node as a reference to current node
                
            if node.el == item { // if the element at the current_node is the item we are looking for...
                return Some(inc); // return the index as an Option... Some(index)
            }
            else { // if it's not what we're looking for...
                current_node = &node.next; // go to the next node
                inc += 1; // increment
            }
            
        } None // if we "run out of list" return None
    }

}


impl<T: fmt::Display> fmt::Display for ListNode<T> 
where T: Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node: {}", self.el)
    }
}

impl<T: fmt::Display> fmt::Display for LinkedList<T> 
where T: Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current_node = &self.head;
        for _ in 0..self.length {
            if let Some(n) = current_node {
                write!(f, "{} ", n.el);
                current_node = &n.next;
            }
        }
        Ok(())


    }
}