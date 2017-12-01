use std::fmt;

#[derive(Debug)]
pub struct Arc {
    pub to: u64,
    pub distance: i64,
}

pub struct Vertex {
    pub id: u64,
    pub best_distance: i64,
    pub best_verticie: u64,
    pub arcs: Vec<Arc>,
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
    pub fn add_arc(&mut self, to: u64, distance: i64) {
        self.arcs.push(Arc {
            to: to,
            distance: distance,
        });
    }

    pub fn remove_arc(&mut self, to: u64) {
        self.arcs.retain(|a| a.to != to);
    }

    pub fn new(id: u64) -> Vertex {
        Vertex {
            id: id,
            best_distance: 0,
            best_verticie: 0,
            arcs: Vec::new(),
        }
    }
}
