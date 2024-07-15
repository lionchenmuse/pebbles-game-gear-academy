#![no_std]

use gstd::{debug, msg, exec};
use pebbles_game_io::*;

static mut PEBBLES_GAME: Option<GameState> = None;

#[no_mangle]
unsafe extern "C" fn init() {
    let pebbles_init: PebblesInit = msg::load().expect("Failed to decode PebblesInit");

    // 验证石子总数 是否大于每回合最大可拿的石子数
    if pebbles_init.pebbles_count <= pebbles_init.max_pebbles_per_turn {
        panic!("Pebbles count must be greater than max pebbles per turn");
    }

    // 获取先手玩家
    let first_player = choose_first_player();
    
    // 初始化游戏状态
    PEBBLES_GAME = Some(GameState::init(pebbles_init, first_player));

    // 如果是程序先手，则让程序先执行
    if first_player == Player::Program {
        exec_program_turn();
    }
}

#[no_mangle]
unsafe extern "C" fn handle() {
    // 获取游戏状态
    let game_state = PEBBLES_GAME.as_mut().expect("Failed to load game state");
    // 获取玩家动作
    let action: PebblesAction = msg::load().expect("Failed to decode PebblesAction");

    // 游戏结束标志
    let mut finished = false;
    // 玩家（程序）本轮移除的石子数量
    let mut counter_turn = 0_u32;
    match action {
        PebblesAction::Turn(count) => {
            debug!("玩家（用户）移除的石子数量：{}", count);
            // 移除石子
            match remove_pebbles(count) {
                // 如果输入数据有效，进入 Some 分支
                Some(()) => {
                    // 判断是否游戏结束，
                    if game_state.pebbles_remaining != 0 {
                        // 如果未结束，则轮到【程序】移除石子
                        counter_turn = exec_program_turn();
                        if game_state.pebbles_remaining == 0 {
                            game_state.set_winner(Player::Program);
                            finished = true;                        
                        }
                    } else {
                        // 如果结束，设置赢家
                        game_state.set_winner(Player::User);
                        // 将结束标志设为true
                        finished = true;    
                    }
                },
                // 如果输入数据无效，进入 None 分支
                None => ()
            }
            
        },
        PebblesAction::GiveUp => {
            debug!("用户放弃");
            game_state.set_winner(Player::Program);
            finished = true;
        },
        PebblesAction::Restart { difficulty, pebbles_count, max_pebbles_per_turn } => {
            debug!("用户重新开始游戏");
            let pebbles_init = PebblesInit {
                difficulty,
                pebbles_count,
                max_pebbles_per_turn,
            };
            // 获取先手玩家
            let first_player = choose_first_player();    
            // 初始化游戏状态
            PEBBLES_GAME = Some(GameState::init(pebbles_init, first_player));
            if first_player == Player::Program {
                counter_turn = exec_program_turn();
            }
        }
    }
    if finished {
        // 游戏结束，通知用户赢家是谁
        debug!("游戏结束，赢家是: {:?}", game_state.winner.unwrap());
        msg::reply(PebblesEvent::Won(game_state.winner.unwrap()), 0)
                .expect("Failed to send game over event");
    } else {
        // 游戏未结束，通知用户：程序移除的石子数量
        msg::reply(PebblesEvent::CounterTurn(counter_turn), 0)
                .expect("Failed to send counter turn event");
    }


}

#[no_mangle]
unsafe extern "C" fn state() {
    let game_state = PEBBLES_GAME.take().expect("Game state is not initialized");
    msg::reply(game_state, 0).expect("Failed to reply from `state()`");
}

#[cfg(feature = "prod")]
fn choose_first_player() -> Player {
    let r_num = get_random_u32();
    // 如果随机数是偶数，则第一个玩家为用户，否则为程序
    if r_num % 2 == 0 {
        debug!("First player: User");
        Player::User
    } else {
        debug!("First player: Program");
        Player::Program
    }
}

#[cfg(feature = "test_user_first")]
fn choose_first_player() -> Player {
    debug!("First player: User");
    Player::User
}

#[cfg(feature = "test_program_first")]
fn choose_first_player() -> Player {
    debug!("First player: Program");
    Player::Program
}

/// 执行程序回合
/// 并更新游戏状态
/// 返回被移除的石子数量
unsafe fn exec_program_turn() -> u32 {
    let game_state: &mut GameState = PEBBLES_GAME.as_mut().expect("Failed to load game state");
    let count: u32;
    match game_state.difficulty {
        DifficultyLevel::Easy => {
            count = take_easy_action();
        },
        DifficultyLevel::Hard => {
            count = take_hard_action();
        },
    }
    game_state.pebbles_remaining -= count;
    debug!("程序移除的石子数量：{:?}", count);
    debug!("剩余石子数: {}", game_state.pebbles_remaining);
    count
}

/// 简单模式下，程序回合
/// 随机移除石子
/// 返回被移除的石子数量
unsafe fn take_easy_action() -> u32 {
    // 获取游戏状态
    let game_state: &mut GameState = PEBBLES_GAME.as_mut().expect("Failed to load game state");
    // 获取随机取走石子的数量
    let mut count = get_random_u32();

    // 如果剩余石子数小于每回合最大可拿的石子数，则所取石子不能超过剩余石子数
    if game_state.pebbles_remaining < game_state.max_pebbles_per_turn{
        count = count % game_state.pebbles_remaining;
        if count == 0 {
            // 如果count为0，则取完剩余石子
            count = game_state.pebbles_remaining;
        }
    } else {    // 如果剩余石子数大于每回合最大可拿的石子数，则所取石子不能超过每回合最大可拿的石子数
        count = count % game_state.max_pebbles_per_turn;
        if count == 0 {
            // 如果count为0，则取每回合最大可拿的石子数
            count = game_state.max_pebbles_per_turn;
        }
    }
    count
}

/// 为确保最后剩余的石子数量为所允许拿走的最大石子数 + 1，
/// 应尽量让剩余的石子数量变为 (max_pebbles_per_turn + 1) 的倍数
/// 如果做不到，程序应尽可能多地拿走石子，但不能超过 max_pebbles_per_turn
/// 实际是谁先利用这条规律谁赢。
unsafe fn take_hard_action() -> u32 {
    let game_state: &mut GameState = PEBBLES_GAME.as_mut().expect("Failed to load game state");

    // 计算程序本轮应该拿走的石子数
    let target = game_state.pebbles_remaining % (game_state.max_pebbles_per_turn + 1);

    let count = if target == 0 {
        // 如果剩余的石子数量已经是 (max_pebbles_per_turn + 1) 的倍数，则拿走 max_pebbles_per_turn 个石子
        // 此时剩余的石子数已经不是 (max_pebbles_per_turn + 1) 的倍数，如果用户利用这条规律，则用户赢
        game_state.max_pebbles_per_turn
    } else {
        // 如果剩余的石子数量不是 (max_pebbles_per_turn + 1) 的倍数，则拿走target个石子，
        // 使剩余的石子数量变为 (max_pebbles_per_turn + 1) 的倍数
        target
    };  
    count
}

/// 移除石子
unsafe fn remove_pebbles(count: u32) -> Option<()>{
    let game_state = PEBBLES_GAME.as_mut().expect("Failed to load game state");

    if count == 0 {
        debug!("用户输入的石子数量不能为0，用户需要重新输入");
        return None; 
    }

    if count > game_state.max_pebbles_per_turn {
        debug!("用户输入的石子数量不能大于最大可拿的石子数，用户需要重新输入");
        return None;
    }
    if count > game_state.pebbles_remaining {
        debug!("用户输入的石子数量不能大于剩余的石子数，用户需要重新输入");
        return None;
    }
    game_state.pebbles_remaining -= count;

    Some(())
}

fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}