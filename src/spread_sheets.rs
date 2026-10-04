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

fn create_or_get_workbook<P: AsRef<Path>>(path: P) -> Result<WorkBook, String> {
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

// AUTHORING DATA

const ATTRIBUTES: [&str; 6] = ["STR", "SPI", "INT", "CUN", "DEX", "MOT"];

const NAME_TYPES: [&str; 6] = [
    "Adjective",
    "Name",
    "Equipment",
    "Bodypart",
    "Creature",
    "Role",
];

const CATEGORIES: [&str; 3] = ["Mix", "Masc", "Fem"];

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

/* #region FACTIONS DATA */

const KNIGHTS_FAMILIES: [&str; 14] = [
    "Tilbury",
    "Wade",
    "Tregor",
    "Placidius",
    "Wolder",
    "Kihei",
    "Sceptile",
    "Retsen",
    "Talowi",
    "Archandiel",
    "Kessada",
    "Malori",
    "Victus",
    "Thayer",
];

/* #endregion */

/* #region ATTRIBUTES DATA */

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

const STR_TITLES: [&str; 6] = [
    STR_ADJECTIVE,
    STR_NAME,
    STR_EQUIPMENT,
    STR_BODYPART,
    STR_CREATURE,
    STR_ROLE,
];

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

/* #region NAMES */

// Take a sring of names
// -> Fill sheet with it

/// Create a names WorkBook
/// Each sheet = 1 Culture Component Names
pub fn create_names_workbook() {
    // println!("---Create Names Workbook");
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

/* #region FACTIONS */

pub fn create_factions_workbook() {
    println!("---Create Factions Workbook");
    let path = std::path::Path::new("test_out/dyng_fations.ods");
    let mut wb = create_or_get_workbook(path).unwrap();

    clear_sheets(&mut wb);

    // Get as String
    let mut factions: Vec<String> = FACTIONS.iter().map(|s| s.to_string()).collect();
    let mut culture_components: Vec<String> =
        CULTURE_COMPONENTS.iter().map(|s| s.to_string()).collect();
    let mut knights_families: Vec<String> =
        KNIGHTS_FAMILIES.iter().map(|s| s.to_string()).collect();

    // Sheet1 = Cultural Components
    let cc_sheet = create_sheet(&mut wb, "cultural_components", 0).unwrap();

    cc_sheet.set_value(0, 0, "Faction");
    fill_column_offset(cc_sheet, 0, 1, &mut factions.clone());
    fill_row_offset(cc_sheet, 0, 1, &mut culture_components);

    // Sheet2 = Family Names
    let fn_sheet = create_sheet(&mut wb, "family_names", 1).unwrap();

    fill_row(fn_sheet, 0, &mut factions);
    fill_column_offset(fn_sheet, 0, 1, &mut knights_families);

    // Write to File
    spreadsheet_ods::write_ods(&mut wb, path).expect("write_ods")
}

/* #endregion */

/* #region ATTRIBUTES */

pub fn create_attributes_workbook() {
    println!("---Create Attributes Workbook");
    let path = std::path::Path::new("test_out/dyng_attributes.ods");
    let mut wb = create_or_get_workbook(path).unwrap();

    clear_sheets(&mut wb);

    // Get String
    // let mut str_adjectives = sort_names(STR_ADJECTIVE.to_owned());

    // Sheet1 = Titles
    let titles_sheet = create_sheet(&mut wb, "titles", 0).unwrap();

    // First Row
    let mut columns_names = assign_strings(ATTRIBUTES.to_vec(), NAME_TYPES.to_vec());

    fill_row(titles_sheet, 0, &mut columns_names);

    // STR Titles
    for i in 0..STR_TITLES.len() {
        let mut str_title = sort_names(STR_TITLES[i].to_owned());
        fill_column_offset(titles_sheet, 0 + i as u32, 1, &mut str_title);
    }

    // Sheet2 = Values
    let values_sheet = create_sheet(&mut wb, "values", 1).unwrap();
    // TODO

    // Write to File
    spreadsheet_ods::write_ods(&mut wb, path).expect("write_ods");
}

/* #endregion */
