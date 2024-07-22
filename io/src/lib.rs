#![no_std]

use gmeta::{In, InOut, Out, Metadata};
use gstd::prelude::*;

pub struct PebblesMetadata;

impl Metadata for PebblesMetadata {
    type Init = In<PebblesInit>;
    type Handle = InOut<PebblesAction, PebblesEvent>;
    type State = Out<GameState>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}

/// 游戏初始化参数
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct PebblesInit {
    /// 难度
    pub difficulty: DifficultyLevel,
    /// 石子总数
    pub pebbles_count: u32,
    /// 每回合最大可拿的石子数
    pub max_pebbles_per_turn: u32,
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub enum DifficultyLevel {
    #[default]
    Easy,
    Hard,
}

/// 用户可以采取的动作
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum PebblesAction {
    /// 回合，及拿走的石子数量
    Turn(u32),
    /// 放弃
    GiveUp,
    /// 重新开始
    Restart {
        difficulty: DifficultyLevel,
        pebbles_count: u32,
        max_pebbles_per_turn: u32,
    },
}

/// 游戏事件
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum PebblesEvent {
    /// 玩家（Program）移除的石子总数
    CounterTurn(u32),
    /// 游戏结束，及赢家
    Won(Player),
}

/// 玩家
#[derive(Debug, Default, Clone, Copy, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum Player {
    /// 用户
    #[default]
    User,
    /// 程序
    Program,
}

/// 
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct GameState {
    /// 石子总数
    pub pebbles_count: u32,
    /// 每回合最大可拿的石子数
    pub max_pebbles_per_turn: u32,
    /// 剩余石子数
    pub pebbles_remaining: u32,
    /// 难度
    pub difficulty: DifficultyLevel,
    /// 先手
    pub first_player: Player,
    /// 赢家
    pub winner: Option<Player>,
}

impl GameState {
    /// 初始化游戏状态
    pub fn init(init: PebblesInit, first_player: Player) -> Self {
        GameState {
            pebbles_count: init.pebbles_count,
            max_pebbles_per_turn: init.max_pebbles_per_turn,
            pebbles_remaining: init.pebbles_count,
            difficulty: init.difficulty,
            first_player: first_player,
            winner: None,
        }
    }
    pub fn set_winner(&mut self, winner: Player) {
        self.winner = Some(winner);
    }
}