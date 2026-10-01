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

pub struct ColumnEntry2 {
    attribute: Attribute,
    mix_string: String,
    masc_string: String,
    fem_string: String,
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

/// Create Nordic Culture Component name registry
pub fn create_registry() {
    let path = std::path::Path::new("test_out/name_registry.ods");

    // STR
    let str_mix = ColumnEntry {
        attribute: Attribute::Strength,
        category: Category::Mix,
        string: String::from(
            "-Vyrkol
            -Howitir
            -Idwyr
            -Hilmar
            -Hrovitnir",
        ),
    };
    fill_registry(path, str_mix);

    let str_masc = ColumnEntry {
        attribute: Attribute::Strength,
        category: Category::Masc,
        string: String::from(
            "-Bjohrn
            -Ragnvald
            -Ulthor
            -Olaf",
        ),
    };
    fill_registry(path, str_masc);

    let str_fem = ColumnEntry {
        attribute: Attribute::Strength,
        category: Category::Fem,
        string: String::from("-Jarla"),
    };
    fill_registry(path, str_fem);

    // SPI
    let spi_mix = ColumnEntry {
        attribute: Attribute::Spirit,
        category: Category::Mix,
        string: String::from("-Eradan"),
    };
    fill_registry(path, spi_mix);

    let spi_masc = ColumnEntry {
        attribute: Attribute::Spirit,
        category: Category::Masc,
        string: String::from(
            "-Ymladd
            -Lothrik
            -Unduradh",
        ),
    };
    fill_registry(path, spi_masc);

    let spi_fem = ColumnEntry {
        attribute: Attribute::Spirit,
        category: Category::Fem,
        string: String::from("Ulgrate"),
    };
    fill_registry(path, spi_fem);

    // INT
    let int_mix = ColumnEntry {
        attribute: Attribute::Intelligence,
        category: Category::Mix,
        string: String::from("-Yhorm"),
    };
    fill_registry(path, int_mix);

    let int_masc = ColumnEntry {
        attribute: Attribute::Intelligence,
        category: Category::Masc,
        string: String::from("-Einar"),
    };
    fill_registry(path, int_masc);

    let int_fem = ColumnEntry {
        attribute: Attribute::Intelligence,
        category: Category::Fem,
        string: String::from(""),
    };
    fill_registry(path, int_fem);

    // CUN
    let cun_mix = ColumnEntry {
        attribute: Attribute::Cunning,
        category: Category::Mix,
        string: String::from(
            "-Lokat
            -Leiden",
        ),
    };
    fill_registry(path, cun_mix);

    let cun_masc = ColumnEntry {
        attribute: Attribute::Cunning,
        category: Category::Masc,
        string: String::from("-Zigomar"),
    };
    fill_registry(path, cun_masc);

    let cun_fem = ColumnEntry {
        attribute: Attribute::Cunning,
        category: Category::Fem,
        string: String::from("-Shanar"),
    };
    fill_registry(path, cun_fem);

    // DEX
    let dex_mix = ColumnEntry {
        attribute: Attribute::Dexterity,
        category: Category::Mix,
        string: String::from("-Ivyr"),
    };
    fill_registry(path, dex_mix);

    let dex_masc = ColumnEntry {
        attribute: Attribute::Dexterity,
        category: Category::Masc,
        string: String::from(
            "-Rarick
            -Holten",
        ),
    };
    fill_registry(path, dex_masc);

    let dex_fem = ColumnEntry {
        attribute: Attribute::Dexterity,
        category: Category::Fem,
        string: String::from("-Valla"),
    };
    fill_registry(path, dex_fem);

    // MOT
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

    let mot_fem = ColumnEntry {
        attribute: Attribute::Motricity,
        category: Category::Fem,
        string: String::from("-Yigit"),
    };
    fill_registry(path, mot_fem);
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
        let sheet = Sheet::new("Nordic");
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
