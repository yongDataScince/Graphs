use serde_json;
use serde::{
    Serialize,
    Deserialize,
};
use std::{fs::File, io::Read};

#[derive(Debug, Serialize, Deserialize)]
struct Node<T> {
    name: T,
    subjects: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Graph<T> {
    nodes: Vec<Node<T>>,
}

impl<'a, T: Deserialize<'a>> Graph<T> {
    fn new(file_name: Option<String>) -> Graph<T> {
        match file_name {
            None => Graph { nodes: Vec::new() },
            Some(file_name) => Self::load_graph(file_name)
        }
    }

    fn load_graph(file_name: String) -> Graph<T> {
        /*
            Format of JSON file:
            [
                {"name": <T>, "subjects": [<T>...]},
            ]
        */
        let mut str_file = String::new();
        let mut file = File::open(file_name).unwrap();
        let _ = file.read_to_string(&mut str_file);

        let arg = Box::new(str_file);
        let arg: &'static str = Box::leak(arg);

        let nodes = serde_json::from_str::<Vec<Node<T>>>(arg).unwrap();

        Graph { nodes }
    }
}

fn main() {
    let graph: Graph<String> = Graph::new(Some("./data/graph_data.json".to_string()));
    println!("{:?}", graph)
}
