use std::collections::HashMap;
use std::collections::LinkedList;
struct Vertex {
    id: u64,
    best_distance: i64,
    best_verticie: u64,
    arcs: HashMap<u64, i64>,
}

struct Graph {
    source: u64,
    destination: u64,
    verticies: Vec<Vertex>,
    touch_dest: bool,
}

struct BestPath {
    distance: i64,
    path: Vec<Vertex>,
}


fn new_vertex(id: u64) -> Vertex {
    Vertex {
        id: id,
        best_distance: 0,
        best_verticie: 0,
        arcs: HashMap::new(),
    }
}

fn new_graph() -> Graph {
    Graph {
        source: 0,
        destination: 0,
        verticies: Vec::new(),
        touch_dest: false,
    }
}

impl Graph {
    fn add_vertex(&mut self, v: Vertex) {
        self.verticies.push(v);
    }

    fn remove_vertex(&mut self, id: u64) {
        let mut i = 0;
        while i < self.verticies.len() {
            if self.verticies[i].id == id {
                self.verticies.remove(i);
                return;
            }
        }
    }
    fn setup(&mut self) {
        //set best distance to i64max
        //reset all best distance and vertex
    }
    fn shortest(&mut self, source: u64, destination: u64) -> BestPath {
        let mut best = 0;
        let mut visiting = LinkedList::new();
        let mut v;
        self.setup();
        visiting.append(source);
        while visiting.len() > 0 {
            v = self.verticies.get(visiting.pop());
            for (id, dist) in self.verticies[visit].arcs.iter() {
                if v.distance > self.verticies.get(id).best_distance {}
            }
        }

    }
}

impl Vertex {
    fn add_arc(&mut self, to: u64, distance: i64) {
        self.arcs.insert(to, distance);
    }

    fn remove_arc(&mut self, to: u64) {
        self.arcs.remove(&to);
    }
}

fn main() {
    let g = new_graph();

}
