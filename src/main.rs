use serde_json;
use serde::{
    Serialize,
    Deserialize,
};
use std::{fs::File, io::Read, collections::HashMap, hash::Hash, fmt::Debug};

type Table = Vec<Vec<u16>>;

enum GraphInit {
    LoadFromJSON(String),
    Empty
}

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
    fn new(init_type: GraphInit) -> Graph<T> {
        match init_type {
            GraphInit::LoadFromJSON(file_name) => Graph::load_graph(file_name),
            GraphInit::Empty => Graph {
                nodes: Vec::new(),
                table: Vec::new()
            },
        }
    }

    fn load_graph(file_name: String) -> Graph<T> {
        /*
            Format of JSON file:
            [X
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

    fn create_table(&mut self) {
        let mut table: Table = vec![];
        let mut field_ids:HashMap<T, usize> = HashMap::new();
        // Count of vertecles
        let n: usize = *(&self.nodes.len());
        // create empty table
        (0..n).into_iter().for_each(|_| table.push( zeros(n)));

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
        self.table
            .clone()
            .into_iter()
            .for_each(|line| {
                println!("{:?}", line);
            });
    }

    fn close_triplet_count(&self) -> f32 {
        // number of triangles
        let v = self.len();

        let mul = |(a, b) : (&Table, &Table)| -> Vec<Vec<u16>> {
            let mut res: Vec<Vec<u16>>  = (0..v).into_iter().map(|_| zeros(v)).collect();
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

    /* 
        Global clusterring coeff.:

            number of closed triplets
        -----------------------------------
            number of connected triples
    */
    fn c(&self) -> f32 {
        println!("{} / {}", self.close_triplet_count(), self.close_triplet_count() + self.open_triplet_count());
        self.close_triplet_count()
            / ( self.close_triplet_count() + self.open_triplet_count())
    }

    fn load_from_table(file_name: String) -> Graph<u32> {
        let mut str_file = String::new();
        let mut file = File::open(file_name).unwrap();
        let _ = file.read_to_string(&mut str_file);

        let table_len = str_file.split("\n").collect::<Vec<_>>().len();
        let mut new_table: Table = (0..table_len).into_iter().map(|_| zeros(table_len)).collect();

        str_file.split("\n").into_iter().enumerate().for_each(|(row, line)| {
            line.split(",").into_iter().enumerate().for_each(|(col, digit)| {
                // new_table[row][col] = digit.parse::<u16>().unwrap();
                match digit.parse::<u16>() {
                    Ok(data) => new_table[row][col] = data,
                    Err(_) => println!("invalid digit: {}", digit)
                };
            })
        });

        let new_nodes: Vec<Node<u32>> = new_table
            .clone()
            .into_iter()
            .enumerate()
            .map(|(idx, row)| {
                let mut curr_node = Node { name: idx as u32, subjects: zeros(table_len).into_iter().map(|x| x as u32).collect()};
                row.iter().enumerate().into_iter().for_each(|(i, val)| {
                    if *val == 1 {
                        curr_node.subjects[i] = i as u32;
                    }
                });
                curr_node
            })
            .collect();

        Graph { nodes: new_nodes, table: new_table }
    }
}

fn zeros(n: usize) -> Vec<u16> {
    (0..n).into_iter().map(|_| 0).collect()
}

fn main() {
    let mut graph: Graph<&str> = Graph::new();
    println!("{:.5}", graph.c());

    graph.load_from_table("./data/table.txt".to_string());

    println!("{:.5}", graph.c())
}
