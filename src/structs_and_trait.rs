trait Weapon {
    fn attack(&self);
    fn name(&self) -> &'static str;
}

struct Stick {
    name: &'static str,
    damage: u32,
}

impl Stick {
    fn new() -> Stick {
        Stick { name: "Wood stick", damage: 2 }
    }
}

impl Weapon for Stick {
    fn attack(&self) {
        println!("[ATTACK] {} whistles and cuts the air. Air get damage: {}", self.name, self.damage);
    }

    fn name(&self) -> &'static str {
        self.name
    }
}

struct Character<'weapon> {
    name: &'static str,
    weapon: &'weapon Weapon,
}

pub fn run() {
    let chr = Character {
        name: "Di",
        weapon: &Stick::new(),
    };

    println!("We have a character, his name {}", chr.name);
    println!("and he armed with {}", chr.weapon.name());
    chr.weapon.attack();
    chr.weapon.attack();
}
