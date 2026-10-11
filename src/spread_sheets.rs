use icu_locale_core::locale;
use spreadsheet_ods::{
    Sheet, Value, WorkBook,
};
use std::{fs, path::Path};

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

pub fn clear_sheets(wb: &mut WorkBook) {
    for i in (0..wb.num_sheets()).rev() {
        // println!("Remove Sheet at : {}", i);
        wb.remove_sheet(i);
    }
}

/// Try to Create a sheet in a WorkBook at Index
pub fn create_sheet<'a>(
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
pub fn fill_row(sheet: &mut Sheet, row_id: u32, strings: &mut Vec<String>) {
    let str_count = strings.len().clone();

    for i in 0..str_count {
        let value = strings.remove(0);
        // println!("Set value : {} at x = {}, y = {}", value, row_id, i as u32);
        sheet.set_value(row_id, i as u32, value);
    }
}

pub fn fill_row_offset(sheet: &mut Sheet, row_id: u32, offset: u32, strings: &mut Vec<String>) {
    for i in 0..strings.len() {
        sheet.set_value(row_id, i as u32 + offset, strings.remove(0));
    }
}

/// Fill a column with Strings
pub fn fill_column(sheet: &mut Sheet, column_id: u32, strings: &mut Vec<String>) {
    for i in 0..strings.len() {
        sheet.set_value(i as u32, column_id, strings.remove(0));
    }
}

pub fn fill_column_offset(sheet: &mut Sheet, column_id: u32, offset: u32, strings: &mut Vec<String>) {
    for i in 0..strings.len() {
        sheet.set_value(i as u32 + offset, column_id, strings.remove(0));
    }
}

// UTILS

/// print a Vec<String>
pub fn print_vec(strings: &Vec<String>) {
    for i in 0..strings.len() {
        println!("{} : {}", i, strings[i])
    }
}

/// Take 2 Vec<&str>
/// Each value in Vec1 is assigned each value from Vec2
/// Return Vec3 = Vec1[i] + "_" + Vec2[j]
pub fn assign_strings(vec1: Vec<&str>, vec2: Vec<&str>) -> Vec<String> {
    let mut vec3 = vec![];

    for i in 0..vec1.len() {
        for j in 0..vec2.len() {
            let value = vec1[i].to_owned() + "_" + vec2[j];
            vec3.push(value);
        }
    }

    return vec3;
}
