mod common;
mod challenge_1;
mod challenge_2;
mod challenge_3;
mod challenge_4;
mod challenge_5;
mod challenge_6;
mod challenge_7;
mod challenge_8;
mod challenge_9;

const CHALLENGES: [Option<fn()>; 9] = [
    Some(challenge_1::challenge1),
    Some(challenge_2::challenge2),
    Some(challenge_3::challenge3),
    Some(challenge_4::challenge4),
    Some(challenge_5::challenge5),
    Some(challenge_6::challenge6),
    Some(challenge_7::challenge7),
    Some(challenge_8::challenge8),
    Some(challenge_9::challenge9),

];


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        eprintln!("Too many arguments. Optionally select which challenge to execute with integer.");
    }

    if args.len() == 2 {
        let second_arg = args[1].parse::<usize>();
        let mut ran_challenge = false;
        if let Ok(challenge_selected) = second_arg {
            if let Some(Some(challenge_function)) = CHALLENGES.get(challenge_selected.wrapping_sub(1)) {
                challenge_function();
                ran_challenge = true;
            }
        }
        if !ran_challenge {
            eprintln!("Non-implemented day selected")
        }
    } else {
        for challenge_function in CHALLENGES.iter().flatten() {
            challenge_function();
            println!("\n\n")
        }
    }


}
