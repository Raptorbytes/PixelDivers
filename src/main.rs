/* Imports */
use bevy; /* Importing game engine */

/* Functions and the actual program */
fn logo() { /* This is the function for printing the PixelDivers banner in Slant (Small) ASCII font */
    println!("   ___  _          _____  _                 ");
    println!(r"  / _ \(_)_ _____ / / _ \(_)  _____ _______");
    println!(r" / ___/ /\ \ / -_) / // / / |/ / -_) __(_-<");
    println!(r"/_/  /_//_\_\\__/_/____/_/|___/\__/_/ /___/");
}

fn stratagem_codes() { /* Listing all stratagem codes (Arrows but with WASD) */
    let orbital_gatling = "dsaww"; /* Orbital gatling barrage */
    let orbital_walking = "dsdsds"; /* Orbital walking barrage */
    let orbital_120 = "ddsads"; /* Orbital 120MM HE barrage */
    let orbital_380 = "dswwass"; /* Orbital 380MM HE barrage */
    let orbital_gas = "ddsd"; /* Orbital gas strike */
    let orbital_ems = "ddas"; /* Orbital EMS strike */
    let orbital_smoke = "ddsw"; /* Orbital smoke strike */
    let orbital_airburst = "ddd"; /* Orbital airburst strike */
    let orbital_precision = "ddw"; /* Orbital precision strike */
    let orbital_railcannon = "dwssd"; /* Orbital railcannon strike */
    let orbital_laser = "dswds"; /* Orbital laser strike */
    let eagle_strafe = "wdd"; /* Eagle strafing run */
    let eagle_napalm = "wdsw"; /* Eagle napalm airstrike */
    let eagle_110_rocket = "wdwa"; /* Eagle 110MM rocket pods */
    let eagle_airstrike = "wdsd"; /* Eagle airstrike */
    let eagle_smoke = "wdws"; /* Eagle smoke strike */
    let eagle_cluster = "wdssd"; /* Eagle cluster bomb */
    let eagle_500 = "wdsss"; /* Eagle 500KG bomb */
    let grenade_launcher = "sawas"; /* Grenade launcher */
    let expend_anti_tank = "ssawd"; /* Expendable anti-tank */
    let recoilless = "sadda"; /* Recoilless rifle */
    let spear = "sswss"; /* Spear */
    let railgun = "sdswad"; /* Railgun */
    let anti_material_rifle = "sadws"; /* Anti-material rifle */
    let autocannon = "saswwd"; /* Autocannon */
    let laser_cannon = "saswa"; /* Laser cannon */
    let quasar_cannon = "sswad"; /* Quasar cannon */
    let arc_thrower = "sdswaa"; /* Arc thrower */
    let flamethrower = "sawsw"; /* Flamethrower */
    let mortar_sentry = "swdds"; /* Mortar sentry */
    let ems_sentry = "swdsd"; /* EMS mortar sentry */
    let anti_personnel_mines = "sawd"; /* Anti personnel mines */
    let incendiary_mines = "saas"; /* Incendiary mines */
    let machine_gun = "saswd"; /* Machine gun */
    let stalwart = "saswwa"; /* Stalwart */
    let heavy_machine_gun = "sawss"; /* Heavy machine gun */
    let hmg_emplacement = "swadda"; /* HMG emplacement */
    let machine_gun_sentry = "swddw"; /* Machine gun sentry */
    let gatling_sentry = "swda"; /* Gatling sentry */
    let autocannon_sentry = "swdwaw"; /* Autocannon sentry */
    let rocket_sentry = "swdda"; /* Rocket sentry */
    let tesla_tower = "swdwad"; /* Tesla tower */
    let shield_generator = "ssadad"; /* Shield generator */
    let patriot_exosuit = "asdwass"; /* Patriot exosuit */
    let resupply = "sswd"; /* Resupply */
/*
     let emancipator_exosuit = "";
*/
}

/*
fn INSERT STRATAGEM HERE() {
    /* Add the stratagems function */
}
*/

fn start() { /* Running the "logo" function and setting up persistance */
    logo();

    /* Add save loading and setup code */
}