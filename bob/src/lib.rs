pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let is_empty = message.is_empty();
    let is_shouting = is_all_uppercase(message);
    let ends_with_question_mark = message.ends_with('?') && !is_not_alphabetical(message);

    if is_empty {
        "Fine. Be that way!"
    } else if is_shouting && ends_with_question_mark {
        "Calm down, I know what I'm doing!"
    } else if is_shouting {
        "Whoa, chill out!"
    } else if ends_with_question_mark {
        "Sure."
    } else {
        "Whatever."
    }
}

fn is_not_alphabetical (s: &str) -> bool {
     !s.chars().any(|c|c.is_ascii_alphabetic())
}

fn is_all_uppercase(s: &str) -> bool {
    s.chars().filter(|character| character.is_ascii_alphabetic()).all(|character| character.is_ascii_uppercase())
}


fn has_uppercase(s: &str) -> bool {
    s.chars()
        .any(|c| c.is_ascii_alphabetic() && c.is_ascii_uppercase())
}

