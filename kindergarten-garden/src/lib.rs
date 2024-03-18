pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let mut result = Vec::new();

    result.push("value");
    let real = convert_rows_into_characters(diagram, student);

    return real;
}

fn convert_rows_into_characters(rows: &str, student: &str) -> Vec<&'static str> {
    let mut plants: Vec<&'static str> = Vec::new();

    let rows: Vec<&str> = rows.split("\n").collect();

    println!("{}", rows[0]);

    let plant_column1 = divide_string(rows[0]);

    let plants_column2 = divide_string(rows[1]);

    for (i, students) in STUDENTS.iter().enumerate() {
        // plant_row = the
        if student.to_string() == students.to_string() {
            let chars = plant_column1[i].chars();

            for char in chars {
                let object = Plant::from_char(char).unwrap();
                let plant = Plant::to_string(object);
                plants.push(plant)
            }
        }
    }

    for (i, students) in STUDENTS.iter().enumerate() {
        // plant_row = the
        if student.to_string() == students.to_string() {
            let chars = plants_column2[i].chars();

            for char in chars {
                let object = Plant::from_char(char).unwrap();
                let plant = Plant::to_string(object);
                plants.push(plant)
            }
        }
    }



    plants
}
fn divide_string(s: &str) -> Vec<&str> {
    s.as_bytes()
        .chunks(2)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect()
}
#[derive(Clone, Debug)]
enum Plant {
    Grass,
    Clover,
    Radish,
    Violet,
}

impl Plant {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'G' => Some(Plant::Grass),
            'C' => Some(Plant::Clover),
            'R' => Some(Plant::Radish),
            'V' => Some(Plant::Violet),
            _ => None,
        }
    }

    fn to_string(c: Plant) -> &'static str {
        let var_name = match c {
            Plant::Grass => "grass",
            Plant::Clover => "clover",
            Plant::Radish => "radishes",
            Plant::Violet => "violets",
            _ => "",
        };
        var_name
    }
}

const STUDENTS: &[&str] = &[
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];
