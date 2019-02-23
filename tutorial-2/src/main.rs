extern crate gstreamer as gst;
use gst::prelude::*;

fn main() {
    gst::init().expect("Tienes algo raro en tu instalación de GStreamer");

    let element =
        gst::ElementFactory::make("fakesrc", "source").expect("No se pudo instanciar 'fakesrc'");

    // Soy también un GObject!
    let name = element
        .get_property("name")
        .expect("el element no tiene la propiedad 'name'");

    // Y hablo con GValues!
    println!(
        "El nombre del elemento es '{}'",
        name.get::<String>().unwrap()
    );
}
