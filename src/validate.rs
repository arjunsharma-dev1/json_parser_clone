use std::any::Any;
use std::collections::{HashMap};
use std::rc::Rc;
use crate::{Token, Value};
use std::cell::RefCell;
use std::ops::Deref;
use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::visit::{NodeIndexable, NodeRef};

pub struct JsonValidator {
    graph: Graph<Token, Option<()>>,
    stack: Vec<Rc<Token>>,
    current_node: Rc<NodeIndex>,
}

/*#[derive(Debug, Clone)]
struct Node {
    graph: Graph<Token, None>,
    current_node: NodeIndex
}*/

/*impl Node {
    fn add_multiple(&mut self, next_nodes: Vec<Rc<RefCell<Node>>>) {
        for next_node in next_nodes {
            self.next_tokens.insert(Rc::clone(&RefCell::clone(&next_node).borrow().current_token), next_node);
        }
    }
}*/

/*impl Node {
    fn default() -> Self {
        /*let string_token = Rc::new(Token::Value(Value::String("".to_string())));
        let number_token = Rc::new(Token::Value(Value::NumberFloating(0)));
        let boolean_token = Rc::new(Token::Value(Value::Boolean(false)));
        let null_token = Rc::new(Token::Null);
        let array_start_token = Rc::new(Token::ArrayStart);
        let array_end_token = Rc::new(Token::ArrayEnd);
        let object_start_token = Rc::new(Token::ObjectStart);
        let object_end_token = Rc::new(Token::ObjectEnd);
        let comma_token = Rc::new(Token::Comma);
        let colon_token = Rc::new(Token::Colon);*/

        /*let mut value_string = Rc::new(RefCell::new(Node::new(string_token.clone())));
        let mut value_number = Rc::new(RefCell::new(Node::new(number_token)));
        let mut value_boolean = Rc::new(RefCell::new(Node::new(boolean_token)));
        let mut value_null = Rc::new(RefCell::new(Node::new(null_token)));
        let mut array_start = Rc::new(RefCell::new(Node::new(Rc::clone(&array_start_token))));
        let mut array_end = Rc::new(RefCell::new(Node::new(array_end_token)));
        let mut object_start = Rc::new(RefCell::new(Node::new(Rc::clone(&object_start_token))));
        let mut object_end = Rc::new(RefCell::new(Node::new(object_end_token)));
        let mut comma = Rc::new(RefCell::new(Node::new(comma_token)));
        let mut colon = Rc::new(RefCell::new(Node::new(colon_token)));
        let mut key = Rc::new(RefCell::new(Node::new(string_token.clone())));*/



        /*array_start.deref().borrow_mut().add_multiple(vec![
            Rc::clone(&object_start),
            Rc::clone(&value_boolean),
            Rc::clone(&value_null),
            Rc::clone(&value_string),
            Rc::clone(&value_number)
        ]);*/

        /*comma.deref().borrow_mut().add_multiple(vec![
            Rc::clone(&value_boolean),
            Rc::clone(&value_null),
            Rc::clone(&value_string),
            Rc::clone(&value_number),
            Rc::clone(&array_start),
            Rc::clone(&object_start),
            Rc::clone(&key)
        ]);*/


        /*key.deref().borrow_mut().add_multiple(vec![Rc::clone(&colon_node)]);*/
        /*array_end.deref().borrow_mut().add_multiple(vec![Rc::clone(&comma), Rc::clone(&object_end)]);*/
        /*object_start.deref().borrow_mut().add_multiple(vec![
            Rc::clone(&key),
            Rc::clone(&object_end),
            Rc::clone(&object_start) // Add this line
        ]);*/

        /*value_string.deref().borrow_mut().add_multiple(vec![Rc::clone(&comma), Rc::clone(&array_end), Rc::clone(&object_end)]);*/
        /*value_number.deref().borrow_mut().add_multiple(vec![Rc::clone(&comma), Rc::clone(&array_end), Rc::clone(&object_end)]);*/
        /*value_null.deref().borrow_mut().add_multiple(vec![Rc::clone(&comma), Rc::clone(&array_end), Rc::clone(&object_end)]);*/
        /*value_boolean.deref().borrow_mut().add_multiple(vec![Rc::clone(&comma), Rc::clone(&array_end), Rc::clone(&object_end)]);*/

        /*object_end.deref().borrow_mut().add_multiple(vec![Rc::clone(&comma), Rc::clone(&array_end), ]);*/


        /*colon_node.deref().borrow_mut().add_multiple(vec![
            Rc::clone(&value_boolean),
            Rc::clone(&value_number),
            Rc::clone(&value_null),
            Rc::clone(&value_string),
            Rc::clone(&array_start),
            Rc::clone(&object_start),
        ]);*/


        /*let mut root: HashMap<Rc<Token>, Rc<RefCell<Node>>> = HashMap::new();
        root.insert(Rc::clone(&object_start_token), Rc::clone(&object_start));
        root.insert(Rc::clone(&array_start_token), Rc::clone(&array_start));

        Node::new_with_next_nodes(Token::Root, root)*/
    }

    /*fn new(token: Rc<Token>) -> Self {
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
    }*/
}*/



impl JsonValidator {

    pub fn new() -> Self {
        let mut graph: Graph<Token, Option<()>> = Graph::new();
        let root_node = graph.add_node(Token::Root);
        let string_node = graph.add_node(Token::Value(Value::String("".to_string())));
        let number_node = graph.add_node(Token::Value(Value::NumberFloating(0)));
        let boolean_node = graph.add_node(Token::Value(Value::Boolean(false)));
        let null_node = graph.add_node(Token::Null);
        let array_start_node = graph.add_node(Token::ArrayStart);
        let array_end_node = graph.add_node(Token::ArrayEnd);
        let object_start_node = graph.add_node(Token::ObjectStart);
        let object_end_node = graph.add_node(Token::ObjectEnd);
        let comma_node = graph.add_node(Token::Comma);
        let colon_node = graph.add_node(Token::Colon);
        let key_node = graph.add_node(Token::Value(Value::String("".to_string())));

        graph.add_edge(array_start_node, object_start_node, None);
        graph.add_edge(array_start_node, boolean_node, None);
        graph.add_edge(array_start_node, null_node, None);
        graph.add_edge(array_start_node, string_node, None);
        graph.add_edge(array_start_node, number_node, None);

        graph.add_edge(comma_node, object_start_node, None);
        graph.add_edge(comma_node, array_start_node, None);
        graph.add_edge(comma_node, key_node, None);
        graph.add_edge(comma_node, boolean_node, None);
        graph.add_edge(comma_node, null_node, None);
        graph.add_edge(comma_node, string_node, None);
        graph.add_edge(comma_node, number_node, None);

        graph.add_edge(key_node, colon_node, None);

        graph.add_edge(object_start_node, key_node, None);
        graph.add_edge(object_start_node, object_end_node, None);
        graph.add_edge(object_start_node, object_start_node, None);

        graph.add_edge(string_node, comma_node, None);
        graph.add_edge(string_node, array_end_node, None);
        graph.add_edge(string_node, object_end_node, None);

        graph.add_edge(number_node, comma_node, None);
        graph.add_edge(number_node, array_end_node, None);
        graph.add_edge(number_node, object_end_node, None);

        graph.add_edge(null_node, comma_node, None);
        graph.add_edge(null_node, array_end_node, None);
        graph.add_edge(null_node, object_end_node, None);

        graph.add_edge(boolean_node, comma_node, None);
        graph.add_edge(boolean_node, array_end_node, None);
        graph.add_edge(boolean_node, object_end_node, None);

        graph.add_edge(object_end_node, comma_node, None);
        graph.add_edge(object_end_node, array_end_node, None);

        graph.add_edge(colon_node, object_start_node, None);
        graph.add_edge(colon_node, array_start_node, None);
        graph.add_edge(colon_node, string_node, None);
        graph.add_edge(colon_node, boolean_node, None);
        graph.add_edge(colon_node, number_node, None);
        graph.add_edge(colon_node, null_node, None);

        graph.add_edge(root_node, object_start_node, None);
        graph.add_edge(root_node, array_start_node, None);

        graph.add_edge(array_end_node, comma_node, None);


        JsonValidator {
            stack: Vec::new(),
            graph,
            current_node: Rc::new(root_node),
        }
    }

    fn is_neighbour(&self, first: NodeIndex, second: NodeIndex) -> bool {
        let graph = &self.graph;
        graph.neighbors(first).any(|neighbour| second.weight().eq(&neighbour.weight()))
    }

    pub fn validate(&mut self, next_token: &Token) -> bool {
        let graph: &Graph<Token, Option<()>> = &self.graph;

        let current_token_node = *self.current_node.deref();
        let next_token_node = graph.raw_nodes()
            .iter()
            .position(|x| x.weight.eq(next_token))
            .map(NodeIndex::new);
        if next_token_node.is_none() {
            panic!("Invalid JSON");
        }
        let next_token_node = next_token_node.unwrap();
        let is_neighbour = self.is_neighbour(current_token_node, next_token_node);

        if next_token.eq(&Token::ObjectStart) || next_token.eq(&Token::ArrayStart) {
            self.stack.push(Rc::new(next_token.clone()));
        }

        if next_token.eq(&Token::ObjectEnd) || next_token.eq(&Token::ArrayEnd) {
            let is_start_present = match self.stack.last() {
                Some(token) => {
                    let end_token = get_end(token).unwrap();
                    next_token.deref().eq(&end_token)
                },
                None => false
            };

            if !is_start_present {
                panic!("Invalid JSON");
            }
            self.stack.pop();
        }

        self.current_node = Rc::new(next_token_node);
        is_neighbour
    }
}

fn get_end(token: &Token) -> Option<Token> {
    return match token {
        Token::ArrayStart => Some(Token::ArrayEnd),
        Token::ObjectStart => Some(Token::ObjectEnd),
        _ => None
    }
}