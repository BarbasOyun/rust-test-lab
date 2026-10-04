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

#[derive(Clone)]
pub struct ColumnEntry {
    attribute: Attribute,
    category: Category,
    string: String,
}

pub struct ColumnEntry2 {
    attribute: Attribute,
    mix_string: String,
    masc_string: String,
    fem_string: String,
}

// Sheets

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

// TODO : Fill Row & Column

// DyNG Spread Sheet Filler

/* #region NAMES */

// Take a sring of names
// -> Fill sheet with it

/// Create a names WorkBook
/// Each sheet = 1 Culture Component Names
pub fn create_names_workbook() {
    let path = std::path::Path::new("test_out/dyng_names.ods");
    let mut wb = create_or_get_workbook(path).unwrap();

    // European Names
    let european_sheet = create_sheet(&mut wb, "European", 0).unwrap();
    // TODO

    // Nordic Names
    let nordic_sheet = create_sheet(&mut wb, "Nordic", 1).unwrap();

    let nordic_name_list = vec![
        // STR
        "-STR_Mix
            -Vyrkol
            -Howitir
            -Idwyr
            -Hilmar
            -Hrovitnir",
        "-STR_Masc
            -Bjohrn
            -Ragnvald
            -Ulthor
            -Olaf",
        "-STR_Fem
            -Jarla",
        // SPI
        "-SPI_Mix
            -Eradan",
        "-SPI_Masc
            -Ymladd
            -Lothrik
            -Unduradh",
        "-SPI_Fem
            -Ulgrate",
        // INT
        "-INT_Mix
            -Yhorm",
        "-INT_Masc
            -Einar",
        "-INT_Fem",
        // CUN
        "-CUN_Mix
            -Lokat
            -Leiden",
        "-CUN_Masc
            -Zigomar",
        "-CUN_Fem
            -Shanar",
        // DEX
        "-DEX_Mix
            -Ivyr",
        "-DEX_Masc
            -Rarick
            -Holten",
        "-DEX_Fem
            -Valla",
        // MOT
        "-MOT_Mix
            -Hyreim
            -Rakvar
            -Sjaard
            -Rekvam",
        "-MOT_Masc
            -Vugnar
            -Harald",
        "-MOT_Fem
            -Yigit",
    ];
    fill_sheet(nordic_sheet, nordic_name_list);

    // Write to File
    spreadsheet_ods::write_ods(&mut wb, path).expect("write_ods")
}

/// Fill sheet with names_list = Vec<names>
/// Use DyNG Data structure for names
fn fill_sheet(sheet: &mut Sheet, names_list: Vec<&str>) {
    let mut attribute = Attribute::Strength;
    let mut category = Category::Mix;

    for names in names_list {
        let column_entry = ColumnEntry {
            attribute: attribute.clone(),
            category: category.clone(),
            string: names.to_string(),
        };

        add_to_sheet(sheet, column_entry);

        let is_last_category = category == Category::Fem;
        if is_last_category {
            attribute = attribute.get_next();
        }
        category = category.get_next();
    }
}

/// Add a Column entry to a Sheet
pub fn add_to_sheet(sheet: &mut Sheet, column_entry: ColumnEntry) {
    let column_id = column_entry.attribute as u8 * 3 + column_entry.category as u8;
    let mut strings = sort_names(column_entry.string);

    // Loop over names
    for i in 0..strings.len() {
        sheet.set_value(i as u32, column_id as u32, strings.swap_remove(0));
    }
}

/// Take a string of : -name1 -name2 -...
/// Return Vec<String> of each name
pub fn sort_names(string: String) -> Vec<String> {
    println!("---Sorting Names");
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

/* #endregion */

/* #region FACTIONS */

pub fn create_factions_workbook() {
    // TODO
}

/* #endregion */

/* #region ATTRIBUTES */

pub fn create_attributes_workbook() {
    // TODO
}

/* #endregion */