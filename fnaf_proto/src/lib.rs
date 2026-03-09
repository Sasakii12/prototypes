#[derive(PartialEq, Copy, Clone, Default, Debug)]
enum Pizzaria {
    #[default]
    Stage,
    LeftHallway,
    RightHallway,
    AtLeftDoor,
    AtRightDoor
}

#[derive(Default)]
struct Animatronic {
    name: String,
    path: Vec<Pizzaria>,
    location: Pizzaria,
    interval: u32,
    ai_level: u32
}

struct Office {
    left: bool,
    right: bool,
}

impl Animatronic {
    fn advance(&mut self) {
        if self.location == Pizzaria::AtLeftDoor || self.location == Pizzaria::AtRightDoor {
            self.location = self.path[0]
        }
        let location = self.path.iter().position(|&r| r == self.location).unwrap();
        self.location = self.path[location + 1]
    }

    fn show_location(&self) -> Pizzaria {
        self.location
    }
}

impl Office {
    fn door_status(&mut self) {
        if self.left == true {
            println!("Left door: open")
        } else {
            println!("Left door: closed")
        }
        if self.right == true {
            println!("Right door: open")
        } else {
            println!("Right door: closed")
        }
        
    }

    fn toggle_left_door(&mut self) {
        if self.left == true {
            println!("Opening left door");
            self.left = false
        } else {
            println!("Closing left door");
            self.left = true
        }
    }

    fn toggle_right_door(&mut self) {
        if self.right == true {
            println!("Opening right door");
            self.right = false
        } else {
            println!("Closing right door");
            self.left = true
        }
    }
}

fn initialize() -> Vec<Animatronic> {
    vec![Animatronic {
    name: String::from("Bonnie"), 
    path: vec![Pizzaria::Stage, Pizzaria::LeftHallway, Pizzaria::AtLeftDoor],
    location: Pizzaria::Stage,
    interval: 2,
    ai_level: 1
    }]
}

fn input() -> String {
    let mut buf = String::new();

    std::io::stdin().read_line(&mut buf).unwrap();

    buf.trim().to_string()
}


fn check_cam(anim: &Vec<Animatronic>) {
    let locations = anim.iter().map(|x| (x.name.clone(),x.show_location())).collect::<Vec<(String, Pizzaria)>>();
    println!("Which cam would you like to view?");
    println!("[1] Stage");
    println!("{:?}", locations)
}

fn toggle_doors(office: &mut Office) {
    office.door_status();
    println!("Which door would you like to toggle?\n [1] Left door\n[2] Right door");
    match input().as_str() {
        "1" => office.toggle_left_door(),
        "2" => office.toggle_right_door(),
        _ => todo!()
    }
}

pub fn game_loop() {
    let anim = initialize();
    let mut office = Office {left: false, right: false};
    loop {
        println!("What would you like to do?");
        println!("[1] Check cams\n[2] Toggle doors");
        let action = input();
        match action.as_str() {
            "1" => check_cam(&anim),
            "2" => toggle_doors(&mut office),
            _ => continue
        }
    }
    
}