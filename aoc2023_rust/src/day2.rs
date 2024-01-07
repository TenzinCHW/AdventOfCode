use std::io;
use std::fs;
use std::collections::HashMap;


fn max_balls(line: &str) -> HashMap<&str, u32> {
    let mut balls: HashMap<&str, u32> = HashMap::new();
    for ball_count in line.split(',') {
        let split_ball_count = ball_count.trim_matches(' ').split(' ').collect::<Vec<&str>>();
        assert_eq!(split_ball_count.len(), 2);
        let (count, ball) = (split_ball_count[0], split_ball_count[1]);
        balls.insert(ball.trim_matches(' '), count.parse().unwrap());
    }
    balls
}


fn make_game(line: &str) -> (u32, HashMap<&str, u32>) {
    let game_line: Vec<&str> = line.split(":").collect();
    assert_eq!(game_line.len(), 2);
    let game_id: u32 = game_line[0].split(' ').collect::<Vec<&str>>()[1].parse().unwrap();
    let rounds = game_line[1].split(';').map(|x| x.trim_matches(' '));
    let mut max_num_balls: HashMap<&str, u32> = HashMap::new();
    max_num_balls.insert("red", 0);
    max_num_balls.insert("green", 0);
    max_num_balls.insert("blue", 0);
    for round in rounds {
        let mb = max_balls(round);
        for (b, v) in mb.iter() {
            if *v > max_num_balls[b] {
                max_num_balls.insert(b, *v);
            }
        }
    }
    (game_id, max_num_balls)
}


fn game_is_possible(game_balls: &HashMap<&str, u32>, max_balls: &HashMap<&str, u32>) -> bool {
    for (k, v) in max_balls.iter() {
        if *v < game_balls[k] {
            return false;
        }
    }
    true
}


pub fn day2_p1(input_file: &str) -> Result<u32, io::Error> {
    // write fn that filters out not-possible games
    // store each game as a hashmap w/ max of each colour per game
    let max_balls: HashMap<&str, u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut valid_game_ids: Vec<u32> = Vec::new();
    for line in fs::read_to_string(input_file)?.lines() {
        let (game_id, game_balls) = make_game(line);
        if game_is_possible(&game_balls, &max_balls) {
            valid_game_ids.push(game_id);
        }
    }
    Ok(valid_game_ids.iter().copied().sum())
}


fn calculate_game_power(game_balls: HashMap<&str, u32>) -> u32 {
    let mut pow: u32 = 1;
    for (_, c) in game_balls.iter() {
        pow *= c;
    }
    pow
}


pub fn day2_p2(input_file: &str) -> Result<u32, io::Error> {
    // store each game as a hashmap w/ max of each colour per game
    let mut game_powers: Vec<u32> = Vec::new();
    for line in fs::read_to_string(input_file)?.lines() {
        let (game_id, game_balls) = make_game(line);
        let pow: u32 = calculate_game_power(game_balls);
        game_powers.push(pow);
    }
    Ok(game_powers.iter().copied().sum())
}
