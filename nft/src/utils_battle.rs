use crate::state::{BattleArgs, Monster};

pub fn battle_round(monster: Monster, args: BattleArgs) -> u8 {
    if monster.hp >= args.hp {
        return 1;
    };
    return 0;

    // let mut me_hp = monster.hp.clone();
    // let mut enemy_hp = args.hp.clone();
    // let mut me_time = 10000;
    // let mut enemy_time = 10000;
    // let me_speed = monster.speed;
    // let enemy_speed = args.speed;
    //
    // let win;
    //
    // loop {
    //     let is_my_turn;
    //     if me_time / me_speed < enemy_time / enemy_speed {
    //         enemy_time = safe_sub(enemy_time, me_time / enemy_speed);
    //         is_my_turn = true;
    //     } else {
    //         me_time = safe_sub(me_time, enemy_time / me_speed);
    //         is_my_turn = false;
    //     }
    //
    //     if is_my_turn {
    //         let damage = apply_attack(monster.attack, args.defense);
    //         enemy_hp = safe_sub(enemy_hp, damage);
    //     } else {
    //         let damage = apply_attack(args.attack, monster.defense);
    //         me_hp = safe_sub(me_hp, damage);
    //     }
    //
    //     if me_hp <= 0 || enemy_hp <= 0 {
    //         win = me_hp > 0;
    //         break;
    //     }
    // }
    //
    // return win;
}

pub fn apply_attack(attack: u32, defense: u32) -> u32 {
    let damage: u32;

    if defense > attack * 3 {
        damage = attack * 10 / 100;
    } else if defense > attack * 2 {
        damage = attack * 10 / 100 + (attack * 3 - defense) * 5 / 100;
    } else if defense > attack {
        damage = attack * 15 / 100 + (attack * 2 - defense) * 5 / 100;
    } else {
        damage = attack - defense * 7 / 100;
    }

    return damage;
}

pub fn safe_sub(a: u32, b: u32) -> u32 {
    if a > b {
        return a - b;
    }
    return 0;
}