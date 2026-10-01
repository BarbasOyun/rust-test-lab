use icu_locale_core::locale;
use spreadsheet_ods::{
    OdsResult, Sheet, Value, WorkBook,
    color::Rgb,
    format, formula, mm,
    style::CellStyle,
    style::units::{Border, Length, TextRelief},
};
use std::{fs, path::Path};

enum Attribute {
    Strength,
    Spirit,
    Intelligence,
    Cunning,
    Dexterity,
    Motricity,
}

enum Category {
    Mix,
    Masc,
    Fem,
}

pub struct ColumnEntry {
    attribute: Attribute,
    category: Category,
    string: String,
}

/// Test
pub fn create_spread_sheet() {
    fs::create_dir_all("test_out").expect("create_dir");

    // Create or Get WorkBook Object
    let path = std::path::Path::new("test_out/lib_example.ods");
    let mut wb = if path.exists() {
        spreadsheet_ods::read_ods(path).unwrap()
    } else {
        WorkBook::new(locale!("en-US"))
    };

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

// DyNG Spread Sheet Filler

/// Create Nord Culture Component name registry
pub fn create_registry() {
    let path = std::path::Path::new("test_out/name_registry.ods");

    let mot_mix = ColumnEntry {
        attribute: Attribute::Motricity,
        category: Category::Mix,
        string: String::from(
            "-Hyreim
            -Rakvar
            -Sjaard
            -Rekvam",
        ),
    };

    fill_registry(path, mot_mix);

    let mot_masc = ColumnEntry {
        attribute: Attribute::Motricity,
        category: Category::Masc,
        string: String::from(
            "-Vugnar
            -Harald",
        ),
    };

    fill_registry(path, mot_masc);
}

/// Create or Write a .ods file at path
/// Fill Sheet1 at column index with Vec<String>
pub fn fill_registry<P: AsRef<Path>>(path: P, column_entry: ColumnEntry) {
    let column_id = column_entry.attribute as u8 * 3 + column_entry.category as u8;
    let mut strings = sort_names(column_entry.string);

    // Create or Get WorkBook
    let mut wb = if path.as_ref().exists() {
        spreadsheet_ods::read_ods(&path).unwrap()
    } else {
        WorkBook::new(locale!("en-US"))
    };

    // Create Sheet1
    if wb.num_sheets() == 0 {
        let sheet = Sheet::new("one");
        wb.push_sheet(sheet);
    }

    let sheet = wb.sheet_mut(0);

    // Loop over names
    for i in 0..strings.len() {
        sheet.set_value(i as u32, column_id as u32, strings.swap_remove(0));
    }

    // Write to File
    spreadsheet_ods::write_ods(&mut wb, path).expect("write_ods")
}

/// Take a string that contain -name1 -name2 -...
/// Return Vec<String>
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
            println!("Name = {}", current_name);
            names.push(current_name.clone());
            current_name = String::from("");
        }
    }

    current_name = current_name.trim().to_string();
    println!("Name = {}", current_name);
    names.push(current_name.clone());

    return names;
}
