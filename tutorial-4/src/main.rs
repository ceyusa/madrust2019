extern crate gstreamer as gst;
use gst::prelude::*;
use std::{thread, time};

fn main() {
    gst::init().expect("Tienes algo raro en tu instalación de GStreamer");

    let source = gst::ElementFactory::make("videotestsrc", "source")
        .expect("No se pudo crear 'videotestsrc'");
    let filter =
        gst::ElementFactory::make("identity", "filter").expect("No se pudo crear 'identity'");
    let sink = gst::ElementFactory::make("autovideosink", "sink")
        .expect("No se pudo crear 'autovideosink'");

    // Mi primer pipeline!
    let pipeline = gst::Pipeline::new("prueba");

    // Metemos los elementos en el bin antes de vincularlos
    pipeline.add_many(&[&source, &filter, &sink]).unwrap();

    // Definimos nuestro grafo dirigido
    gst::Element::link_many(&[&source, &filter, &sink]).unwrap();

    // Play
    pipeline
        .set_state(gst::State::Playing)
        .expect("Falló el pipeline al ponerlo en estado `Playing`");

    // Esperamos 5 sec
    thread::sleep(time::Duration::new(5, 0));

    // Null
    pipeline
        .set_state(gst::State::Null)
        .expect("Falló el pipeline al ponerlo en estado `Null`");
}
