struct Fighter {
    stamina: i32,
    high_damage: i32,
    low_damage: i32,
}

enum Attack {
    HighKick(i32),
    LowKick(i32),
    Punch(i32),
    Hook(i32),
}

impl Attack {
    fn cary_out(self, target: &mut Fighter) {
        match self {
            Attack::HighKick(dmg) | Attack::Punch(dmg) | Attack::Hook(dmg) => {
                target.stamina -= dmg;
                target.high_damage += dmg;
            },
            Attack::LowKick(dmg) => {
                target.stamina -= dmg;
                target.low_damage += dmg;
            },
        }
    }
}

pub fn run() {
    let mut fighter = Fighter {
        stamina: 21,
        high_damage: 0,
        low_damage: 0
    };

    let atck1 = Attack::LowKick(3);
    atck1.cary_out(&mut fighter);

    let atck2 = Attack::Punch(2);
    atck2.cary_out(&mut fighter);

    let atck3 = Attack::Hook(8);
    atck3.cary_out(&mut fighter);

    let atck4 = Attack::HighKick(4);
    atck4.cary_out(&mut fighter);

    println!("Fighter got hight damage {} and damage low {}, his stamina {}", fighter.high_damage, fighter.low_damage, fighter.stamina);
}
