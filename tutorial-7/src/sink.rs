use glib;
use glib::subclass;
use glib::subclass::prelude::*;
use gst;
use gst::prelude::*;
use gst::subclass::prelude::*;
use gst_video;

use std::sync::Mutex;

#[derive(Debug, Clone)]
struct Settings {
    sink: Option<gst::Element>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            sink: None::<gst::Element>,
        }
    }
}

static PROPERTIES: [subclass::Property; 1] = [subclass::Property("sink", |name| {
    glib::ParamSpec::object(
        name,
        "sink",
        "The video sink chain to use",
        gst::Element::static_type(),
        glib::ParamFlags::READWRITE,
    )
})];

struct ServoSink {
    cat: gst::DebugCategory,
    sinkpad: gst::GhostPad,
    settings: Mutex<Settings>,
}

impl ServoSink {
    fn set_sink(
        &self,
        bin: &gst::Bin,
        sink: Option<gst::Element>,
    ) -> Result<(), glib::error::BoolError> {
        let mut settings = self.settings.lock().unwrap();

        if let Some(ref cur_sink) = settings.sink {
            cur_sink.set_locked_state(true);
            self.sinkpad.set_target(None::<&gst::Pad>)?;
            bin.remove(cur_sink)?;
        }

        settings.sink = sink;

        if let Some(ref new_sink) = settings.sink {
            new_sink.set_name("sink")?;
            bin.add(new_sink)?;

            let target_pad = new_sink
                .get_static_pad("sink")
                .ok_or_else(|| glib_bool_error!("Can't find static sink pad"))?;
            self.sinkpad.set_target(Some(&target_pad))?;
        }

        Ok(())
    }

    fn query(&self, pad: &gst::GhostPad, parent: &gst::Element, query: &mut gst::QueryRef) -> bool {
        gst_log!(self.cat, obj: pad, "Handling query {:?}", query);
        let ret = match query.view_mut() {
            gst::QueryView::Allocation(ref mut q) => {
                q.add_allocation_meta::<gst_video::VideoMeta>(None);
                true
            }
            _ => pad.query_default(Some(parent), query),
        };

        if ret {
            gst_log!(self.cat, obj: pad, "Handled query {:?}", query);
        } else {
            gst_info!(self.cat, obj: pad, "Didn't handle query {:?}", query);
        }
        ret
    }
}

impl ObjectSubclass for ServoSink {
    const NAME: &'static str = "ServoSink";
    type ParentType = gst::Bin;
    type Instance = gst::subclass::ElementInstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;

    glib_object_subclass!();

    fn class_init(klass: &mut subclass::simple::ClassStruct<Self>) {
        klass.set_metadata(
            "Servo Sink",
            "Sink/Video",
            "Receives video frames for Servo",
            "Servo developers",
        );

        let caps = gst::Caps::builder("video/x-raw")
            .field("format", &gst_video::VideoFormat::Bgra.to_string())
            .field("pixel-aspect-ratio", &gst::Fraction::from((1, 1)))
            .build();

        let sink_pad_template = gst::PadTemplate::new(
            "sink",
            gst::PadDirection::Sink,
            gst::PadPresence::Always,
            &caps,
        )
        .unwrap();
        klass.add_pad_template(sink_pad_template);

        klass.install_properties(&PROPERTIES);
    }

    fn new_with_class(klass: &subclass::simple::ClassStruct<Self>) -> Self {
        let pad_templ = klass.get_pad_template("sink").unwrap();
        let ghost_pad = gst::GhostPad::new_no_target_from_template("sink", &pad_templ).unwrap();

        ghost_pad.set_query_function(|pad, parent, query| {
            ServoSink::catch_panic_pad_function(
                parent,
                || false,
                |servosink, element| servosink.query(pad, element, query),
            )
        });

        Self {
            cat: gst::DebugCategory::new("servosink", gst::DebugColorFlags::empty(), "Servo Sink"),
            sinkpad: ghost_pad,
            settings: Mutex::new(Settings::default()),
        }
    }
}

impl ObjectImpl for ServoSink {
    glib_object_impl!();

    fn set_property(&self, obj: &glib::Object, id: usize, value: &glib::Value) {
        let prop = &PROPERTIES[id];
        let bin = obj.downcast_ref::<gst::Bin>().unwrap();

        match *prop {
            subclass::Property("sink", ..) => {
                self.set_sink(bin, value.get()).unwrap();
            }
            _ => unimplemented!(),
        }
    }

    fn get_property(&self, _: &glib::Object, id: usize) -> Result<glib::Value, ()> {
        let prop = &PROPERTIES[id];
        match *prop {
            subclass::Property("sink", ..) => {
                let settings = self.settings.lock().unwrap();
                Ok(settings.sink.to_value())
            }
            _ => unimplemented!(),
        }
    }
}

impl ElementImpl for ServoSink {
    fn change_state(
        &self,
        element: &gst::Element,
        transition: gst::StateChange,
    ) -> Result<gst::StateChangeSuccess, gst::StateChangeError> {
        match transition {
            gst::StateChange::NullToReady => {
                let settings = self.settings.lock().unwrap();
                if settings.sink.is_none() {
                    gst_error!(self.cat, obj: element, "No sink assigned");
                    return Err(gst::StateChangeError);
                }
            }
            _ => (),
        }

        self.parent_change_state(element, transition)
    }
}

impl BinImpl for ServoSink {}

pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(plugin, "servosink", 0, ServoSink::get_type())
}
