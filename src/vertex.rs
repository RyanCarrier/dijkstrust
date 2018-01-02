use std::fmt;
use std::cmp::Ordering;


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Arc {
    pub to: u64,
    pub distance: i64,
}
#[derive(Clone, Eq, PartialEq)]
pub struct Vertex {
    pub id: u64,
    pub best_distance: i64,
    pub best_verticie: u64,
    pub arcs: Vec<Arc>,
}

impl Ord for Vertex {
    fn cmp(&self, other: &Vertex) -> Ordering {
        other.best_distance.cmp(&self.best_distance).then_with(|| {
            self.id.cmp(&other.id)
        })
    }
}
impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Vertex) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
impl Arc {
    pub fn export(&self) -> String {
        return format!("{},{}", self.to, self.distance);
    }
}

impl Vertex {
    pub fn new(id: u64) -> Vertex {
        Vertex {
            id: id,
            best_distance: 0,
            best_verticie: 0,
            arcs: Vec::new(),
        }
    }

    pub fn add_arc(&mut self, to: u64, distance: i64) {
        self.arcs.push(Arc {
            to: to,
            distance: distance,
        });
    }

    pub fn remove_arc(&mut self, to: u64) {
        self.arcs.retain(|a| a.to != to);
    }

    pub fn export(&self) -> String {
        return format!(
            "{} {}",
            self.id,
            (&self.arcs)
                .into_iter()
                .map(|a| a.export())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vertex::new(1234);
        let expected = Vertex {
            id: 1234,
            best_distance: 0,
            best_verticie: 0,
            arcs: Vec::new(),
        };
        assert_eq!(v, expected);
        let a = Vertex::new(1234);
        assert_eq!(v, a);
    }

    #[test]
    fn test_add_arc() {
        let mut a = Vertex::new(0);
        let mut b = Vertex::new(0);
        assert_eq!(a.arcs, b.arcs);
        a.add_arc(1, 2);
        assert!(a.arcs != b.arcs);
        b.add_arc(1, 2);
        assert_eq!(a, b);
        a.add_arc(3, 4);
        assert_eq!(
            a.arcs,
            [Arc { to: 1, distance: 2 }, Arc { to: 3, distance: 4 }]
        );
    }

    #[test]
    fn test_remove_arc() {
        let mut a = Vertex::new(0);
        let b = Vertex::new(0);
        assert_eq!(a.arcs, b.arcs);
        a.add_arc(1, 2);
        assert!(a.arcs != b.arcs);
        a.remove_arc(1);
        assert_eq!(a, b);

        a.add_arc(3, 4);
        a.add_arc(5, 6);
        a.add_arc(7, 8);
        a.remove_arc(5);
        assert_eq!(
            a.arcs,
            [Arc { to: 3, distance: 4 }, Arc { to: 7, distance: 8 }]
        );
    }
}
