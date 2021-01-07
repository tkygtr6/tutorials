use std::rc::Rc;
use std::cell::RefCell;
use std::mem;

struct Node {
    val: i32,
    next: Option<Box<Node>>
}

struct SingleLinkedList {
    head: Box<Node>,
}

impl SingleLinkedList {
    fn new() -> SingleLinkedList {
        let mut head_node = Node{
            val: -1,
            next: None
        };

        SingleLinkedList {
            head: Box::new(head_node),
        }
    }

    fn insert_from_head(&mut self, val: i32) {
        let new_node = Node {
            val,
            next: mem::replace(&mut (*self.head).next, None)
        };
        (*self.head).next = Some(Box::new(new_node));
    }

    fn output_vals(&self) -> Vec<i32> {
        let mut vec_output: Vec<i32> = vec![];
        let mut current_node: &Box<Node> = &self.head;
        loop {
            match &(*current_node).next {
                Some(cur_next) => {
                    current_node = &cur_next;
                    vec_output.push((*current_node).val)
                },
                _ => { break; }
            }
        }
        vec_output
    }

    fn print_output_vals(&self) {
        for item in self.output_vals() {
            println!("{}", item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert1() {
        let mut sll = SingleLinkedList::new();
        sll.insert_from_head(1);
        sll.insert_from_head(2);
        sll.insert_from_head(3);

        let vec_output = sll.output_vals();
        assert_eq!(vec_output[0], 3);
        assert_eq!(vec_output[1], 2);
        assert_eq!(vec_output[2], 1);
    }
}
