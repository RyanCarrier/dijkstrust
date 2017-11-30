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
        let mut i = 0;
        let l = self.verticies.len();
        while i < l {
            self.verticies[i].best_distance = std::i64::MAX;
            self.verticies[i].best_verticie = 0;
            i += 1;
        }
        //set best distance to i64max
        //reset all best distance and vertex
    }
    fn shortest(&mut self, source: &mut u64, destination: u64) -> BestPath {
        let mut visiting = LinkedList::new();
        let mut touch_dest = false;
        let mut best_distance = std::i64::MAX;
        self.setup();
        visiting.push_back(*source);
        while visiting.len() > 0 {
            //for each node we are visiting, get it's current id, distance and arcs to other nodes
            let visitingid = visiting.pop_front().unwrap();
            let distance = self.verticies[visitingid as usize].best_distance;
            let arcs = &self.verticies[visitingid as usize].arcs;
            //for each arc to other nodes, check if that path is the new best path to it
            //(from our visiting node)
            for (mut id, mut dist) in arcs.iter() {
                //Check if this arc will be the new best path to the arc'd node
                if distance + dist >= self.verticies[*id as usize].best_distance {
                    continue;
                }
                if *id == destination && distance + dist < best_distance {
                    touch_dest = true;
                }
                //if it is the new best node, make sure we update it's distance and best node, then
                //set it to be (re)visited later
                self.verticies[*id as usize].best_distance = distance + dist;
                self.verticies[*id as usize].best_verticie = *id;
                visiting.push_back(*id)
            }
        }

        let mut b = BestPath {
            success: touch_dest,
            distance: best_distance,
            path: Vec::new(),
        };
        //you can just fuck this stuff right off if you need to
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
    let _g = new_graph();

}
