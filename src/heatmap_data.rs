use rocket_contrib::json::JsonValue;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

fn get_csv_labels<T: Into<Option<char>>, P: AsRef<Path>>(path: P, delimiter: T) -> Option<Vec<String>> {
    let delimiter = delimiter.into().unwrap_or(',');
    Some(
        fs::read_to_string(path).ok()?
            .split("\n")
            .nth(0)?
            .split(delimiter)
            .map(|column| 
                 column.trim_matches(
                    |a| a == '"'
                 ).to_string())
            .collect()
    )
}

fn get_csv_data<T: Into<Option<char>>, P: AsRef<Path>>(path: P, delimiter: T) -> Option<Vec<Vec<usize>>> {
    let delimiter = delimiter.into().unwrap_or(',');
    Some(
        fs::read_to_string(path).ok()?
        .split("\n")
        .skip(1)
        .map(|line|
            line
            .split(delimiter)
            .map(|number|
                number
                .trim_matches(|a| a == '"')
                .parse::<usize>().unwrap()
            ).collect()
        ).collect()
    )
}

fn normalize(v: Vec<Vec<usize>>) -> Vec<Vec<f64>> {
    if v.is_empty() {
        return vec![];
    }
    let col_count = v[0].len();
    let mut v_out = vec![vec![0.0; col_count]; v.len()];
    for row in 0..v.len() {
        v_out[row][0] = v[row][0] as f64;
    }
    for col in 1..col_count {
        let mut max = 0.0;
        for row in 0..v.len() {
            if v[row][col] as f64 > max {
                max = v[row][col] as f64;
            }
        }
        if max != 0.0 {
            for row in 0..v.len() {
                v_out[row][col] = v[row][col] as f64 / max;
            }
        }
    }
    v_out
}

fn read_zip_csv() -> Option<Vec<(usize, f64, f64)>> {
    Some(fs::read_to_string("zips.csv").ok()?
        .split("\n")
        .map(|a| {dbg!(a.clone()); a})
        .filter_map(|line| 
             (Some((
                line.split(",").nth(0)?.parse().ok().unwrap(),
                line.split(",").nth(1)?.parse().ok().unwrap(),
                line.split(",").nth(2)?.parse().ok().unwrap()
        ))))
        .collect()
    )
}

fn get_zip_locs() -> Option<HashMap<usize, (f64, f64)>> {
    let mut h = HashMap::<usize, (f64, f64)>::new();
    
    for (zip, long, lat) in read_zip_csv()?.iter() {
        h.insert(*zip, (*long, *lat));
    }

    Some(h)
}

fn get_zip_loc(zip_locs: &HashMap<usize, (f64, f64)>, zip: usize) -> (f64, f64) {
    if !zip_locs.contains_key(&zip) {
        println!("Warning zip {} not found", zip);
        return (0.0, 0.0);
    }
    zip_locs[&zip]
}

pub fn simplify_heatmap_data() -> Option<JsonValue> {
    let csv_labels = get_csv_labels("client_encounters.csv", '|').unwrap();
    let csv_data = normalize(get_csv_data("client_encounters.csv", '|').unwrap());
    let zip_locs = get_zip_locs().unwrap();
    dbg!(&zip_locs);
    Some(json![{
        "data" :
        csv_labels
            .iter()
            .enumerate()
            .map(|(i, label)| 
                 json![{
                     label.clone() : 
                        csv_data
                         .iter()
                         .map(|row|
                            json![{
                                "zip" : row[0] as usize,
                                "loc" : get_zip_loc(&zip_locs, row[0] as usize),
                                "val" : row[i]
                            }])
                        .collect::<Vec<JsonValue>>()
                 }]
            ).collect::<Vec<JsonValue>>()
    }])
}
