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
                self.state.frame_rate = (level * 10) as u64;
                set_framerate( |frame_rate| {
                    *frame_rate = self.state.frame_rate;
                })
                .await;
                self.state.show();
            }

            //if only button A is held (BLUE)
            if buttona_level == Level::Low && buttonb_level == Level::High {
            //   Change the BLUE level from off on over 16 steps.
                // TODO: 16 steps, rn this is just doing this over whatever frametick time 
                self.state.levels[2] = level;
                //increment per frame until you hit hte target?, but this can change /constantly/ so do the changes to color /trail after the changes from knob?
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
                //TODO 16 STEPS STUFF
                self.state.levels[0] = level;
                set_rgb_levels( |rgb| {
                    *rgb = self.state.levels;
                })
                .await;
            }
            self.state.show();
            
            let time_frame = self.state.frame_rate.clamp(1,16);
            Timer::after_millis(time_frame).await; //Adjust Rgb delays according to the frame rate.
        }
    }
}
