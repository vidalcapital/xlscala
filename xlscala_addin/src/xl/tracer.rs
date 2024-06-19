
use crate::singleton::Singleton;


pub enum TraceKind {
    Debug,
    Info,
    Fail
}

pub struct Trace {
    pub(crate) kind: TraceKind,
    pub(crate) msg: String
}

impl Trace {
    pub fn new(kind: TraceKind, msg: &String) -> Trace {
        Trace {
            kind: kind,
            msg: msg.clone()
        }
    }
}


mod details {
    use circular_buffer::CircularBuffer;
    use macros::SingletonInstance;
    use crate::singleton::Singleton;
    use crate::xl::tracer::Trace;

    pub const TRACER_MAX_LINES: usize = 1024;

    #[derive(SingletonInstance)]
    pub struct Tracer {
        traces: CircularBuffer::<TRACER_MAX_LINES, Trace>
    }

    impl Singleton for Tracer {
        fn initialize() -> Self {
            Tracer {
                traces: CircularBuffer::<TRACER_MAX_LINES, Trace>::new()
            }
        }
    }

    impl Tracer {
        pub fn trace(&mut self, trace: Trace) {
            self.traces.push_back(trace)
        }
    }
}

pub struct Tracer;

impl Tracer {
    #[inline(always)]
    pub fn initialize() {
        details::Tracer::object_initialize (details::Tracer::initialize());
        ()
    }


    #[inline(always)]
    pub fn info(message: &String) {
        details::Tracer::object().trace(Trace::new(TraceKind::Info, message))
    }

    #[inline(always)]
    pub(crate) fn debug(message: &String) {
        details::Tracer::object().trace(Trace::new(TraceKind::Debug, message))
    }

    #[inline(always)]
    pub(crate) fn fail(message: &String) {
        details::Tracer::object().trace(Trace::new(TraceKind::Fail, message))
    }
}