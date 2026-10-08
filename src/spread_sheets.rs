use icu_locale_core::locale;
use spreadsheet_ods::{
    OdsResult, Sheet, Value, WorkBook,
    color::Rgb,
    format, formula, mm,
    style::CellStyle,
    style::units::{Border, Length, TextRelief},
};
use std::{fs, path::Path};

#[derive(Clone)]
enum Attribute {
    Strength,
    Spirit,
    Intelligence,
    Cunning,
    Dexterity,
    Motricity,
}

impl Attribute {
    fn get_next(&self) -> Self {
        return match self {
            Attribute::Strength => Attribute::Spirit,
            Attribute::Spirit => Attribute::Intelligence,
            Attribute::Intelligence => Attribute::Cunning,
            Attribute::Cunning => Attribute::Dexterity,
            Attribute::Dexterity => Attribute::Motricity,
            Attribute::Motricity => Attribute::Strength,
        };
    }
}

#[derive(Clone, PartialEq)]
enum Category {
    Mix,
    Masc,
    Fem,
}

impl Category {
    fn get_next(&self) -> Self {
        return match self {
            Category::Mix => Category::Masc,
            Category::Masc => Category::Fem,
            Category::Fem => Category::Mix,
        };
    }
}

/// print a Vec<String>
fn print_vec(strings: &Vec<String>) {
    for i in 0..strings.len() {
        println!("{} : {}", i, strings[i])
    }
}

/* #region SHEETS */

pub fn create_spread_sheet_test() {
    fs::create_dir_all("test_out").expect("create_dir");
    let path = std::path::Path::new("test_out/lib_example.ods");

    let mut wb = create_or_get_workbook(path).expect("");

    // Create Sheet1
    if wb.num_sheets() == 0 {
        let mut sheet = Sheet::new("one");
        sheet.set_value(0, 0, true);
        wb.push_sheet(sheet);
    }

    // Read Sheet1
    let sheet = wb.sheet(0);
    let _n = sheet.value(0, 0).as_f64_or(0f64);
    if let Value::Boolean(v) = sheet.value(1, 1) {
        if *v {
            println!("was true");
        }
    }

    // Create Sheet 2
    if wb.num_sheets() == 1 {
        wb.push_sheet(Sheet::new("two"));
    }

    // Write to File
    spreadsheet_ods::write_ods(&mut wb, "test_out/lib_example.ods").expect("write_ods")
}

pub fn create_or_get_workbook<P: AsRef<Path>>(path: P) -> Result<WorkBook, String> {
    let wb = if path.as_ref().exists() {
        spreadsheet_ods::read_ods(path).unwrap()
    } else {
        WorkBook::new(locale!("en-US"))
    };

    return Ok(wb);
}

fn clear_sheets(wb: &mut WorkBook) {
    for i in (0..wb.num_sheets()).rev() {
        // println!("Remove Sheet at : {}", i);
        wb.remove_sheet(i);
    }
}

/// Try to Create a sheet in a WorkBook at Index
fn create_sheet<'a>(
    wb: &'a mut WorkBook,
    sheet_name: &str,
    index: usize,
) -> Result<&'a mut Sheet, String> {
    if wb.num_sheets() <= index {
        let sheet = Sheet::new(sheet_name);
        wb.push_sheet(sheet);
        return Ok(wb.sheet_mut(index));
    } else {
        return Err(format!(
            "Failed to Create Sheet : {}, at Index {}, current number of sheets = {}",
            sheet_name,
            index,
            wb.num_sheets()
        ));
    }
}

/// Fill a row with Strings
fn fill_row(sheet: &mut Sheet, row_id: u32, strings: &mut Vec<String>) {
    let str_count = strings.len().clone();

    for i in 0..str_count {
        let value = strings.remove(0);
        // println!("Set value : {} at x = {}, y = {}", value, row_id, i as u32);
        sheet.set_value(row_id, i as u32, value);
    }
}

fn fill_row_offset(sheet: &mut Sheet, row_id: u32, offset: u32, strings: &mut Vec<String>) {
    for i in 0..strings.len() {
        sheet.set_value(row_id, i as u32 + offset, strings.remove(0));
    }
}

/// Fill a column with Strings
fn fill_column(sheet: &mut Sheet, column_id: u32, strings: &mut Vec<String>) {
    for i in 0..strings.len() {
        sheet.set_value(i as u32, column_id, strings.remove(0));
    }
}

fn fill_column_offset(sheet: &mut Sheet, column_id: u32, offset: u32, strings: &mut Vec<String>) {
    for i in 0..strings.len() {
        sheet.set_value(i as u32 + offset, column_id, strings.remove(0));
    }
}

/* #endregion */

// DyNG Spread Sheet Filler
// Converte Authoring Data from Obsidian to .ods

/* #region AUTHORING DATA */

const FACTIONS: [&str; 10] = [
    "Knights",
    "Vikings",
    "Dwarfs",
    "Elves",
    "Samurai",
    "Barbarians",
    "Zorks",
    "Phraraons",
    "Dinomancers",
    "Demons",
];

const CULTURE_COMPONENTS: [&str; 11] = [
    "European", "Nordic", "Russian", "Dwarven", "Tribal", "Aztek", "Egyptian", "Asian", "Elven",
    "Demonic", "Goblin",
];

const CATEGORIES: [&str; 3] = ["Mix", "Masc", "Fem"];

const NAME_TYPES: [&str; 6] = [
    "Adjective",
    "Name",
    "Equipment",
    "Bodypart",
    "Creature",
    "Role",
];

const ATTRIBUTES: [&str; 6] = ["STR", "SPI", "INT", "CUN", "DEX", "MOT"];

/* #endregion */

/* #region FACTIONS DATA */

const KNIGHTS_FAMILIES: &str = "-Tilbury
-Wade
-Tregor
-Placidius
-Kihei
-Sceptile
-Retsen
-Talowi
-Archandiel
-Kessada
-Malori
-Victus
-Thayer
-Appolloosa
-Laughton
-Raby
-Kiel
-Mbari
-Andurs
-Hagel
-Sayar
-Vergon
-Cahine
-Guedon
-Lagar
-Barner
-Lamy
-Pax
-Madox
-Fender
-Smallwood
-Vaderius
-Gonsalve
-Thomasius
-Delso
-Teissen
-Landor
-Mauri
-Moribund
-Goatsi
-Crendor
-Wistar
-Cubeka
-Baros
-Morical
-Gabbit
-Herbert
-Burada
-Kallister
-Bernstein
-Morden";

const NORDIC_FAMILIES: &str = "-Voren
-Wolder
";

const DWARF_FAMILIES: &str = "-Mammon
-Higley
-Ansbach
-Hardon
-Ramos
-Fink
-Tuired
-Thorpe
-Phips
-Tin
-Gertin
-Wallis
-Wasser
-Mardum
-Karabec
-Roklas";

const FAMILY_NAMES: [&str; 3] = [KNIGHTS_FAMILIES, NORDIC_FAMILIES, DWARF_FAMILIES];

/* #endregion */

/* #region COMPONENTS DATA */

const NORDIC_STR: [&str; 3] = [
    "-Vyrkol
-Howitir
-Idwyr
-Hilmar
-Hrovitnir",
    "-Bjohrn
-Ragnvald
-Ulthor
-Olaf",
    "-Jarla",
];

const NORDIC_SPI: [&str; 3] = [
    "-Eradan",
    "-Ymladd
-Lothrik
-Unduradh",
    "",
];

const NORDIC_INT: [&str; 3] = ["-Yhorm", "-Einar", ""];

const NORDIC_CUN: [&str; 3] = [
    "-Lokat
-Leiden",
    "-Zigomar",
    "-Shanar",
];

const NORDIC_DEX: [&str; 3] = [
    "-Ivyr
-Frol",
    "-Rarick
-Holten",
    "-Valla",
];

const NORDIC_MOT: [&str; 3] = [
    "-Hyreim
-Rakvar
-Sjaard
-Rekvam
-Roark",
    "-Vugnar
-Harald",
    "-Yigit",
];

const NORDIC_NAMES: [[&str; 3]; 6] = [
    NORDIC_STR, NORDIC_SPI, NORDIC_INT, NORDIC_CUN, NORDIC_DEX, NORDIC_MOT,
];

/* #endregion */

/* #region ATTRIBUTES DATA */

/* #region STR NAMES */

const STR_ADJECTIVE: &str = "-Big
-Ugly
-Smelly
-Stinky
-Grotesque
-Cracking
-Molten
-Steady
-Strong
-Brave
-Stubborn
-Cruel
-Armored
-Long
-Large
-Fractured
-Giant
-Hearthbound
-Sundered
-Titanic
-Drooling
-Blazing
-Ashen";

const STR_NAME: &str = "-Iron
-Ash
-Metal
-Mountain
-Rock
-Stone
-Tree
-Armor
-Cinder
-Bastion
-Hunger
-Rage
-Battle
-War
-Ham
-Fracture
-Anvil
-Vitality
-Forge
-Juggernaut
-Stench
-Bane";

const STR_EQUIPMENT: &str = "-Axe
-Helmet
-Warhammer
-Belt
-Ruby
-Cleaver
-Plate
-Gauntlet
-Bulwark
-Anchor";

const STR_BODYPART: &str = "-Arms
-Belly
-Teeth
-Shoulder
-Jaw
-Skin
-Fist
-Beard
-Skull
-Hearth
-Lungs";

const STR_CREATURE: &str = "-Bear
-Bull
-Leviathan
-Golem
-Dragon";

const STR_ROLE: &str = "-Eater
-Breaker
-Destroyer
-Smith
-Crusher
-Snapper
-Glutton
-Butcher
-Licker
-Chewer
-Guardian
-Drinker
-Goliath
-Warden
-Bastion
-Knight
-Sunderer
-Devourer
-Keeper
-Walker
-Shattering
-Flayer";

/* #endregion */

/* #region SPI NAMES */

const SPI_ADJECTIVE: &str = "-Pure
-Eternal
-Fanatical
-Ambitious
-Fiendish
-Forbidden
-Oblivion
-Zealous
-Abyssal
-Endless
-Hollow
-Immortal
-Imperial
-Unending
-Demonic
-Divine
-Innervating
-Desecrated
-Seven";

const SPI_NAME: &str = "-Pain
-Storm
-Oath
-Soul
-Will
-Eternity
-Flame
-Chain
-Hex
-Steel
-Curse
-Drive
-Crypt
-Dawn
-Radiance
-Mandate
-Bane
-Blessing
-Omen
-Redemption
-Rite
-Embrace
-Sky
-Sun
-Trinity
-Demon
-Miracle
-Virtue
-Rune
-Ascension
-Aspect
-Salvation
-Requiem";

const SPI_EQUIPMENT: &str = "-Scepter
-Maul
-Coffin
-Pickaxe
-Shovel
-Armguard
-Tomb stone
-Sigil
-Flail
-Torch
-Locket
-Aegis
-Crown
-Talisman
-Mallet
-Lantern";

const SPI_BODYPART: &str = "-Blood
-Arm
-Hand
-Flesh";

const SPI_CREATURE: &str = "-Stag
-Dragon
-Gargoyle";

const SPI_ROLE: &str = "-Elder
-Bringer
-Avenger
-Keeper
-Believer
-Purifier
-Angel
-Lord
-Carver
-Martyr
-Oracle";

/* #endregion */

/* #region INT NAMES */

const INT_ADJECTIVE: &str = "-Chill
-Ancient
-Amplifying
-Lucid
-Blasting
-Glowing
-Crystalline
-Fated
-Glacial
-Cosmic
-Frozen
-Flowing
-Arch
-One";

const INT_NAME: &str = "-Desolation
-Void
-faerie
-Aether
-Wisp
-Chapter
-Barrier
-Blackfire
-Echo
-Essence
-Horizon
-Focus
-Rift
-Water
-Convergence
-Field
-Power
-Black-hole
-Reality
-Reverberation
-Arcane
-Nether
-Remnant
-Singularity
-Portal
-Paradox";

const INT_EQUIPMENT: &str = "-Orb
-Tome
-Mantle
-Catalyst
-Codex
-Idol
-Crystal
-Sash
-Staff
-Diadem
-Rod
-Circlet
-Hourglass
-Quill
-Parchment
-Scroll";

const INT_BODYPART: &str = "-Brain
-Spine
-Head";

const INT_CREATURE: &str = "-Howl
-Wyvern
-Fiend";

const INT_ROLE: &str = "-Advisor
-Seeker
-Alternator
-Banshee
-Lich
-Pyromancer
-Absorber";

/* #endregion */

/* #region CUN NAMES */

const CUN_ADJECTIVE: &str = "-Accursed
-Rotten
-Death
-Plagued
-Clever
-Heartless
-Diseased
-Small
-Dark
-Monstruous
-Deadly
-Blighting
-Haunting
-Lost
-Black
-Ruined
-Profane
-Umbral
-Sinister
-Slithering";

const CUN_NAME: &str = "-Cave
-Death
-Thread
-Mirror
-Dream
-Dusk
-Torment
-Grudge
-Shadow
-Moon
-Despair
-Scarcrow
-Promise
-Mirage
-Person
-Pain";

const CUN_EQUIPMENT: &str = "-Blade
-Dagger
-Guise
-Disguise
-Mask
-Censer";

const CUN_BODYPART: &str = "-Ears
-Bone
-Corpse
-Scar
-Maw
-Nerve
-Pancreas
-Tongue";

const CUN_CREATURE: &str = "-Rat
-Spider";

const CUN_ROLE: &str = "-Bag
-Artist
-Dweller
-Assassin
-Whisperer
-Spectre
-Puppeteer
-Reaper
-Being";

/* #endregion */

/* #region DEX NAMES */

const DEX_ADJECTIVE: &str = "-Elusive
-Fast
-Swift
-Bramble
-Quick
-Verdant
-Winged
-Free
-Light
-Two";

const DEX_NAME: &str = "-Wild
-Silver
-Nature
-Stride
-Haste
-Spirit
-Silk
-Moon";

const DEX_EQUIPMENT: &str = "-Bow
-Arrow
-Vest
-Buckler
-Slingshot
-Sling
-Dirk
-Razor
-Hood";

const DEX_BODYPART: &str = "-Nose
-Finger
-Feets
-Eye
-Ears
-Talon
-Fang
-Claw";

const DEX_CREATURE: &str = "-Beast
-Hawk
-Scorpion
-Serpent";

const DEX_ROLE: &str = "-Hunter
-Smuggler
-Tracker
-Pathfinder
-Slinger
-Dancer
-Prowler
-Stalker
-Savage";

/* #endregion */

/* #region MOT NAMES */

const MOT_ADJECTIVE: &str = "-Beautiful
-Beloved
-Merciful
-Great
-Handsome
-Lucky
-Crimson
-Agile
-Serrated
-Ravenous
-Blossoming
-Dire";

const MOT_NAME: &str = "-Rabble
-Raid
-Hull
-Hurricane
-Dice";

const MOT_EQUIPMENT: &str = "-Sword
-Boots
-Gloves
-Jewel
-Bracer
-Cloak
-Mail
-Bolts
-Scimitar
-Harness
-Spear
-Cutlass
-Glaive
-Machete
-Muzzle
-Whip";

const MOT_BODYPART: &str = "-Legs
-Hair
-Hands
-Tusks
-Fur";

const MOT_CREATURE: &str = "-Hound
-Kraken
-Hydra";

const MOT_ROLE: &str = "-Delver
-Slayer
-Tamer
-Skewer
-Savior
-Killer
-Ravager
-Rouser
-Leader
-Render
-Executioner
-Reaver
-Collector
-Raiser
-Gambler
-Companion
-Impaler
-Brawler";

/* #endregion */

/* #region NAMES */

const STR_TITLES: [&str; 6] = [
    STR_ADJECTIVE,
    STR_NAME,
    STR_EQUIPMENT,
    STR_BODYPART,
    STR_CREATURE,
    STR_ROLE,
];

const SPI_TITLES: [&str; 6] = [
    SPI_ADJECTIVE,
    SPI_NAME,
    SPI_EQUIPMENT,
    SPI_BODYPART,
    SPI_CREATURE,
    SPI_ROLE,
];

const INT_TITLES: [&str; 6] = [
    INT_ADJECTIVE,
    INT_NAME,
    INT_EQUIPMENT,
    INT_BODYPART,
    INT_CREATURE,
    INT_ROLE,
];

const CUN_TITLES: [&str; 6] = [
    CUN_ADJECTIVE,
    CUN_NAME,
    CUN_EQUIPMENT,
    CUN_BODYPART,
    CUN_CREATURE,
    CUN_ROLE,
];

const DEX_TITLES: [&str; 6] = [
    DEX_ADJECTIVE,
    DEX_NAME,
    DEX_EQUIPMENT,
    DEX_BODYPART,
    DEX_CREATURE,
    DEX_ROLE,
];

const MOT_TITLES: [&str; 6] = [
    MOT_ADJECTIVE,
    MOT_NAME,
    MOT_EQUIPMENT,
    MOT_BODYPART,
    MOT_CREATURE,
    MOT_ROLE,
];

const TITLES: [[&str; 6]; 6] = [
    STR_TITLES, SPI_TITLES, INT_TITLES, CUN_TITLES, DEX_TITLES, MOT_TITLES,
];

/* #endregion */

/* #endregion */

/// Take 2 Vec<&str>
/// Each values in vec1 is assigned each value from vec2
/// -> vec1[i] + "_" + vec2[j]
fn assign_strings(vec1: Vec<&str>, vec2: Vec<&str>) -> Vec<String> {
    let mut vec3 = vec![];

    for i in 0..vec1.len() {
        for j in 0..vec2.len() {
            let value = vec1[i].to_owned() + "_" + vec2[j];
            vec3.push(value);
        }
    }

    return vec3;
}

/// Take a string of : -name1 -name2 -...
/// Return Vec<String> of each name
pub fn sort_names(string: String) -> Vec<String> {
    // println!("---Sorting Names");
    let mut names = vec![];

    let mut current_name = String::from("");
    let mut record = false;

    for character in string.chars() {
        if record {
            current_name.push(character);
        }

        if character == '-' {
            record = true;
        }

        if character == '\n' {
            record = false;
            current_name = current_name.trim().to_string();
            // println!("Name = {}", current_name);
            names.push(current_name.clone());
            current_name = String::from("");
        }
    }

    current_name = current_name.trim().to_string();
    // println!("Name = {}", current_name);
    names.push(current_name.clone());

    return names;
}

pub fn create_factions_workbook() {
    println!("---Create Factions Workbook");
    let path = std::path::Path::new("test_out/dyng_fations.ods");
    let mut wb = create_or_get_workbook(path).unwrap();

    clear_sheets(&mut wb);

    // Get as String
    let mut factions: Vec<String> = FACTIONS.iter().map(|s| s.to_string()).collect();
    let mut culture_components: Vec<String> =
        CULTURE_COMPONENTS.iter().map(|s| s.to_string()).collect();

    // 1] Sheet1 = Cultural Components
    let cc_sheet = create_sheet(&mut wb, "cultural_components", 0).unwrap();

    cc_sheet.set_value(0, 0, "Faction");
    fill_column_offset(cc_sheet, 0, 1, &mut factions.clone());
    fill_row_offset(cc_sheet, 0, 1, &mut culture_components);

    // 2] Sheet2 = Family Names
    let fn_sheet = create_sheet(&mut wb, "family_names", 1).unwrap();

    fill_row(fn_sheet, 0, &mut factions);
    for i in 0..FAMILY_NAMES.len() {
        let mut family_names = sort_names(FAMILY_NAMES[i].to_owned());
        fill_column_offset(fn_sheet, i as u32, 1, &mut family_names);
    }

    // Write to File
    spreadsheet_ods::write_ods(&mut wb, path).expect("write_ods")
}

/* #region NAMES */

// Take a sring of names
// -> Fill sheet with it

/// Create a names WorkBook
/// Each sheet = 1 Culture Component Names
pub fn create_names_workbook() {
    println!("---Create Names Workbook");
    let path = std::path::Path::new("test_out/dyng_names.ods");
    let mut wb = create_or_get_workbook(path).unwrap();

    clear_sheets(&mut wb);

    let mut column_names = assign_strings(ATTRIBUTES.to_vec(), CATEGORIES.to_vec());
    // print_vec(&column_names);

    // European Names
    let european_sheet = create_sheet(&mut wb, "european", 0).unwrap();
    // TODO

    // Nordic Names
    let nordic_sheet = create_sheet(&mut wb, "nordic", 1).unwrap();
    fill_row(nordic_sheet, 0, &mut column_names);

    for i in 0..NORDIC_NAMES.len() {
        let name_categories = NORDIC_NAMES[i].to_owned();

        for j in 0..name_categories.len() {
            let column_id = i * 3 + j; // j = category
            let mut names = sort_names(name_categories[j].to_owned());
            fill_column_offset(nordic_sheet, column_id as u32, 1, &mut names);
        }
    }

    // Write to File
    spreadsheet_ods::write_ods(&mut wb, path).expect("write_ods")
}

/// Fill a Culture Component Sheet with names
pub fn fill_sheet(sheet: &mut Sheet) {
    // TODO
    // let column_id = column_entry.attribute as u32 * 3 + column_entry.category as u32;
    // let mut strings = sort_names(column_entry.string);

    // fill_column(sheet, column_id, &mut strings);
}

/* #endregion */

pub fn create_attributes_workbook() {
    println!("---Create Attributes Workbook");
    let path = std::path::Path::new("test_out/dyng_attributes.ods");
    let mut wb = create_or_get_workbook(path).unwrap();

    clear_sheets(&mut wb);

    // 1] Sheet1 = Titles
    let titles_sheet = create_sheet(&mut wb, "titles", 0).unwrap();

    // First Row
    let mut columns_names = assign_strings(ATTRIBUTES.to_vec(), NAME_TYPES.to_vec());
    fill_row(titles_sheet, 0, &mut columns_names);

    // Titles
    for i in 0..TITLES.len() {
        let column_id = (i * 6) as u32; // titles types
        for j in 0..TITLES[i].len() {
            let mut title = sort_names(TITLES[i][j].to_owned());
            fill_column_offset(titles_sheet, column_id + j as u32, 1, &mut title);
        }
    }

    // 2] Sheet2 = Values
    let values_sheet = create_sheet(&mut wb, "values", 1).unwrap();
    // TODO

    // Write to File
    spreadsheet_ods::write_ods(&mut wb, path).expect("write_ods");
}