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
            levels: [LEVELS - 16, LEVELS - 16, LEVELS - 1], //what should they actually be
            frame_rate: 100, //is this a leading or trailing inicator
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
            //detect if change happened to button
            let buttona_level: Level = self._button_a.get_level();
            let buttonb_level: Level = self._button_b.get_level();
            let level = self.knob.measure().await; //are we catching the await wrong
            rprintln!("knob level is {}", level); //knob still pissy
            //does this even know how to detect or describe change in the knob?
            //oh how does the knob level and the led levels play together (measured at the same time)
            // detect for knob change
            // check/restrict for max 10::160 
            if buttona_level == Level::Low && buttonb_level == Level::Low {
                // Change the frame rate in steps of 10 frames per second from 10..160.
                // if self.state.frame_rate and {+/- 10} by each stop
                // rgb.frame_tick_time
            } 
            if  buttona_level == Level::High && buttonb_level == Level::High {
                //  Change the red level from off on over 16 steps.
                // self.state.levels[0]
            }
            if buttona_level == Level::Low {
                rprintln!("button A read as low");
                //button a is which color?
                //detect for whether the dial changes
            }
            if buttonb_level == Level::Low {
                rprintln!("button B read as low");
                //hang the  green level from off to on over 16 steps.
                // self.state.levels[2]
            }            
            //it hsould be able ot handle different changes in level why does it crash here does it crash ehere
            if level != self.state.levels[2] { //which level is even being cheked here?
                self.state.levels[2] = level;
                self.state.show();
                set_rgb_levels(|rgb| { //wait rgb stuff is set here too
                    *rgb = self.state.levels;
                })
                .await;
            }
            Timer::after_millis(50).await;
        }
    }
}
