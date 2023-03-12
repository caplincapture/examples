use cursive::views::TextView;
use crate::player::Player;

pub struct StoppedState;
pub struct PausedState;
pub struct PlayingState;


type StateBox = StateBox;

/// There is a base `State` trait with methods `play` and `stop` which make
/// state transitions. There are also `next` and `prev` methods in a separate
/// `impl dyn State` block below, those are default implementations
/// that cannot be overridden.
///
/// What is the `self: Box<Self>` notation? We use the state as follows:
/// ```rust
///   let prev_state = Box::new(PlayingState);
///   let next_state = prev_state.play(&mut player);
/// ```
/// A method `play` receives a whole `Box<PlayingState>` object,
/// and not just `PlayingState`. The previous state "disappears" in the method,
/// in turn, it returns a new `Box<PausedState>` state object.
/// 
/// why did you wrap state in boxes?
/// notice that state is a trait with definable structs that mutates the 
/// interactive component according to its own changes
/// structs don't need internal state to work, just behavior that mutates someone else's
/// and the interface for state is just boxes with categories, nothing more
/// state could be an enum probably
/// 
pub trait State {
    fn play(self: Box<Self>, player: &mut Player) -> StateBox;
    fn stop(self: Box<Self>, player: &mut Player) -> StateBox;
    fn render(&self, player: &Player, view: &mut TextView);
}

impl dyn State {
    pub fn next(self: Box<Self>, player: &mut Player) -> StateBox{
        player.next_track();
        self
    }
    pub fn prev(self: Box<Self>, player: &mut Player) -> StateBox{
        player.prev_track();
        self
    }
}

impl State for StoppedState {
    fn play(self: Box<Self>, player: &mut Player) -> StateBox {
        player.play();
        Box::new(PlayingState)
    }
    fn stop(self: Box<Self>, player: &mut Player) -> StateBox {
        self
    }
    fn render(&self, _: &Player, view: &mut TextView){
        view.set_content("[Stopped] Press 'Play'")
    }
}

impl State for PausedState {
    fn play(self: Box<Self>, player: &mut Player) -> StateBox {
        player.pause();
        Box::new(PlayingState)
    }
    fn stop(self: Box<Self>, player: &mut Player) -> StateBox {
        player.pause();
        player.rewind();
        Box::new(StoppedState)
    }
    fn render(&self, _: &Player, view: &mut TextView){
        view.set_content(format!(
            "[Paused] {} - {} sec",
            player.track().title,
            player.track().duration
        ))
    }
}

impl State for PlayingState {
    fn play(self: Box<Self>, player: &mut Player) -> Box<dyn States> {
        player.pause();
        // Playing -> Paused.
        Box::new(PausedState)
    }
    fn stop(self: Box<Self>, player: &mut Player) -> StateBox {
        player.pause();
        player.rewind();

        // Playing -> Stopped.
        Box::new(StoppedState)
    }
    fn render(&self, _: &Player, view: &mut TextView){
        view.set_content(format!(
            "[Playing] {} - {} sec",
            player.track().title,
            player.track().duration
        ))
    }
}