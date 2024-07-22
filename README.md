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

## 4. 测试
Cargo.toml 配置了3个 features：
1. prod：用于生产环境，这也是默认的feature； 
2. test_user_first：用于测试用户先手； 
3. test_program_first：用于测试程序先手。


在 tests/basic.rs 中，提供了4个测试用例，分别测试用户先手和程序先手的情况。分别是：
1. test_handle_user_first_easy(): 测试用户先手，简单难度；对应的测试命令是：
    - cargo test test_handle_user_first_easy --no-default-features --features test_user_first
2. test_handle_program_first_easy()：测试程序先手，简单难度；对应的测试命令是：
    - cargo test test_handle_program_first_easy --no-default-features --features test_program_first
3. test_handle_user_first_hard()：测试用户先手，困难难度；对应的测试命令是：
    - cargo test test_handle_user_first_hard --no-default-features --features test_user_first
4. test_handle_program_first_hard()：测试程序先手，困难难度；对应的测试命令是：
    - cargo test test_handle_program_first_hard --no-default-features --features test_program_first

以上4个测试用例如果不指定特性，则默认为 prod。依然可以测试通过，但是谁先手，由随机决定。
如果不想一个一个测试，建议分别执行以下命令，可以覆盖所有情况：
1. cargo test：默认启用prod特性，谁先手随机。
2. cargo test --no-default-features --features test_user_first：启用test_user_first特性，用户先手。但test_handle_program_first_easy() 和 test_handle_program_first_hard() 实际先手为用户。
3. cargo test --no-default-features --features test_program_first：启用test_program_first特性，程序先手。但test_handle_user_first_easy() 和 test_handle_user_first_hard() 实际先手为程序。

## 4. 其他
游戏中设有两种难度级别：DifficultyLevel::Easy和DifficultyLevel::Hard。在简单难度下，程序应当随机选择要移除的石子数量；而在困难难度下，程序需要找出最优的石子数量移除。

具体思路是：
只要当程序移除石子后，剩余的石子数量始终为所允许拿走的最大石子数 + 1 (max_pebbles_per_turn + 1)，那么最后无论用户拿走多少，最终胜利的一定是程序。
因此，程序应尽量让剩余的石子数量变为 (max_pebbles_per_turn + 1) 的倍数，
如果做不到，程序应尽可能多地拿走石子，但不能超过 max_pebbles_per_turn。