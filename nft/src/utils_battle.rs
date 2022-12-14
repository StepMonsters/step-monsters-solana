use crate::state::{BattleArgs, Monster};

pub fn battle_round(monster: Monster, args: BattleArgs) -> (u8, Vec<u32>) {
    let mut me_hp = monster.hp.clone();
    let mut enemy_hp = args.hp.clone();
    let me_attack = monster.attack.clone();
    let enemy_attack = args.attack.clone();
    let me_defense = monster.defense.clone();
    let enemy_defense = args.defense.clone();
    let me_speed = monster.speed.clone();
    let enemy_speed = args.speed.clone();
    let mut me_distance = 1000;
    let mut enemy_distance = 1000;

    let mut arr = Vec::new();

    for _i in 0..20 {
        if me_distance / me_speed <= enemy_distance / enemy_speed {
            enemy_distance -= (me_distance / me_speed) * enemy_speed;
            me_distance = 1000;
            let damage = apply_attack(me_attack, enemy_defense);
            enemy_hp = safe_sub(enemy_hp, damage);
            arr.push(enemy_hp * 10 + 1);
            if enemy_hp <= 0 {
                break;
            }
        } else {
            me_distance -= (enemy_distance / enemy_speed) * me_speed;
            enemy_distance = 1000;
            let damage = apply_attack(enemy_attack, me_defense);
            me_hp = safe_sub(me_hp, damage);
            arr.push(me_hp * 10 + 2);
            if me_hp <= 0 {
                break;
            }
        }
    }

    let len = 20 * 2 - arr.len().clone();
    for _i in 0..len {
        arr.push(0);
    }

    let mut win = 0;
    if me_hp > enemy_hp {
        win = 1;
    }
    return (win, arr);
}

pub fn apply_attack(attack: u32, defense: u32) -> u32 {
    let damage: u32;

    if attack > defense {
        damage = attack * (attack / (attack + defense)) * 1600 / 1000 * (defense / (attack + defense));
    } else {
        damage = attack * (attack / (attack + defense)) * 1600 / 1000 * (defense / (attack + defense));
    }

    return damage;
}

pub fn safe_sub(a: u32, b: u32) -> u32 {
    if a > b {
        return a - b;
    }
    return 0;
}