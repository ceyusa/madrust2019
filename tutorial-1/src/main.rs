extern crate gstreamer as gst;

fn main() {
    gst::init().expect("Tienes algo raro en tu instalación de GStreamer");

    let (major, minor, micro, nano) = gst::version();
    println!(
        "La versión de GStreamer es {}.{}.{}-{}",
        major, minor, micro, nano
    );

    unsafe {
        gst::deinit();
    }
}
