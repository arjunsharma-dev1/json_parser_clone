use std::rc::Rc;
use crate::{Token, Value};
use petgraph::Graph;
use petgraph::graph::NodeIndex;

pub struct JsonValidator {
    graph: Graph<Token, Option<()>>,
    stack: Vec<Rc<Token>>,
    current_node: Rc<NodeIndex>,
}

impl JsonValidator {

    pub fn is_done_processing(&self) -> bool {
        self.stack.is_empty()
    }

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
        let key_node = graph.add_node(Token::Key("".to_string()));

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
        graph.neighbors(first).any(|neighbour| {
            second == neighbour
        })
    }

    pub fn validate(&mut self, next_token: &Token) -> bool {
        let graph: &Graph<Token, Option<()>> = &self.graph;

        let current_token_node = *self.current_node;
        let next_token_node = graph.raw_nodes()
            .iter()
            .position(|x| {
                x.weight.eq(next_token)
            })
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
                    next_token.eq(&end_token)
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