use std::collections::{ HashMap, HashSet, VecDeque };
use super::*;
use super::Productions;


/// Alias type for a parsing table.
pub type ParseTable = Vec<HashMap<ElemT, Action>>;



#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Item {
    prod: usize,
    pos: usize
}
impl Item {
    fn new (prod: usize) -> Self {
        Item {
            prod,
            pos: 0
        }
    }

    fn inc (&self, prods: &Productions) -> Option<Self> {
        if self.pos < prods.v[self.prod].v.len() {
            Some(Item {
                pos: self.pos+1,
                ..(*self)
            })
        } else {
            None
        }
    }

    fn next (&self, prods: &Productions) -> Option<ElemT> {
        let p = &prods.v[self.prod];
        if self.pos == p.v.len() {
            None
        } else {
            Some(p.v[self.pos].clone())
        }
    }

    fn node (&self, prods: &Productions) -> NodeT {
        prods.v[self.prod].node.clone()
    }

    fn print(&self, prods: &Productions) {
        let p = &prods.v[self.prod]; 
        print!("{} -> ", p.node);
        for i in 0..p.v.len() {
            if self.pos == i {
                print!(".");
            }
            print!("{} ", p.v[i]);
        }
        if self.pos == p.v.len() { 
            println!(".");
        }
    }
}



impl Productions {
    fn into_node (&self, node: &NodeT) -> Vec<usize> {
        self.v
            .iter()
            .enumerate()
            .filter(|(_, p)| p.node == *node)
            .map(|(i, _)| i)
            .collect()
    }
}


type StateHash = Vec<(usize, usize)>;
#[derive(Debug, Clone)]
struct State {
    items: HashSet<Item>,
    edges: HashMap<ElemT, Action>
}
impl State {
    fn get_inheritances (&self, prods: &Productions, x: &ElemT) -> HashSet<Item> {
        self.items
            .iter()
            .filter(|i| { 
                let n = i.next(prods); 
                if n.is_some() {
                    n.unwrap() == *x 
                } else {
                    false
                }
            })
            .filter_map(|i| i.inc(prods))
            .collect()
    }
    fn make_hash (&self) -> StateHash {
        Self::to_hash(&self.items)
    }
    fn to_hash (items: &HashSet<Item>) -> StateHash {
        let mut v: Vec<_> = items
            .iter()
            .filter(|i| i.pos > 0)
            .map(|i| (i.prod, i.pos))
            .collect();
        v.sort();
        v
    }
    fn print (&self, prods: &Productions) {
        println!("[");
        for item in &self.items {
            print!("\t");
            item.print(prods);
            println!();
        }
        println!("]");
    }
}


type StateMap = HashMap<StateHash, usize>;
struct States {
    map: StateMap,
    v: Vec<State>
}
impl States {
    fn add (&mut self, state: State) -> usize {
        // Add to states
        self.v.push(state);
        let index = self.v.len()-1;

        // Add to Map
        let key = self.v[index].make_hash();
        self.map.insert(key, index);

        //println!("{}", self.v.len());
        index
    }

    fn get (&self, ni: &HashSet<Item>) -> Option<usize> {
        // Check in Map
        let key = State::to_hash(ni); 
        self.map.get(&key).cloned()
    }

    fn print (&self, prods: &Productions) {
        self.v.iter().enumerate().for_each(|(i, s)| {
            println!("{i}");
            s.print(prods);
        });
    }
}


/// Constructs Parsing table. Wraps "make_state(..)" and transfrom the state graph into a table. 
impl Productions {
    pub fn make_table(&self) -> Vec<HashMap<ElemT, Action>> {
        

        // Make State Machine
        let mut states = States {
            v: vec![],
            map: HashMap::new()
        };
        let mut init_items = HashSet::new();
        for i in 0..self.v.len() {
            let p = &self.v[i];
            if self.roots.contains(&p.node) {
                init_items.insert(Item::new(i));
            }
        }
    
        println!("Productions:\n{}", self);
        println!("making state machine...");
        make_state(self, &mut states, init_items);

        //println!("STATES:");
        //states.print(self);

        // Make Table
        let mut table: Vec<_> = states.v.into_iter().map(|s| s.edges).collect(); 

        // All roots go Shift to origin state.
        for root in &self.roots {
            table[0].insert(ElemT::Node(root.clone()), Action::Shift(0));
        }

        /*
        println!("PARSING TABLE:");
        println!("{}", visualizer::print_table(&table).unwrap());
        */

        table
    }
}


// Recursively creates state graph.
fn make_state (prods: &Productions, states: &mut States, inherits: HashSet<Item>) -> usize {

    //println!("{:?}", inherits);

    let mut s = State {
        edges: HashMap::new(),
        items: inherits
    };

    // Expand
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    for item in &s.items {
        if let Some(ElemT::Node(n)) = item.next(prods) {
            if !visited.contains(&n) {
                queue.push_back(n);
                visited.insert(n);
            }
        }
    }
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        for i in prods.into_node(&node) {
            let item = Item::new(i);
            if let Some(ElemT::Node(n)) = item.next(prods) {
                if !visited.contains(&n) {
                    queue.push_back(n);
                    visited.insert(n);
                }
            }
            s.items.insert(item);
        }
    }

    let index = states.add(s.clone());
    
    /*
    println!("iter:");
    s.print(prods);
    */
    

    let mut edges = HashMap::new();

    // Explore: make & recurse into edges
    for item in &s.items {
        if let Some(x) = item.next(prods) {
            let ni = s.get_inheritances(prods, &x);
            let ns = 
                if let Some(s) = states.get(&ni) { s }
                else { make_state(prods, states, ni) };

            //println!("add edge: {}, shift{}", x, ns);
            edges.insert(x, Action::Shift(ns));
        } else {
            if let Some(follows) = prods.follows.get(&item.node(&prods)) {
                for t in follows {
                    //println!("add edge: {}, reduce{}", t, item.prod);
                    edges.insert(t.clone(), Action::Reduce(item.prod));
                }
            } else {
                // The execution of this block means an item has been found that reduces to
                // something that isn't followed by anything.
                // The only case where this should happen is on the root node.
                assert!(prods.roots.contains(&item.node(prods)));
            }
        }
    }
    states.v[index].edges = edges;
    return index;
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn table_test () {
        let table = Productions::make_test().make_table();
        /*
           |a       |b       |A       |S       |
          0|S      1|S      2|S      4|        |
          1|S      1|S      2|S      3|        |
          2|R      1|R      1|R      1|        |
          3|R      2|R      2|R      2|        |
          4|S      1|S      2|S      5|        |
          5|        |        |        |        |
         */

        let s = if let Action::Shift(s) = table[0].get(&ElemT::Token(TokenT::b)).unwrap() {
            *s
        } else { panic!(); };
        let r = if let Action::Reduce(p) = table[s].get(&ElemT::Token(TokenT::a)).unwrap() {
            *p 
        } else { panic!(); };
        assert_eq!(r, 1);
        

        let s = if let Action::Shift(s) = table[0].get(&ElemT::Token(TokenT::a)).unwrap() {
            *s
        } else { panic!(); };
        let s = if let Action::Shift(s) = table[s].get(&ElemT::Node(NodeT::A)).unwrap() {
            *s
        } else { panic!(); };
        let r = if let Action::Reduce(p) = table[s].get(&ElemT::Token(TokenT::a)).unwrap() {
            *p 
        } else { panic!(); };
        assert_eq!(r, 2);
        assert!(table[s].get(&ElemT::Token(TokenT::K_LET)).is_none());


        let s = if let Action::Shift(s) = table[0].get(&ElemT::Node(NodeT::A)).unwrap() {
            *s
        } else { panic!(); };
        let s = if let Action::Shift(s) = table[s].get(&ElemT::Node(NodeT::A)).unwrap() {
            *s
        } else { panic!(); }; 
        assert_eq!(*table[s].get(&ElemT::Token(TokenT::EOF)).unwrap(), Action::Reduce(0));
    }
}
