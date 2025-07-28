// main.rs
use minimax_toruney::{Position, eval_counter, reset_eval_counter};
use std::{env, error::Error, fs, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    //parse command line args
    let mut args: std::iter::Skip<env::Args> = env::args().skip(1);
    let in_path: String = args.next().expect(
        "usage: MiniMax <input file> <output file> [variant: white, black, white_ab, black_ab, white_improved, black_improved, white_ab_improved, black_ab_improved]",
    );
    let out_path: String = args.next().expect(
        "usage: MiniMax <input file> <output file> [variant: white, black, white_ab, black_ab, white_improved, black_improved, white_ab_improved, black_ab_improved]",
    );

    let minimax_type: String = args.next().unwrap_or_else(|| "white".to_string());
    if minimax_type != "white"
        && minimax_type != "black"
        && minimax_type != "white_ab"
        && minimax_type != "black_ab"
        && minimax_type != "white_improved"
        && minimax_type != "black_improved"
        && minimax_type != "white_ab_improved"
        && minimax_type != "black_ab_improved"
    {
        eprintln!(
            "invalid minimax type: {minimax_type}, expected one of: white, black, white_ab, black_ab, white_improved, black_improved, white_ab_improved, black_ab_improved"
        );
        std::process::exit(1);
    }

    if args.next().is_some() {
        eprintln!(
            "too many arguments, expected 3: <input file> <output file> [variant: white, black, white_ab, black_ab, white_improved, black_improved, white_ab_improved, black_ab_improved]"
        );
        std::process::exit(1);
    }

    // get args from board1.txt if it exists
    let contents: String = fs::read_to_string(&in_path)?;
    let numbers: Vec<u8> = contents
        .split_whitespace()
        .map(|raw: &str| {
            //strip leading zeros on depth args (008 -> 8)
            let s: &str = raw.trim_start_matches('0');
            let cleaned = if s.is_empty() { "0" } else { s };
            cleaned.parse::<u8>()
               .unwrap_or_else(|_| panic!("couldn't parse `{raw}` as u8"))
        })
        .collect(); // collect into a vector

    // make sure file is of correct format
    if numbers.len() != 5 {
        panic!(
            "expected 5 arguments for board position and depth in input.txt, but found {}",
            numbers.len()
        );
    }

    // destructure the numbers into variables
    let [w1, w2, b1, b2, depth] =
        <[u8; 5]>::try_from(numbers).expect("slice with incorrect length");

    let start_time = Instant::now();
    // set the depth for minimax
    let max_depth: u8 = depth;
    
    // create the starting position
    let start: Position = Position::new(w1, w2, b1, b2);

    // run minimax and capture results
    let (best, score, evals, depth): (Position, i32, usize, u8) = match minimax_type.as_str() {
        "white" => minimax_white(&start, max_depth),
        "black" => minimax_black(&start, max_depth),
        "white_ab" => minimax_white_ab(&start, max_depth),
        "black_ab" => minimax_black_ab(&start, max_depth),
        "white_improved" => minimax_white_improved(&start, max_depth),
        "black_improved" => minimax_black_improved(&start, max_depth),
        "white_ab_improved" => minimax_white_ab_improved(&start, max_depth),
        "black_ab_improved" => minimax_black_ab_improved(&start, max_depth),
        // if minimax_type is not one of the above, panic
        _ => panic!("invalid minimax type: {minimax_type}"),
    };

    let duration: std::time::Duration = start_time.elapsed();
    println!("Board Position: {best}");
    println!("Positions evaluated by static estimation: {evals}");
    println!("Search depth: {depth}");
    println!("MINIMAX estimate: {score}");

    println!("Time taken: {:?}", duration);

    // write best, evals, depth, and score to output.txt
    let out: String = format!(
        "{best}\n\
         Positions evaluated by static estimation: {evals}\n\
         Search depth: {depth}\n\
         MINIMAX estimate: {score}\n"
    );
    fs::write(&out_path, out)?;

    Ok(())
}

fn minimax_white(pos: &Position, depth: u8) -> (Position, i32, usize, u8) {
    reset_eval_counter(); // to make sure the eval counter is set to zero before starting the search
    let (best, score): (Position, i32) =
        Position::best_white_move(pos, depth).expect("White must have a legal move");

    // evaluate the score of the best move
    let evals: usize = eval_counter();

    (best, score, evals, depth)
}

fn minimax_white_ab(pos: &Position, depth: u8) -> (Position, i32, usize, u8) {
    reset_eval_counter();
    let (best, score): (Position, i32) =
        Position::best_white_move_ab(pos, depth).expect("White must have a legal move");

    // evaluate the score of the best move
    let evals: usize = eval_counter();

    (best, score, evals, depth)
}

fn minimax_black(pos: &Position, depth: u8) -> (Position, i32, usize, u8) {
    reset_eval_counter(); // to make sure the eval counter is set to zero before starting the search
    let (best, score): (Position, i32) =
        Position::best_black_move(pos, depth).expect("Black must have a legal move");

    // evaluate the score of the best move
    let evals: usize = eval_counter();

    (best, score, evals, depth) // score is not used for black moves in this context
}

fn minimax_black_ab(pos: &Position, depth: u8) -> (Position, i32, usize, u8) {
    reset_eval_counter(); // to make sure the eval counter is set to zero before starting the search
    let (best, score): (Position, i32) =
        Position::best_black_move_ab(pos, depth).expect("Black must have a legal move");

    // evaluate the score of the best move
    let evals: usize = eval_counter();

    (best, score, evals, depth)
}

pub fn minimax_white_improved(pos: &Position, depth: u8) -> (Position, i32, usize, u8) {
    reset_eval_counter(); // to make sure the eval counter is set to zero before starting the search
    let (best, score): (Position, i32) =
        Position::best_white_move_improved(pos, depth).expect("White must have a legal move");

    // evaluate the score of the best move
    let evals: usize = eval_counter();

    (best, score, evals, depth)
}

pub fn minimax_white_ab_improved(pos: &Position, depth: u8) -> (Position, i32, usize, u8) {
    reset_eval_counter(); // to make sure the eval counter is set to zero before starting the search
    let (best, score): (Position, i32) =
        Position::best_white_move_ab_improved(pos, depth).expect("White must have a legal move");

    // evaluate the score of the best move
    let evals: usize = eval_counter();

    (best, score, evals, depth)
}

pub fn minimax_black_improved(pos: &Position, depth: u8) -> (Position, i32, usize, u8) {
    reset_eval_counter(); // to make sure the eval counter is set to zero before starting the search
    let (best, score): (Position, i32) =
        Position::best_black_move_improved(pos, depth).expect("Black must have a legal move");

    // evaluate the score of the best move
    let evals: usize = eval_counter();

    (best, score, evals, depth)
}

pub fn minimax_black_ab_improved(pos: &Position, depth: u8) -> (Position, i32, usize, u8) {
    reset_eval_counter(); // to make sure the eval counter is set to zero before starting the search
    let (best, score): (Position, i32) =
        Position::best_black_move_ab_improved(pos, depth).expect("Black must have a legal move");

    // evaluate the score of the best move
    let evals: usize = eval_counter();

    (best, score, evals, depth)
}