extern crate gstreamer as gst;
use gst::prelude::*;

fn main() {
    gst::init().expect("Tienes algo raro en tu instalación de GStreamer");

    let factory = gst::ElementFactory::find("fakesrc").expect("No se pudo instanciar 'fakesrc'");

    println!(
        "La fábrica '{}' es de la categoría {}.\nDescripción: \"{}\"",
        factory.get_plugin_name().unwrap(),
        factory.get_metadata(&gst::ELEMENT_METADATA_KLASS).unwrap(),
        factory
            .get_metadata(&gst::ELEMENT_METADATA_DESCRIPTION)
            .unwrap()
    );
}
