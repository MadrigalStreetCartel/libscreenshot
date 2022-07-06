use std::time::Instant;

use libscreenshot::prelude::*;

fn main() {
    let provider = libscreenshot::get_window_capture_provider().expect("Unable to find provider");
    let start_time = Instant::now();
    let target_runs = 10000;

    let mut time = start_time.clone();
    let mut runs = 0;
    let mut crashes = 0;

    for i in 1..=target_runs {
        let now = Instant::now();
        let diff = now.duration_since(time).as_millis();
        if diff > 1000 {
            let runtime = now.duration_since(start_time).as_secs();
            println!(
                "[libscreenshot/x11-stresstest] runtime: {}s; fps: {}; total: {}/{}", 
                runtime, runs, i, target_runs
            );
            time = now;
            runs = 0;
        }
        match provider.capture_focused_window() {
            Ok(_) => (),
            Err(_) => crashes += 1,
        }
        runs += 1;
    }

    println!("[libscreenshot/x11-stresstest] SUMMARY");
    println!("[libscreenshot/x11-stresstest] | Total runtime: {}s", Instant::now().duration_since(start_time).as_secs());
    println!("[libscreenshot/x11-stresstest] | Crashes: {} ({:.02}%)", crashes, (crashes as f32 / target_runs as f32) * 100_f32);
}
