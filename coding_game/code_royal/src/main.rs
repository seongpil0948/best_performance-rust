use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::rc::Rc;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

type C = i32; // common type
#[derive(Debug, Clone)]
struct Locate {
    x: C,
    y: C,
}

impl Locate {
    fn get_distance(&self, other: Rc<Locate>) -> C {
        let dist_y = (&self.y - other.y).pow(2);
        let dist_x = (&self.x - other.x).pow(2);
        let distance = ((dist_x + dist_y) as f32).sqrt().round() as C;
        distance
    }
}
#[derive(Debug)]
enum CreepType {
    KNIGHT,
    ARCHER,
}
#[derive(Debug, Clone)]
enum OwnerType {
    None,
    Friendly,
    Enemy,
}
impl OwnerType {
    fn update(&mut self, owner_type: C) -> () {
        *self = match owner_type {
            -1 => OwnerType::None, // No Structure
            0 => OwnerType::Friendly,
            1 => OwnerType::Enemy,
            _ => panic!(
                "Not Matched Site Type neither -1 or 2, \n input ->> {}",
                owner_type
            ),
        }
    }
}
#[derive(Debug, Clone)]
enum StructureType {
    None,
    Barrack,
}
impl StructureType {
    fn update(&mut self, structure_type: C) -> () {
        *self = match structure_type {
            -1 => StructureType::None, // No Structure
            2 => StructureType::Barrack,
            _ => panic!(
                "Not Matched Site Type neither -1 or 2, \n input ->> {}",
                structure_type
            ),
        }
    }
}
#[derive(Debug, Clone)]
struct Site {
    id: C,
    radius: C,
    locate: Rc<Locate>,
    structure: StructureType,
    owner: OwnerType,
}

impl Site {
    fn new(id: C, radius: C, locate: Locate) -> Site {
        Site {
            id,
            radius,
            locate: Rc::new(locate),
            structure: StructureType::None,
            owner: OwnerType::None,
        }
    }
    fn update(mut self, structure_type: C, owner_type: C) -> () {
        self.structure.update(structure_type);
        self.owner.update(owner_type)
    }
}
// First Print
enum First {
    Wait,
    Moving,
    Build,
}

// Second Print
enum Second {
    Train,
}

impl First {
    fn printing(
        &self,
        site: Option<Site>,
        site_id: Option<C>,
        creep_type: Option<CreepType>,
    ) -> () {
        match self {
            First::Wait => println!("WAIT"),
            First::Moving => match site {
                Some(site) => println!("MOVE {} {}", site.locate.x, site.locate.y),
                None => panic!("required argument Locate in Output::First::Moving "),
            },
            First::Build => match site_id {
                Some(site_id) => match creep_type {
                    Some(creep_type) => println!("BUILD {:?} BARRACKS-{:?}", site_id, creep_type),
                    None => panic!("required argument creep_type in Output::First::Build "),
                },
                None => panic!("required argument site_id in Output::First::Build "),
            },
        }
    }
}

impl Second {
    fn printing(&self, site_id: Option<C>) {
        let mut output = String::from("TRAIN");
        match site_id {
            Some(id) => {
                let output = output.push_str(&format!(" {}", id));
                println!("{:?}", output)
            }
            None => println!("{}", &output),
        }
    }
}
#[derive(Debug, Clone)]
struct Queen {
    hp: C,
    gold: C,
    touched_site: Option<C>,
    locate: Locate,
}

fn find_shortest_site(queen: Queen, sites: Rc<RefCell<HashMap<C, Site>>>) -> C {
    let sites = sites.borrow();
    let mut shortest_path: C = 2000;
    let mut shortest_site_id: C = -1;
    let q_locate = queen.locate;
    for site in sites.values() {
        let new_dist = q_locate.get_distance(Rc::clone(&site.locate));
        if new_dist < shortest_path {
            shortest_path = new_dist;
            shortest_site_id = site.id;
        }
    }
    shortest_site_id
}
// &mut 
fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");
    const _MAX_RIGHT: C = 1920;
    const _MAX_HEIGHT: C = 1000;
    let sites = Rc::new(RefCell::new(HashMap::new()));
    let mut first_output = First::Wait;
    let second_output = Second::Train;

    let queen = Queen {
        hp: 100,
        gold: 100,
        touched_site: None,
        locate: Locate { x: 150, y: 150 },
    };

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let num_sites = parse_input!(input_line, i32);
    {
        let mut mut_sites = sites.borrow_mut();
        for i in 0..num_sites as usize {
            let mut input_line = String::new(); io::stdin().read_line(&mut input_line).unwrap(); let inputs = input_line.split(" ").collect::<Vec<_>>();
            let site_id = parse_input!(inputs[0], i32); let x = parse_input!(inputs[1], i32); let y = parse_input!(inputs[2], i32);
            let radius = parse_input!(inputs[3], i32);
            mut_sites.insert(site_id, Site::new(site_id, radius, Locate { x: x, y: y }));
        }
    }
    // game loop
    loop {
        let mut input_line = String::new(); io::stdin().read_line(&mut input_line).unwrap(); let inputs = input_line.split(" ").collect::<Vec<_>>();
        let gold = parse_input!(inputs[0], i32);
        let touched_site = parse_input!(inputs[1], i32); // -1 if none
        let mut queen = queen.clone();
        queen.gold = parse_input!(inputs[0], i32);
        if touched_site > 0 {
            queen.touched_site = Some(touched_site);
        } else {
            queen.touched_site = None
        }
        for i in 0..num_sites as usize {
            let mut input_line = String::new(); io::stdin().read_line(&mut input_line).unwrap(); let inputs = input_line.split(" ").collect::<Vec<_>>();
            let site_id = parse_input!(inputs[0], i32); let structure_type = parse_input!(inputs[3], i32); // -1 = No structure, 2 = Barracks
            let owner_type = parse_input!(inputs[4], i32); // -1 = No structure, 0 = Friendly, 1 = Enemy
            let sites = sites.borrow();
            let site = sites.get(&site_id).unwrap();
            site.clone().update(structure_type, owner_type)
        }
        let mut input_line = String::new(); io::stdin().read_line(&mut input_line).unwrap(); let num_units = parse_input!(input_line, i32);
        for i in 0..num_units as usize {
            let mut input_line = String::new(); io::stdin().read_line(&mut input_line).unwrap(); let inputs = input_line.split(" ").collect::<Vec<_>>();
            let x = parse_input!(inputs[0], i32); let y = parse_input!(inputs[1], i32); let owner = parse_input!(inputs[2], i32);
            let unit_type = parse_input!(inputs[3], i32); // -1 = QUEEN, 0 = KNIGHT, 1 = ARCHER let health = parse_input!(inputs[4], i32);
        }
        let site2 = Rc::clone(&sites);
        let shortest_site_id = find_shortest_site(queen, site2);
        let sites = sites.borrow();
        let shortest_site = match sites.get(&shortest_site_id) {
            Some(site) => {
                first_output = First::Moving;
                site
            }
            None => panic!("not Found Shortest Site's Locate"),
        };

        first_output.printing(Some(shortest_site.clone()), None, None);
        second_output.printing(None)
    }
}
