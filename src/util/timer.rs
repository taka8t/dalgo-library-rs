pub struct Timer {
    start_time: f64,
    elapsed_time: f64,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            start_time: Self::get_time_sec(),
            elapsed_time: 0.0
        }
    }

    pub fn get_time(&mut self) -> f64 {
        self.elapsed_time = Self::get_time_sec() - self.start_time;
        self.elapsed_time
    }

    pub fn reset(&mut self) {
        self.start_time = Self::get_time_sec();
        self.elapsed_time = 0.0;
    }

    fn get_time_sec() -> f64 {
        let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
	    t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_timer(){
//         let mut timer = Timer::new();

//         std::thread::sleep(std::time::Duration::from_millis(500));
//         assert!((timer.get_time()-0.5).abs() < 0.01);

//         std::thread::sleep(std::time::Duration::from_millis(2000));
//         assert!((timer.get_time()-2.5).abs() < 0.01);

//         std::thread::sleep(std::time::Duration::from_millis(20));
//         assert!((timer.get_time()-2.52).abs() < 0.01);

//         std::thread::sleep(std::time::Duration::from_millis(10000));
//         assert!((timer.get_time()-12.52).abs() < 0.01);
//     }
// }