use crate::*;

pub type Adc = saadc::Saadc<'static, 1>; //does it get instantiated correcly somehwere

pub struct Knob(Adc);

impl Knob {
    pub async fn new(adc: Adc) -> Self {
        adc.calibrate().await;
        Self(adc)
    }

    // `&mut [[[i16; 1]; _]; 2]`
    //mabye write your own sample clojure?

    pub async fn measure(&mut self) -> u32 {
        let mut buf = [0];
        // let mut buf : [[i16; 1]; 2] = [[0; 1]; 2];
        self.0.sample(&mut buf).await; //what's self.0 
        // let mut twobuf = [[0 as i16]; 2];
        // self.0.run_timer_sampler(buf, 4, self.0.sample(&mut buf).await);
        let raw = buf[0].clamp(0, 0x7fff) as u16; //what is 0x7fff on a u16? is it just the natrual max?
        let scaled = raw as f32 / 10_000.0; //what is sthis scaling for real
        let result = ((LEVELS + 2) as f32 * scaled - 2.0) //what's really hapepnignin in this math
            .clamp(0.0, (LEVELS - 1) as f32)
            .floor();
        result as u32
    }
}
