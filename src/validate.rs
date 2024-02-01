use std::collections::{HashMap};
use std::rc::Rc;
use crate::{Token, Value};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::Deref;
use std::ops::DerefMut;

pub struct JsonValidator {
    stack: Vec<Rc<Token>>,
    node: Rc<RefCell<Node>>
}

#[derive(Debug, Clone)]
struct Node {
    current_token: Rc<Token>,
    next_tokens: HashMap<Rc<Token>, Rc<RefCell<Node>>>
}

impl Node {
    fn add_multiple(&mut self, next_nodes: Vec<Rc<RefCell<Node>>>) {
        for next_node in next_nodes {
            self.next_tokens.insert(Rc::clone(&RefCell::clone(&next_node).borrow().current_token), next_node);
        }
    }
}

impl Node {
    fn default() -> Self {
        let string_token = Rc::new(Token::Value(Value::String("".to_string())));
        let number_token = Rc::new(Token::Value(Value::NumberFloating(0)));
        let boolean_token = Rc::new(Token::Value(Value::Boolean(false)));
        let null_token = Rc::new(Token::Null);
        let array_start_token = Rc::new(Token::ArrayStart);
        let array_end_token = Rc::new(Token::ArrayEnd);
        let object_start_token = Rc::new(Token::ObjectStart);
        let object_end_token = Rc::new(Token::ObjectEnd);
        let comma_token = Rc::new(Token::Comma);
        let colon_token = Rc::new(Token::Colon);

        let mut value_string = Rc::new(RefCell::new(Node::new(string_token.clone())));
        let mut value_number = Rc::new(RefCell::new(Node::new(number_token)));
        let mut value_boolean = Rc::new(RefCell::new(Node::new(boolean_token)));
        let mut value_null = Rc::new(RefCell::new(Node::new(null_token)));
        let mut array_start = Rc::new(RefCell::new(Node::new(Rc::clone(&array_start_token))));
        let mut array_end = Rc::new(RefCell::new(Node::new(array_end_token)));
        let mut object_start = Rc::new(RefCell::new(Node::new(Rc::clone(&object_start_token))));
        let mut object_end = Rc::new(RefCell::new(Node::new(object_end_token)));
        let mut comma = Rc::new(RefCell::new(Node::new(comma_token)));
        let mut colon = Rc::new(RefCell::new(Node::new(colon_token)));
        let mut key = Rc::new(RefCell::new(Node::new(string_token.clone())));

        array_start.deref().borrow_mut().add_multiple(vec![
            Rc::clone(&object_start),
            Rc::clone(&value_boolean),
            Rc::clone(&value_null),
            Rc::clone(&value_string),
            Rc::clone(&value_number)
        ]);

        comma.deref().borrow_mut().add_multiple(vec![
            Rc::clone(&value_boolean),
            Rc::clone(&value_null),
            Rc::clone(&value_string),
            Rc::clone(&value_number),
            Rc::clone(&array_start),
            Rc::clone(&object_start),
            Rc::clone(&key)
        ]);


        key.deref().borrow_mut().add_multiple(vec![Rc::clone(&colon)]);
        array_end.deref().borrow_mut().add_multiple(vec![Rc::clone(&comma), Rc::clone(&object_end)]);
        // object_start.deref().borrow_mut().add_multiple(vec![Rc::clone(&key), Rc::clone(&object_end)]);
        object_start.deref().borrow_mut().add_multiple(vec![
            Rc::clone(&key),
            Rc::clone(&object_end),
            Rc::clone(&object_start) // Add this line
        ]);

        value_string.deref().borrow_mut().add_multiple(vec![Rc::clone(&comma), Rc::clone(&array_end), Rc::clone(&object_end)]);
        value_number.deref().borrow_mut().add_multiple(vec![Rc::clone(&comma), Rc::clone(&array_end), Rc::clone(&object_end)]);
        value_null.deref().borrow_mut().add_multiple(vec![Rc::clone(&comma), Rc::clone(&array_end), Rc::clone(&object_end)]);
        value_boolean.deref().borrow_mut().add_multiple(vec![Rc::clone(&comma), Rc::clone(&array_end), Rc::clone(&object_end)]);

        object_end.deref().borrow_mut().add_multiple(vec![Rc::clone(&comma), Rc::clone(&array_end), ]);


        colon.deref().borrow_mut().add_multiple(vec![
            Rc::clone(&value_boolean),
            Rc::clone(&value_number),
            Rc::clone(&value_null),
            Rc::clone(&value_string),
            Rc::clone(&array_start),
            Rc::clone(&object_start),
        ]);


        let mut root: HashMap<Rc<Token>, Rc<RefCell<Node>>> = HashMap::new();
        root.insert(Rc::clone(&object_start_token), Rc::clone(&object_start));
        root.insert(Rc::clone(&array_start_token), Rc::clone(&array_start));

        Node::new_with_next_nodes(Token::Root, root)
    }

    fn new(token: Rc<Token>) -> Self {
        Node {
            current_token: token,
            next_tokens: HashMap::new()
        }
    }
    fn new_with_next_nodes(token: Token, next_nodes: HashMap<Rc<Token>, Rc<RefCell<Node>>>) -> Self {
        Node {
            current_token: Rc::new(token),
            next_tokens: next_nodes
        }
    }
}



impl JsonValidator {

    pub fn new() -> Self {
        JsonValidator {
            stack: Vec::new(),
            node: Rc::new(RefCell::new(Node::default()))
        }
    }
    pub fn validate(&mut self, next_token: &Token) -> bool {
        // dbg!(&self.node.borrow().current_token);
        // // dbg!(&self.node.borrow().next_tokens.get(next_token).and_then(|value| Some(&value.borrow().current_token)));
        // dbg!(next_token);
        let previous_node = Rc::clone(&self.node);
        let borrow = previous_node.borrow();
        let next_node = borrow.next_tokens.get(next_token);

        if next_node.is_none() {
            return false;
        }
        let next_node_value = next_node.unwrap();

        let next_token = &next_node_value.deref().borrow().current_token;
        if next_token.deref().eq(&Token::ObjectStart) || next_token.deref().eq(&Token::ArrayStart) {
            self.stack.push(Rc::clone(next_token));
        }

        if next_token.deref().eq(&Token::ObjectEnd) || next_token.deref().eq(&Token::ArrayEnd) {
            let is_start_present = match self.stack.last() {
                Some(mut token) => {
                    let end_token = get_end(token.deref()).unwrap();
                    next_token.deref().eq(&end_token)
                },
                None => false
            };

            if !is_start_present {
                panic!("Invalid JSON");
            }
            self.stack.pop();
        }

        self.node = Rc::clone(next_node_value);
        return true;
    }
}

fn get_end(token: &Token) -> Option<Token> {
    return match token {
        Token::ArrayStart => Some(Token::ArrayEnd),
        Token::ObjectStart => Some(Token::ObjectEnd),
        _ => None
    }
}