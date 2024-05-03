// use rustream::m3u::parse_m3u;
// use rustream::params::{parse_args, Params};
// use rustream::types::Result;

// fn run() -> Result<()> {
//     let params: Params = parse_args(std::env::args())?;
//     let channels = parse_m3u(&params.m3u_filepath)?;

//     println!("{} channels found", channels.len());

//     Ok(())
// }

// fn main() {
//     if let Err(e) = run() {
//         eprintln!("Error: {}", e);
//         std::process::exit(1);
//     }
// }

use gst::prelude::*;

fn main() {
    gst::init().unwrap();

    let pipeline = gst::parse_launch("playbin uri=link").unwrap();

    pipeline.set_state(gst::State::Playing).unwrap();

    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;
        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null).unwrap();
}
