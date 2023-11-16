use std::env;

use glutin::event_loop::EventLoop;

const MANGOHUD_FONT_SIZE_RATION: u32 = 45;

fn get_env(var: &str) -> String {
    if let Ok(ret) = env::var(var)
        { ret } else { "".to_string() }
}

fn print_help() {
    println!("Usage: {} <arg>
    no arg            Show all info about monitors output
    -m|--mhfsize      Show MangoHud font size for primary monitor
    -x|--xrandr       Show monitors config for xrandr
    -h|--help         Show this usage info", env!("CARGO_BIN_NAME"));
}


fn main() {
    env::remove_var("WAYLAND_DISPLAY");
    let exec_args: Vec<String> = env::args().collect();
    let event_loop = EventLoop::new();

    let primary_monitor = event_loop.primary_monitor().unwrap();
    let primary_monitor_name = primary_monitor.name().unwrap();

    let mut mhfratio: u32 = MANGOHUD_FONT_SIZE_RATION;
    if let Ok(num) = get_env("MANGOHUD_FONT_SIZE_RATION").parse::<u32>() {
        mhfratio = num
    }

    match exec_args.len() {
        1 => {
            for (num, monitor) in event_loop.available_monitors().enumerate() {
                let monitor_name = monitor.name().unwrap();
                let monitor_size = monitor.size();
                let monitor_position = monitor.position();
                if num > 0 { println!("----------------------------") }
                if monitor_name == primary_monitor_name {
                    println!("Primary monitor name: {}", monitor_name)
                } else {
                    println!("Monitor name: {}", monitor_name)
                }
                println!("Mode: {}x{}", monitor_size.width, monitor_size.height);
                println!("Position: {}x{}", monitor_position.x, monitor_position.y);
                println!("Scale factor: {}", monitor.scale_factor());
                
                let video_mode = monitor.video_modes().next().unwrap();
                let video_mode_size = video_mode.size();
                println!("Resolution: {}x{}", video_mode_size.width, video_mode_size.height);
                println!("Refresh Rate: {} Hz", video_mode.refresh_rate());
                println!("Bit Depth: {}", video_mode.bit_depth());

                println!("MangoHud font size: {}", monitor_size.height / mhfratio);
            }
        },
        2 => {
            match exec_args[1].as_str() {
                "-m"|"--mhfsize" => {
                    println!("{}", primary_monitor.size().height / mhfratio)
                },
                "-x"|"--xrandr" => {
                    let mut config = "".to_string();
                    for monitor in event_loop.available_monitors() {
                        let monitor_name = monitor.name().unwrap();
                        let monitor_position = monitor.position();
                        let video_mode = monitor.video_modes().next().unwrap();
                        let video_mode_size = video_mode.size();
                        config.push_str("--output ");
                        if monitor_name == primary_monitor_name {
                            config.push_str(format!("{monitor_name} --primary ").as_str())
                        } else {
                            config.push_str(format!("{monitor_name} ").as_str())
                        }
                        config.push_str(
                            format!("--mode {}x{} --pos {}x{} --rate {} --rotate normal ",
                            video_mode_size.width, video_mode_size.height, 
                            monitor_position.x, monitor_position.y, 
                            video_mode.refresh_rate()).as_str()
                        );
                    }
                    println!("{config}");
                },
                "-h"|"--help" => {
                    print_help()
                },
                _ => {
                    eprintln!("Error: invalid argument!");
                    print_help()
                },
            }
        },
        _ => {
            eprintln!("Error: invalid argument!");
            print_help()
        }
    }
}
