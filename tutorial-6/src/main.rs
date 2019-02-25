extern crate glib;
extern crate gstreamer as gst;
use gst::prelude::*;

extern crate failure;
use failure::Error;

fn run(location: &str) -> Result<(), Error> {
    gst::init()?;

    let pipeline = gst::parse_launch(&"playbin")?;
    pipeline.set_property("uri", &location.to_value())?;
    pipeline.set_state(gst::State::Playing)?;

    let main_loop = glib::MainLoop::new(None, false);
    let main_loop_clone = main_loop.clone();
    let pipeline_weak = pipeline.downgrade();
    let bus = pipeline.get_bus().expect("Pipeline sin bus!");
    bus.add_watch(move |_, msg| {
        let pipeline = match pipeline_weak.upgrade() {
            Some(pipeline) => pipeline,
            None => return glib::Continue(true),
        };

        let main_loop = &main_loop_clone;
        match msg.view() {
            gst::MessageView::Error(err) => {
                eprintln!(
                    "El elmento {:?} mandó el error {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                let _ = pipeline.set_state(gst::State::Ready);
                main_loop.quit();
            }
            gst::MessageView::Eos(..) => {
                let _ = pipeline.set_state(gst::State::Ready);
                main_loop.quit();
            }
            _ => {}
        }

        glib::Continue(true)
    });

    main_loop.run();

    bus.remove_watch()?;
    pipeline.set_state(gst::State::Null)?;

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Te falta poner el URI a reproducir");
        return;
    }

    match run(&args[1]) {
        Ok(_) => {}
        Err(err) => eprintln!("Falló el coso: {}", err),
    }
}
