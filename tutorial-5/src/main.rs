extern crate gstreamer as gst;
use gst::prelude::*;

fn main() {
    gst::init().expect("Tienes algo raro en tu instalación de GStreamer");

    let pipeline =
        gst::parse_launch(&format!("videotestsrc num-buffers={} ! autovideosink", 100)).unwrap();

    // Play
    pipeline
        .set_state(gst::State::Playing)
        .expect("Falló el pipeline al ponerlo en estado `Playing`");

    // Esperamos al end-of-stream
    let bus = pipeline.get_bus().unwrap();
    for msg in bus.iter_timed(gst::CLOCK_TIME_NONE) {
        match msg.view() {
            gst::MessageView::Eos(..) => break,
            gst::MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                break;
            }
            _ => (),
        }
    }

    // Null
    pipeline
        .set_state(gst::State::Null)
        .expect("Falló el pipeline al ponerlo en estado `Null`");
}
