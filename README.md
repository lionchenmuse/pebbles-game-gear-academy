# pebbles-game-gear-academy
## 1. 编写一款名为“石子游戏”的程序。游戏的规则如下：
• 游戏中有两名玩家：用户和程序。首先行动的玩家由随机方式决定。
• 游戏开始时有N颗石子（例如，N=15）。
• 在玩家的回合中，他们必须移除从1到K颗石子（例如，如果K=2，那么玩家每回合可以移除1或2颗石子）。
• 最后取走石子的玩家获胜。

## 2. 项目架构
构建两个crate：一个是pebbles-game，用于游戏逻辑的实现；另一个是pebbles-game-io，用于数据结构的定义和管理。这两个组件共同构成了游戏的完整架构。
```Shell
pebbles-game
    ├── io
    │   ├── src
    │   │   └── lib.rs
    │   └── Cargo.toml
    ├── src
    │   └── lib.rs
    ├── tests
    │   └── basic.rs
    ├── Cargo.lock
    ├── Cargo.toml
    └── build.rs
```

## 3. 实现 
1. init()函数，该函数需要： 
• 通过msg::load函数加载PebblesInit类型的初始化信息； 
• 验证输入数据的正确性； 
• 利用exec::random函数随机决定首位玩家； 
• 如果首位玩家是程序，处理第一个游戏回合； 
• 完善GameState结构体的填充。

2. handle()函数，其功能包括： 
• 通过msg::load函数读取PebblesAction类型的动作指令； 
• 校验输入数据的有效性； 
• 执行用户的回合，并判断用户是否赢得比赛； 
• 进行程序的回合，并判断程序是否获胜； 
• 向用户发送与之对应的PebblesEvent事件消息；

3. state()函数，其作用是： 
• 利用msg::reply函数返回当前的GameState结构体内容。

## 4. 其他
游戏中设有两种难度级别：DifficultyLevel::Easy和DifficultyLevel::Hard。在简单难度下，程序应当随机选择要移除的石子数量；而在困难难度下，程序需要找出最优的石子数量移除。

具体思路是：
只要当程序移除石子后，剩余的石子数量始终为所允许拿走的最大石子数 + 1 (max_pebbles_per_turn + 1)，那么最后无论用户拿走多少，最终胜利的一定是程序。
因此，程序应尽量让剩余的石子数量变为 (max_pebbles_per_turn + 1) 的倍数，
如果做不到，程序应尽可能多地拿走石子，但不能超过 max_pebbles_per_turn。