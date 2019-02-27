#![feature(nll)]

#[macro_use]
extern crate glib;
#[macro_use]
extern crate gstreamer as gst;
extern crate gstreamer_video as gst_video;

mod sink;

fn plugin_init(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    sink::register(plugin)?;
    Ok(())
}

gst_plugin_define!(
    "servosink",
    "Servo Video Sink",
    plugin_init,
    "1.0",
    "MIT/X11",
    "servosink",
    "servosink",
    "https://github.com/servo/media",
    "2019-02-26"
);
