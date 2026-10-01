// Player
const BASE_DAMAGE: u8 = 20; // Axe = 25
const CRIT_CHANCE: u8 = 5;
const CRIT_MULTIPLIER: f32 = 0.5;
const BASE_ATTACK_SPEED: f32 = 0.7;
const DPS_PER_LVL: u8 = 6;

// Stage
const DURATION: u16 = 450;

// Enemies
const XP_PER_ENEMY: u8 = 10;

// 3 Ways of Spawn : Timer, Building, Swarm
// 1] Timer
const TIMER_TIME_DELTA: u8 = 7; // Every 5 - 10s
const SPAWN_NBR: u8 = 4; // 3 - 5

// 2] Buildings
const BUILDINGS: u16 = 2160;
const BUILDING_HP: u16 = 255;
const BUILDINGS_AOE: u8 = 3; // Expected Buildings hit by player AOE

const BUILDING_SPAWN_CHANCE: f32 = 0.33;
const BUILDING_SPAWN_NBR: u8 = 1;

// 3] Swarm
const SWARM_NBR: u8 = 5;
const SWARM_DURATION: u8 = 10;
const SWARM_TIME_DELTA: u8 = 2;
const SWARM_SPAWN_NBR: u8 = 10;

pub fn balance_calculation() {
    let total_xp = lvls_to_xp(50);
    println!("Total XP: {total_xp}");

    let total_lvl = xp_to_lvl(11400);
    println!("Total lvls: {total_lvl}");

    let total_dps = dps_calc();
    let start_dps = total_dps.0;
    let end_dps = total_dps.1;
    let average_dps = total_dps.2;
    println!("Start DPS : {start_dps}, End DPS : {end_dps}, Average_DPS : {average_dps}");
}

// Game Balancer
fn lvl_to_xp(lvl: u32) -> f64 {
    return 50_f64 * 1.1_f64.powf(lvl as f64);
}

pub fn lvls_to_xp(lvls: u32) -> u64 {
    let mut total_xp: f64 = 0.0;

    for i in 0..lvls {
        total_xp += lvl_to_xp(i);
    }

    return total_xp.round() as u64
}

pub fn xp_to_lvl(xp: u64) -> u32 {
    let mut xp_count: f64 = xp as f64;
    let mut total_lvls: u32 = 0;

    while xp_count > 0.0
    {
        xp_count -= lvl_to_xp(total_lvls);
        total_lvls += 1;
    }

    return total_lvls
}

pub fn total_enemies() -> u32 {
    let mut total: u32 = 0;

    // Timer
    total += (DURATION as u32 / TIMER_TIME_DELTA as u32) * SPAWN_NBR as u32;

    // Buildings
    total += (BUILDINGS as f32 * BUILDING_SPAWN_CHANCE * BUILDING_SPAWN_NBR as f32) as u32;

    // Swarm
    total += SWARM_NBR as u32 * (SWARM_DURATION as u32 / SWARM_TIME_DELTA as u32) * SWARM_SPAWN_NBR as u32;

    return total;
}

pub fn total_xp() -> u32 {
    let total_enemies= total_enemies();
    return total_enemies * XP_PER_ENEMY as u32;
}

pub fn level_reached() -> u32 {
    return xp_to_lvl(total_xp() as u64);
}

fn dps_increase() -> u32 {
    return DPS_PER_LVL as u32 ^ level_reached();
}

fn crit_dmg_multiplier(crit_chance: u8, crit_damage: f32) -> f32 {
    let crit_dps: f32 = 1_f32 + crit_chance as f32 * crit_damage as f32;
    return crit_dps
}

fn attack_speed_calc(base_attack_speed: f32, increase_attack_speed: u32) -> (f32, f32) {
    // Attack Speed Unit = Attack / Sec
    let final_attack_speed: f32 = base_attack_speed * increase_attack_speed as f32;
    let attack_time: f32 = 1_f32 / final_attack_speed;
    return (final_attack_speed, attack_time);
}

pub fn dps_calc() -> (f32, f32, f32) {
    let attack_speed = attack_speed_calc(BASE_ATTACK_SPEED, 1);
    let start_dps = BASE_DAMAGE as f32 * crit_dmg_multiplier(CRIT_CHANCE, CRIT_MULTIPLIER) * attack_speed.0;
    let end_dps = start_dps * dps_increase() as f32;
    let average_dps: f32 = (start_dps + end_dps) / 2_f32;

    return (start_dps, end_dps, average_dps)
}