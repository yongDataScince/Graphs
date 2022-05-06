use serde_json;
use serde::{
    Serialize,
    Deserialize,
};
use std::{fs::File, io::Read, collections::HashMap, hash::Hash, fmt::Debug};

type Table = Vec<Vec<u16>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Node<T: Hash + Eq + Copy + Debug> {
    name: T,
    subjects: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Graph<T: Hash + Eq + Copy + Debug> {
    nodes: Vec<Node<T>>,
    table: Table
}

impl<'a, T: Deserialize<'a> + Hash + Eq + Copy + Debug> Graph<T> {
    fn new(file_name: Option<String>) -> Graph<T> {
        let mut new_graph = match file_name {
            None => Graph { nodes: Vec::new(), table: Vec::new() },
            Some(file_name) => Self::load_graph(file_name)
        };

        new_graph.create_table();
        new_graph.draw_table();
        new_graph
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

        Graph { nodes, table: Vec::new() }
    }

    fn zeros(&self, n: u32) -> Vec<u16> {
        (0..n).into_iter().map(|_| 0).collect()
    }

    fn create_table(&mut self) {
        let mut table: Table = vec![];
        let mut field_ids:HashMap<T, usize> = HashMap::new();

        let n: usize = *(&self.nodes.len());
        // create empty table
        (0..n).into_iter().for_each(|_| table.push( self.zeros(n as u32)));

        let _ = &self.nodes.clone().into_iter()
            .map(|node: Node<T>| -> T { node.name })
            .enumerate()
            .for_each(|(i, name): (usize, T)| {
                field_ids.insert(name, i);
            });

        let _ = self.nodes.clone().into_iter()
            .for_each(|node: Node<T>| {
                let col = field_ids.get(&node.name).unwrap();
                node.subjects.into_iter().for_each(|s: T| {
                    let row = field_ids.get(&s).unwrap();
                    table[*row][*col] = 1;
                });
            });

        self.table = table;
    }

    fn draw_table(&self) {
        self.nodes
            .clone()
            .into_iter()
            .enumerate()
            .for_each(|(i, _node): (usize, Node<T>)| {
                println!("{:?}", self.table[i]);
            });
    }

    fn close_triplet_count(&self) -> f32 {
        // number of triangles
        let v = self.len();

        let mul = |(a, b) : (&Table, &Table)| -> Vec<Vec<u16>> {
            let mut res: Vec<Vec<u16>>  = (0..v).into_iter().map(|_| self.zeros(v as u32)).collect();
            for i in 0..v {
                for j in 0..v {
                    res[i as usize][j as usize] = 0;
                    for k in 0..v {
                        res[i as usize][j as usize] += (a[i][k] * b[k][j]) as u16;
                    }
                }
            }
            res
        };

        let trace = |table: Table| -> u16 {
            let mut t = 0;
            for i in 0..v {
                t += table[i][i]
            }
            t as u16
        };

        // let mut aux2: Vec<Vec<u8>>  = (0..v).into_iter().map(|_| self.zeros(v as u32)).collect();
        // let mut aux3: Vec<Vec<u8>>  = (0..v).into_iter().map(|_| self.zeros(v as u32)).collect();

        let aux2 = mul((&self.table, &self.table));
        let aux3 = mul((&self.table, &aux2));

        (trace(aux3) / 2).into()
    }

    fn open_triplet_count(&self) -> f32 {
        let mut c = 0;

        self.nodes.clone().into_iter().for_each(|node: Node<T>| {
            if node.subjects.len() > 1 { c += 2}
        });

        c as f32
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn c(&self) -> f32 {
        println!("{} / {}", self.close_triplet_count(), self.close_triplet_count() + self.open_triplet_count());
        self.close_triplet_count()
            / ( self.close_triplet_count() + self.open_triplet_count())
    }

    fn load_from_table(file_name: String) -> Graph<T> {
        todo!()
    }
}

fn main() {
    let graph: Graph<&str> = Graph::new(Some("./data/graph_data.json".to_string()));

    println!("{:.5}", graph.c())
}
