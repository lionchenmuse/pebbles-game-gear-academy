use gstd::{debug, exec, Decode};
use gtest::{Program, RunResult, System};
use pebbles_game_io::*;

static SENDER_ID: u64 = 16;
// static PEBBLES_COUNT: u32 = 23;
// static PEBBLES_COUNT: u32 = 31;
// static PEBBLES_COUNT: u32 = 37;
static PEBBLES_COUNT: u32 = 72;
// static PEBBLES_COUNT: u32 = 111;
static MAX_PEBBLES_PER_TURN: u32 = 5;
// static MAX_PEBBLES_PER_TURN: u32 = 71;

/// 测试初始化合约，预期结果：成功
/// 测试方法，命令行执行：
/// </br>
/// cargo test test_init_success --features test_program_first | test_user_first
/// </br>
/// 注意：test_program_first | test_user_first，只能2选1
#[test]
fn test_init_success() {
    let sys = System::new();
    sys.init_logger();
    init_game_success(&sys, DifficultyLevel::Easy);
}

/// 测试初始化合约，预期结果：失败
/// 测试方法，命令行执行：
/// </br>
/// cargo test test_init_failure --features test_program_first | test_user_first
/// </br>
/// 注意：test_program_first | test_user_first，只能2选1
#[test]
fn test_init_failure() {
    let sys = System::new();
    sys.init_logger();
    let pebbles_init = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: MAX_PEBBLES_PER_TURN,    // 将石子数量与最大可移除数量调换，
        max_pebbles_per_turn: PEBBLES_COUNT,
    };

    let pebbles_game = Program::current(&sys);
    // assert_eq!(pebbles_game.id(), 1.into());

    // 初始化游戏
    let result = pebbles_game.send(SENDER_ID, pebbles_init);
    assert!(result.main_failed());
}

/// 测试读取游戏状态，
/// 测试方法，命令行执行：
/// </br>
/// cargo test test_state --features test_program_first | test_user_first
/// </br>
/// 注意：test_program_first | test_user_first，只能2选1
#[test]
fn test_state() {
    let sys = System::new();
    sys.init_logger();

    let pebbles_game = init_game_success(&sys, DifficultyLevel::Easy);

    let game_state = read_state(&pebbles_game);

    assert_eq!(game_state.pebbles_count, PEBBLES_COUNT);
    assert_eq!(game_state.max_pebbles_per_turn, MAX_PEBBLES_PER_TURN);
}

/// 用户先手，难度：简单。
/// 测试方法，命令行执行：
/// </br>
/// cargo test test_handle_user_first_easy --features test_user_first
#[test]
fn test_handle_user_first_easy() {
    let sys = System::new();
    sys.init_logger();

    let pebbles_game = init_game_success(&sys, DifficultyLevel::Easy);
    let mut game_state = read_state(&pebbles_game);

    let user_turns = create_user_turns(&game_state);

    let mut count = 0;
    loop {
        if game_state.pebbles_remaining == 0 || count == user_turns.len() {
            break;
        }
        pebbles_game.send(SENDER_ID, PebblesAction::Turn(user_turns[count]));

        game_state = read_state(&pebbles_game);
        count += 1;
    }
    
}

/// 程序先手，难度：简单。
/// 测试方法，命令行执行：
/// </br>
/// cargo test test_handle_program_first_easy --features test_program_first
#[test]
fn test_handle_program_first_easy() {
    let sys = System::new();
    sys.init_logger();

    let pebbles_game = init_game_success(&sys, DifficultyLevel::Easy);
    let mut game_state = read_state(&pebbles_game);

    let user_turns = create_user_turns(&game_state);

    let mut count = 0;
    loop {
        if game_state.pebbles_remaining == 0 || count == user_turns.len() {
            break;
        }
        pebbles_game.send(SENDER_ID, PebblesAction::Turn(user_turns[count]));

        game_state = read_state(&pebbles_game);
        count += 1;
    }
}

/// 用户先手，难度：困难。
/// 测试方法，命令行执行：
/// </br>
/// cargo test test_handle_user_first_hard --features test_user_first
#[test]
fn test_handle_user_first_hard() {
    let sys = System::new();
    sys.init_logger();

    let pebbles_game = init_game_success(&sys, DifficultyLevel::Hard);
    let mut game_state = read_state(&pebbles_game);

    let user_turns = create_user_turns(&game_state);

    let mut count = 0;
    loop {
        if game_state.pebbles_remaining == 0 || count == user_turns.len() {
            break;
        }
        pebbles_game.send(SENDER_ID, PebblesAction::Turn(user_turns[count]));

        game_state = read_state(&pebbles_game);
        count += 1;
    }
}

/// 程序先手，难度：困难。
/// 测试方法，命令行执行：
/// </br>
/// cargo test test_handle_program_first_hard --features test_program_first
#[test]
fn test_handle_program_first_hard() {
    let sys = System::new();
    sys.init_logger();

    let pebbles_game = init_game_success(&sys, DifficultyLevel::Hard);
    let mut game_state = read_state(&pebbles_game);

    let user_turns = create_user_turns(&game_state);

    let mut count = 0;
    loop {
        if game_state.pebbles_remaining == 0 || count == user_turns.len() {
            break;
        }
        pebbles_game.send(SENDER_ID, PebblesAction::Turn(user_turns[count]));

        game_state = read_state(&pebbles_game);
        count += 1;
    }
}

/// 测试中途放弃
/// 测试方法，命令行执行：
/// </br>
/// cargo test test_handle_give_up --features test_program_first | test_user_first
/// </br>
/// 注意：test_program_first | test_user_first，只能2选1
#[test]
fn test_handle_give_up() {
    let sys = System::new();
    sys.init_logger();

    let pebbles_game = init_game_success(&sys, DifficultyLevel::Hard);
    let mut game_state = read_state(&pebbles_game);

    let user_turns = create_user_turns(&game_state);

    let mut count = 0;
    loop {
        if game_state.pebbles_remaining <= game_state.pebbles_count / 2 {
            pebbles_game.send(SENDER_ID, PebblesAction::GiveUp);
            break;
        }
        pebbles_game.send(SENDER_ID, PebblesAction::Turn(user_turns[count]));
        game_state = read_state(&pebbles_game);
        count += 1;
    }
    game_state = read_state(&pebbles_game);

    assert_eq!(game_state.winner, Some(Player::Program));
}

/// 测试中途重启
/// 测试方法，命令行执行：
/// </br>
/// cargo test test_handle_restart --features test_program_first | test_user_first
/// </br>
/// 注意：test_program_first | test_user_first，只能2选1
#[test]
fn test_handle_restart() {
    let sys = System::new();
    sys.init_logger();

    let pebbles_game = init_game_success(&sys, DifficultyLevel::Hard);
    let mut game_state = read_state(&pebbles_game);

    let user_turns = create_user_turns(&game_state);

    let mut count = 0;
    loop {
        if game_state.pebbles_remaining <= game_state.pebbles_count / 2 {
            pebbles_game.send(SENDER_ID, PebblesAction::Restart { 
                difficulty: DifficultyLevel::Hard, 
                pebbles_count: PEBBLES_COUNT, 
                max_pebbles_per_turn: MAX_PEBBLES_PER_TURN 
            });
            break;
        }
        pebbles_game.send(SENDER_ID, PebblesAction::Turn(user_turns[count]));
        game_state = read_state(&pebbles_game);
        count += 1;
    }

    game_state = read_state(&pebbles_game);
    assert_eq!(game_state.pebbles_count, PEBBLES_COUNT);
    if game_state.first_player == Player::Program {
        assert_ne!(game_state.pebbles_count, game_state.pebbles_remaining);
    } else {
        assert_eq!(game_state.pebbles_count, game_state.pebbles_remaining);
    }
    
    
}

/// 测试错误输入
/// 测试方法，命令行执行：
/// </br>
/// cargo test test_handle_wrong_input --features test_program_first | test_user_first
/// </br>
/// 注意：test_program_first | test_user_first，只能2选1
#[test]
fn test_handle_wrong_input() {
    let sys = System::new();
    sys.init_logger();

    let pebbles_game = init_game_success(&sys, DifficultyLevel::Easy);
    let mut game_state = read_state(&pebbles_game);

    let mut user_turns = create_user_turns(&game_state);
    user_turns[0] = 0;
    user_turns[1] = 0;
    user_turns[2] = MAX_PEBBLES_PER_TURN + 1;

    let mut count = 0;
    loop {
        if game_state.pebbles_remaining == 0 || count == user_turns.len() {
            break;
        }
        pebbles_game.send(SENDER_ID, PebblesAction::Turn(user_turns[count]));

        game_state = read_state(&pebbles_game);
        count += 1;
    }
}

/// 测试游戏初始化，断言成功
fn init_game_success(sys: &System, difficulty: DifficultyLevel) -> Program {
    let pebbles_init = PebblesInit {
        difficulty: difficulty,
        pebbles_count: PEBBLES_COUNT,
        max_pebbles_per_turn: MAX_PEBBLES_PER_TURN,
    };

    let pebbles_game = Program::current(&sys);
    // assert_eq!(pebbles_game.id(), 1.into());

    // 初始化游戏
    let result = pebbles_game.send(SENDER_ID, pebbles_init);
    // 断言初始化成功
    assert!(!result.main_failed());

    pebbles_game
}

fn read_state(pebbles_game: &Program) -> GameState {
    pebbles_game.read_state(b"").unwrap()
}

fn create_user_turns(game_state: &GameState) -> Vec<u32> {
    let mut user_turns: Vec<u32> = Vec::new();
    let mut count = 0;
    for _ in 0..game_state.pebbles_count {
        let mut turn_num = (count + 31) % MAX_PEBBLES_PER_TURN;
        if turn_num == 0 {
            if count % 2 == 0 {
                turn_num = MAX_PEBBLES_PER_TURN;
            } else {
                turn_num = 1;
            }
        }
        user_turns.push(turn_num);
        count += 1;
    }
    user_turns
}