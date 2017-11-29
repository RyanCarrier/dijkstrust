use std::collections::HashMap;
use std::collections::LinkedList;
struct Vertex {
    id: u64,
    best_distance: i64,
    best_verticie: u64,
    arcs: HashMap<u64, i64>,
}

struct Graph {
    verticies: Vec<Vertex>,
}

struct BestPath {
    success: bool,
    distance: i64,
    path: Vec<u64>,
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
    Graph { verticies: Vec::new() }
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
        for mut v in self.verticies {
            v.best_distance = std::i64::MAX;
            v.best_verticie = 0;
        }
        //set best distance to i64max
        //reset all best distance and vertex
    }
    fn shortest(&mut self, source: &mut u64, destination: u64) -> BestPath {
        let mut best = 0;
        let mut visiting = LinkedList::new();
        let mut touch_dest;
        let mut best_distance = std::i64::MAX;
        self.setup();
        visiting.push_back(*source);
        while visiting.len() > 0 {
            let mut visitingid = visiting.pop_front().unwrap();
            for (id, dist) in v.arcs.iter() {
                if v.best_distance + dist >= self.verticies[*id as usize].best_distance {
                    continue;
                }
                if *id == destination && v.best_distance + dist < best_distance {
                    touch_dest = true;
                }
                self.verticies[*id as usize].best_distance = v.best_distance + dist;
                self.verticies[*id as usize].best_verticie = *id;
                visiting.push_back(*id)
            }
        }

        let mut b = BestPath {
            success: touch_dest,
            distance: best_distance,
            path: Vec::new(),
        };
        let v = &self.verticies[*source as usize];
        b.path.push(v.id);
        while v.id != destination {
            let v = &self.verticies[v.best_verticie as usize];
            b.path.push(v.id);
        }
        return b;
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
