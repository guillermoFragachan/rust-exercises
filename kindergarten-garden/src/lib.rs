
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let mut result = Vec::new();


    result.push("value");
    let real = convert_rows_into_characters(diagram);


    return real;

}


fn convert_rows_into_characters(rows: &str) -> Vec<&'static str> {
    let mut plants: Vec<&'static str> = Vec::new();

    for row in rows.split("\n") {
        let plant_row = row.chars();

        for char in plant_row {
            let object =  Plant::from_char(char).unwrap();
            let plant  = Plant::to_string(object);


            plants.push(plant)

        }

        println!("{}", row);
    }

    plants
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
                _ => ""
        };
        var_name
    }
}