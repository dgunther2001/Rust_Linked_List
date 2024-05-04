
pub mod linked_list;
use crate::linked_list::LinkedList;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = LinkedList::init_list();
        list.append_front(1022);
        list.append_front(1000);
        list.append_front(1500);
        //list.insert_node(10, 1);
        list.append_back(37);
        list.insert_at_index(47000, 6500);

        println!("{}", list);

        //LinkedList::static_delete_back(&mut list.head);
        /* 
        if let Some(node) = list.delete_back(){
            println!("{}", node);
        }
        */

        list.delete_at_index(3);
        println!("{}", list);

        list.delete_at_index(3);
        println!("{}", list);

        list.delete_at_index(0);
        



        //list.delete_front();
        /* 
        if let Some(myvar) = list.delete_front() {
            println!("{}", myvar);
        }
        */
        println!("{}", list);

        if let Some(var) = list.get_head_as_mut() {
            *var = 72;
        }

        if let Some(var2) = list.get_at_index(1) {
            println!("{}", var2);
        }

        println!("{}", list);


        if let Some(var3) = list.get_mutable_ref_at_index(1) {
            *var3 = 89;
        }


        println!("{}", list);

        list.append_back(10234);
        list.append_back(33);
        list.append_back(67435);

        println!("{}", list);

        println!("{}", list.contains(10234));
        println!("{}", list.contains(10233));
        println!();
        println!("{:?}", list.find(33));
        println!("{:?}", list.find(32));

        //list.clear();
        
        //println!("cleared?: {}", list);

        /* 
        if let Some(my_node) = list.get_head_ref() {
            if let Some(next_node) = my_node.next() {
                println!("{}", next_node);
            }
        }
        */

        println!();
        println!("{:?}", list.delete_at_index(4));
        println!("{:?}", list.delete_at_index(4));
        


    }
}
