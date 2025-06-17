use crate::*;

struct UiState {
    levels: [u32; 3],
    frame_rate: u64,
}

impl UiState {
    fn show(&self) {
        let names = ["red", "green", "blue"];
        rprintln!();
        for (name, level) in names.iter().zip(self.levels.iter()) {
            rprintln!("{}: {}", name, level);
        }
        rprintln!("frame rate: {}", self.frame_rate);
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            levels: [LEVELS - 16, LEVELS - 16, LEVELS - 16],
            frame_rate: 100,
        }
    }
}

pub struct Ui {
    knob: Knob,
    _button_a: Button,
    _button_b: Button,
    state: UiState,
}

impl Ui {
    pub fn new(knob: Knob, _button_a: Button, _button_b: Button) -> Self {
        Self {
            knob,
            _button_a,
            _button_b,
            state: UiState::default(),
        }
    }

    pub async fn run(&mut self) -> ! {
        self.state.levels[2] = self.knob.measure().await; //wait what is this color why is this color, where do we want to set framerate
        set_rgb_levels(|rgb| {
            *rgb = self.state.levels;
        })
        .await;
        self.state.show();
        loop {
            let buttona_level: Level = self._button_a.get_level();
            let buttonb_level: Level = self._button_b.get_level();
            let level = self.knob.measure().await;

            //if no buttons held (frame rate)
            if  buttona_level == Level::High && buttonb_level == Level::High {
                // Change the frame rate in steps of 10 frames per second from 10..160.
                self.state.frame_rate = level as u64;
                set_framerate( |frame_rate| {
                    *frame_rate = self.state.frame_rate;
                })
                .await;
                self.state.show();
                // i do not see my frame rate adjusting stuff strongly yet
            //thought 1: write a global setter
            //thought 2: trace self.state.frame_rate and see if we can touch it from rgb.rs
            }

            //thought on manipulating frame rate is hitting the rgb::frame_tick_time with the modified frame_rate leve, but frame_tick_time is only called in new()
            //oh wait how do we set rgb::tick_time

            //if only button A is held (BLUE)
            if buttona_level == Level::Low && buttonb_level == Level::High {
            //   Change the BLUE level from off on over 16 steps.
                // TODO: 16 steps
            //i wonder if we can just leave this outside of the conditional, but this feels good for now
                self.state.levels[2] = level;
                set_rgb_levels(|rgb| {
                    *rgb = self.state.levels;
                })
                .await;
                
            }
            //if only button b is held (green)
            if buttona_level == Level::High && buttonb_level == Level::Low {
                // change the green level from off on over 16 steps
                // TODO 16 STEPS STUFF
                self.state.levels[1] = level;
                set_rgb_levels( |rgb| {
                    *rgb = self.state.levels;
                })
                .await;
            }

             //if only button B and A are held (red)
             if buttona_level == Level::Low && buttonb_level == Level::Low {
                self.state.levels[0] = level;
                set_rgb_levels( |rgb| {
                    *rgb = self.state.levels;
                })
                .await;
            }
            self.state.show();
            Timer::after_millis(50).await;
        }
    }
}

