use std::collections::LinkedList;
use std::fmt;

#[derive(Debug)]
struct Arc {
    to: u64,
    distance: i64,
}

struct Vertex {
    id: u64,
    best_distance: i64,
    best_verticie: u64,
    arcs: Vec<Arc>,
}

impl fmt::Debug for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\n\tid:{}\n\tbest_distance:{}\n\tbest_verticie:{}\n\tarcs:{:?}\n",
            self.id,self.best_distance,self.best_verticie,self.arcs,
        )
    }
}


impl Vertex {
    fn add_arc(&mut self, to: u64, distance: i64) {
        self.arcs.push(Arc {
            to: to,
            distance: distance,
        });
    }

    fn remove_arc(&mut self, to: u64) {
        self.arcs.retain(|a| a.to != to);
    }

    fn new(id: u64) -> Vertex {
        Vertex {
            id: id,
            best_distance: 0,
            best_verticie: 0,
            arcs: Vec::new(),
        }
    }
}


struct Graph {
    verticies: Vec<Vertex>,
}
impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "verticies:\n{:?}\n",
            self.verticies,
        )
    }
}

#[derive(Debug)]
struct BestPath {
    success: bool,
    distance: i64,
    path: Vec<u64>,
}

impl Graph {
    fn new() -> Graph {
        Graph { verticies: Vec::new() }
    }

    fn add_vertex(&mut self, v: Vertex) {
        self.verticies.push(v);
    }

    fn remove_vertex(&mut self, id: u64) {
        self.verticies.retain(|v| v.id != id)
    }

    fn setup(&mut self, source: u64) {
        for vertex in &mut self.verticies {
            vertex.best_distance = std::i64::MAX;
            vertex.best_verticie = 0;
        }
        self.verticies[source as usize].best_distance = 0;
        //set best distance to i64max
        //reset all best distance and vertex
    }

    fn best_path(&self, source: u64, destination: u64, touch_dest: bool) -> BestPath {
        let mut b = BestPath {
            success: touch_dest,
            distance: self.verticies[destination as usize].best_distance,
            path: Vec::new(),
        };
        //you can just fuck this stuff right off if you need to
        let mut vid = destination;
        b.path.push(vid);
        while vid != source {
            vid = self.verticies[vid as usize].best_verticie.clone();
            b.path.push(vid);
        }
        b.path.reverse();
        b
    }

    fn shortest(&mut self, source: u64, destination: u64) -> Option<BestPath> {
        let mut visiting = LinkedList::new();
        let mut touch_dest = false;
        self.setup(source);
        visiting.push_back(source);
        while !visiting.is_empty() {
            //for each node we are visiting, get it's current id, distance and arcs to other nodes
            let visitingid = visiting.pop_front().unwrap();
            let distance = self.verticies[visitingid as usize].best_distance;

            //let arcs = self.verticies[visitingid as usize].arcs.clone();
            //for each arc to other nodes, check if that path is the new best path to it
            //(from our visiting node)
            let mut i = 0;
            let l = self.verticies[visitingid as usize].arcs.len();
            while i < l {
                //arc in arcs {
                let id = self.verticies[visitingid as usize].arcs[i].to.clone();
                let dist = self.verticies[visitingid as usize].arcs[i].distance.clone();
                i += 1;
                let vertex = &mut self.verticies[id as usize];
                if id == visitingid || distance >= vertex.best_distance ||
                    distance + dist >= vertex.best_distance
                {
                    continue;
                }
                if id == destination {
                    touch_dest = true;
                }
                //if it is the new best node, make sure we update it's distance and best node, then
                //set it to be (re)visited later
                vertex.best_distance = distance + dist;
                vertex.best_verticie = visitingid;
                visiting.push_back(id);
            }
        }
        if !touch_dest {
            return None;
        }
        return Some(self.best_path(source, destination, touch_dest));
    }
}


fn main() {
    let mut g = Graph::new();
    for i in 0i64..10i64 {
        let mut v = Vertex::new(i as u64);
        for x in -2i64..3i64 {
            if x == 0 {
                continue;
            }
            if i + x >= 0 && i + x <= 9 {
                v.add_arc((i + x) as u64, 2i64 * ((x + 5i64) / 2i64));
            }
        }
        g.add_vertex(v);
    }
    print!("{:?}", g);
    print!("{:?}", g.shortest(0, 9));
}
