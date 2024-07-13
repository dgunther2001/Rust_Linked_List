
pub mod linked_list;
use crate::linked_list::LinkedList;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_list() {
        let list: LinkedList<i32> = LinkedList::init_list();
        assert!(list.is_empty(), "Newly initialized list should be empty");
    }

    #[test]
    fn test_append_front() {
        let mut list = LinkedList::init_list();
        list.append_front(1000);
        list.append_front(55);
        assert_eq!(list.get_head(), Some(&55), "Front of list should be the most recently appended element");
        assert_eq!(list.get_length(), 2, "Length should be adjusted properly");
    }


    #[test]
    fn test_append_back() {
        let mut list = LinkedList::init_list();
        list.append_back(37);
        list.append_back(33);
        assert_eq!(list.get_at_index(1), Some(&33), "End of list should be most recently appeneded element");
        assert_eq!(list.get_length(), 2, "Length should be adjusted properly");
    }

    #[test]
    fn test_insert_at_index() {
        let mut list = LinkedList::init_list();
        list.insert_at_index(22, 0);
        list.insert_at_index(10, 1);
        list.insert_at_index(23, 1);
        list.insert_at_index(47000, 6500);
        assert_eq!(list.get_at_index(1), Some(&23), "Indexing should adjust as we insert new elements in the middle");
        assert_eq!(list.get_length(), 4, "Length should be adjusted properly");
    }

    #[test]
    fn test_delete_front() {
        let mut list = LinkedList::init_list();
        list.append_front(22);
        list.append_front(34);
        list.delete_front();
        assert_eq!(list.get_head(), Some(&22), "Deletion should occur at the front of the list");
        assert_eq!(list.get_length(), 1, "Length should be adjusted properly");
    }

    #[test]
    fn test_delete_back() {
        let mut list = LinkedList::init_list();
        list.append_back(1000);
        list.append_back(100054);
        list.delete_back();
        assert_eq!(list.get_at_index(1), None, "Index should no longer exist if deletion done properly");
        assert_eq!(list.get_length(), 1, "Length should be adjusted properly");
    }

    #[test]
    fn test_delete_at_index() {
        let mut list = LinkedList::init_list();
        list.append_front(10);
        list.append_back(20);
        list.append_back(30);
        list.delete_at_index(1);
        assert_eq!(list.get_at_index(1), Some(&30), "Deletion not occuring properly");
        assert_eq!(list.get_length(), 2, "Length should be adjusted properly");
    }

    #[test]
    fn test_get_head() {
        let mut list = LinkedList::init_list();
        list.append_front(33);
        list.append_back(57);
        assert_eq!(list.get_head(), Some(&33), "Returned head is incorrect");

    }

    #[test]
    fn test_get_head_as_mut() {
        let mut list = LinkedList::init_list();
        list.append_front(33);
        list.append_back(57);
        if let Some(val) = list.get_head_as_mut() {
            *val = 10000;
        }
        assert_eq!(list.get_head(), Some(&10000), "Should be able to mutate the element at head");
    }

    #[test]
    fn test_get_tail_ref() {
        let mut list: LinkedList<i32> = LinkedList::init_list();
        list.append_back(33);
        list.append_back(57);
        list.append_front(31);
        assert_eq!(list.get_tail_ref(), Some(&57), "Unable to retrieve correct tail reference");
        list.delete_back();
        assert_eq!(list.get_tail_ref(), Some(&33), "Unable to retrieve correct tail reference");

    }

    #[test]
    fn test_get_tail_ref_mut() {
        let mut list: LinkedList<i32> = LinkedList::init_list();
        list.append_back(99);
        list.append_front(-31);
        list.append_front(-33);
        if let Some(my_var) = list.get_tail_ref_mut() {
            *my_var = -10000;
        }
        assert_eq!(list.get_tail_ref(), Some(&-10000), "Mutable reference to tail not updating properly");
    }

    #[test]
    fn test_get_at_index() {
        let mut list = LinkedList::init_list();
        list.append_front("Hello");
        list.append_front("Goodbye");
        assert_eq!(list.get_at_index(1), Some(&"Hello"), "Not returning the correct element");
        assert_eq!(list.get_length(), 2, "Incorrect length");
    }

    #[test]
    fn test_get_mutable_ref_at_index() {
        let mut list = LinkedList::init_list();
        list.append_front(0xffff);
        list.append_front(0xcec1);
        if let Some(test_var) = list.get_mutable_ref_at_index(1) {
            *test_var = 0x0001;
        }
        assert_eq!(list.get_at_index(1), Some(&0x0001), "Mutable reference not being returned properly")
    }

    #[test]
    fn test_get_length() {
        let mut list = LinkedList::init_list();
        list.append_front(33);
        list.append_front(54);
        list.append_front(65);
        assert_eq!(list.get_length(), 3, "Length incorrect");
        list.delete_back();
        assert_eq!(list.get_length(), 2, "Length incorrect");
    }

    #[test]
    fn test_is_empty() {
        let mut list = LinkedList::init_list();
        list.append_front(65);
        assert_eq!(list.is_empty(), false, "List reported as empty when non-empty");
        list.delete_front();
        assert_eq!(list.is_empty(), true, "List reported as non-empty when empty")
    }

    #[test]
    fn test_clear() {
        let mut list = LinkedList::init_list();
        list.insert_at_index(22, 0);
        list.insert_at_index(10, 1);
        list.clear();
        assert!(list.is_empty(), "List should be empty after clearing");
    }

    #[test]
    fn test_contains() {
        let mut list = LinkedList::init_list();
        list.append_back(33);
        list.append_back(37);
        assert_eq!(list.contains(33), true, "Not reporting entry in list");
        assert_eq!(list.contains(10000), false, "Falsely reporting an entry in the list");
    }

    #[test]
    fn test_find() {
        let mut list = LinkedList::init_list();
        list.append_back("Daniel");
        list.append_back("George");
        list.append_back("Larry");
        assert_eq!(list.find("George"), Some(1), "Not reporting the proper index in which an item currently exists");
        assert_eq!(list.find("John"), None, "Incorrectly reporting an index for an item that should not be there");
    }
    
}
