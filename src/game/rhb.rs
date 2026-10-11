use crate::game::{
        Sheet,
        HtmlImageElement,
};

// RHB構造体
pub struct RedHatBoy {
    state_machine: RedHatBoyStateMachine,
    sprite_sheet: Sheet,
    image: HtmlImageElement,
}

impl RedHatBoy {
    pub fn new(sheet: Sheet, image: HtmlImageElement) -> Self {
        RedHatBoy {
            state_machine: RedHatBoyStateMachine::Idle(RedHatBoyState::new()),
            sprite_sheet: sheet,
            image,
        }
    }
}


// RHBステートマシン
#[derive(Copy, Clone)]
// RHBの状態を列挙
enum RedHatBoyStateMachine {
    Idle(RedHatBoyState<Idle>),
    Running(RedHatBoyState<Running>),
}

// RHBのイベント(Runのみ)
pub enum Event {
    Run,
}

// RHBの状態遷移イベント(Idle→Run→Running)
impl RedHatBoyStateMachine {
    // RHBの状態遷移
    fn transition(self, event: Event) -> Self {
        match (self, event) {
            // (RedHatBoyStateMachine::Idle(state), Event::Run) => {
            //     RedHatBoyStateMachine::Running(state.run())
            (RedHatBoyStateMachine::Idle(state), Event::Run) => state.run().into(),
            _ => self,
        }
    }
}

// Fromトレイト(From Runningステート to ステートマシン)
impl From<RedHatBoyState<Running>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Running>) -> Self {
        RedHatBoyStateMachine::Running(state)
    }
}

// RHBの状態を定義
mod red_hat_boy_states {
    use crate::engine::Point;
    const FLOOR: i16 =475;

    // RHBの状態そのものを定義
    #[derive(Copy,Clone)]
    pub struct RedHatBoyState<S> {
        context: RedHatBoyContext,
        _state: S,
    }

    // RHBの状態量の箱を定義
    #[derive(Copy,Clone)]
    pub struct RedHatBoyContext {
        frame: u8,
        position: Point,
        velocity: Point,
    }

    // RHBの状態遷移(Idleから)
    impl RedHatBoyState<Idle> {
        // 初期化
        pub fn new() -> Self { // private関数なのでエラー、後で処置する
            RedHatBoyState {
                context: RedHatBoyContext {
                    frame: 0,
                    position: Point { x: 0, y: FLOOR },
                    velocity: Point { x: 0, y: 0 },
                },
                _state: Idle {},
            }
        }

        // Idle → Running
        pub fn run(self) -> RedHatBoyState<Running> { // private関数なのでエラー、後で処置する
            RedHatBoyState {
                context: self.context,
                _state: Running {},
            }
        }
    }

    // RHBの状態(Idle)
    #[derive(Copy, Clone)]
    pub struct Idle;

    // RHBの状態(Running)
    #[derive(Copy, Clone)]
    pub struct Running;
}

use red_hat_boy_states::{
    RedHatBoyState,
    Idle,
    Running,
};