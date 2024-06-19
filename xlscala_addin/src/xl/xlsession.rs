use crate::semantics::cause::Cause;
use crate::xlsdk::variant::Variant;
use crate::xl::xlregistry::XLRegistry;
use crate::xlsdk::xlsdk::XLSDK;
use crate::xl::error::*;
use crate::xl::scalar::{Handle, XLHandle};
use crate::xl::tracer::Tracer;
use crate::xl::xlobjecthandler::{XLObjectHandler};
use crate::xlsdk::xlfn::Xlfn;

mod details {

    use macros::SingletonInstance;
    use crate::semantics::cause::Cause;
    use crate::semantics::successful::successful;
    use crate::singleton::Singleton;
    use crate::xl::xlregistry::XLRegistry;
    use crate::xlsdk::xlfn::Xlfn;
    use crate::xlsdk::xlsdk::XLSDK;
    use crate::xl::error::*;
    use crate::xl::xlobjecthandler::{XLObjectHandler};
    use crate::xlsdk::variant::Variant;

    #[derive(SingletonInstance)]
    pub struct XLSession {
        path: String,
        registry: XLRegistry,
        object_handler: XLObjectHandler,
        //commands: Mutex<Vec<(XLRef, Box<dyn XLCommand>)>>,
        frozen: bool,
        calc_settings: i32,

        //current_commands: LinkedList<XLCommand>,
    }

    impl Singleton for XLSession {
        fn initialize() -> Self {
            panic!("session cannot be initialized by calling initialize");
        }
    }

    impl XLSession {

        pub fn try_new_scoped_by_auto_open(registry: XLRegistry) -> Ergo<XLSession> {
            match XLSDK::api_call_no_arg(Xlfn::XllName) {
                Ok(ret_xll_name) => {
                    let xll_name = ret_xll_name.to_string();
                    let result = registry.register_scoped_by_auto_open(&xll_name);
                    match result {
                        Ok(()) =>  {
                            let session = XLSession {
                                path: xll_name,
                                registry: registry,
                                object_handler: XLObjectHandler::new(),
                                frozen: false,
                                calc_settings: -1
                            };
                            Ok(session)
                        },
                        Err(error) => Err(error)
                    }
                },
                Err(error) => { Err(Cause::from(error)) }
            }
        }

        #[inline(always)]
        pub fn path(&self) -> &String {
            &self.path
        }

        #[inline(always)]
        pub fn object_handler(&mut self) -> &mut XLObjectHandler {
            &mut self.object_handler
        }

        pub fn freeze(&mut self, display: bool) -> Ergo<()> {
            XLSDK::api_call(Xlfn::Echo, vec![Variant::from(false)].as_mut_slice())?;
            if !self.frozen {
                let result = XLSDK::api_call(Xlfn::GetDocument, vec![Variant::from(14)].as_mut_slice())?;
                let calc_settings = f64::from(&result) as i32;
                XLSDK::api_call(Xlfn::Calculation, vec![Variant::from(3)].as_mut_slice())?;
                self.frozen = true;
                self.calc_settings = calc_settings;
            }
            successful::of::<Ergo<()>>()
        }


       pub fn unfreeze(&mut self) -> Ergo<()> {
           if self.frozen {
               XLSDK::api_call(Xlfn::Calculation, vec![Variant::from(self.calc_settings)].as_mut_slice())?;
               self.frozen = false;
           }
           successful::of::<Ergo<()>>()
       }

    }

}

pub struct XLSession {}

impl XLSession {

    pub fn try_init_scoped_by_auto_open(registry: XLRegistry) -> Ergo<()> {
        match details::XLSession::try_new_scoped_by_auto_open(registry) {
            Ok(session) => {
                details::XLSession::object_initialize(session);
                Ok(())
            },
            Err(error) =>  Err(error)
        }
    }

    pub fn api_call(xlfn: Xlfn, opers: &mut [Variant]) -> Ergo<Variant> {
        Tracer::debug(&format!("XLSDK api call {}, args: {:?}", xlfn, opers));
        match XLSDK::api_call(xlfn, &mut []) {
            Ok(variant) => Ok(variant),
            Err(error) => {
                Tracer::fail(&format!("XLSDK api call {} failed with error: {}, and args: {:?}", xlfn, error, opers));
                Err(Cause::from(error))
            }
        }
    }

    pub fn api_call_no_arg(xlfn: Xlfn) -> Ergo<Variant> {
        Tracer::debug(&format!("XLSDK api call {}, no args", xlfn));
        match XLSDK::api_call_no_arg(xlfn) {
            Ok(variant) => Ok(variant),
            Err(error) => {
                Tracer::fail(&format!("XLSDK api call {} failed, no args, error: {}", xlfn, error));
                Err(Cause::from(error))
            }
        }
    }


    #[inline(always)]
    pub fn path() -> String {
        details::XLSession::object().path().clone()
    }

    #[inline(always)]
    pub fn config_path() -> String {
        XLSession::path() + ".ini.toml"
    }

    #[inline(always)]
    pub fn from_handle_to_xlhandle(handle: Box<Handle>, page: u32, x:u32, y: u32) -> Ergo<XLHandle> {
        let mut oh = details::XLSession::object().object_handler();
        oh.from_handle_to_xlhandle(handle, page, x, y)
    }

    #[inline(always)]
    pub fn from_xlhandle_to_handle(xl_handle: &XLHandle) -> Ergo<Box<Handle>> {
        details::XLSession::object().object_handler().from_xlhandle_to_handle(xl_handle)
    }


    #[inline(always)]
    pub fn freeze(display: bool) -> Ergo<()> {
        details::XLSession::object().freeze(display)
    }

    #[inline(always)]
    pub fn unfreeze() -> Ergo<()> {
        details::XLSession::object().unfreeze()
    }

    #[inline(always)]
    pub fn on_recalc() {
    }

    /*pub fn add_command(&mut self, xlref: XLRef, command: Box<dyn XLCommand>) {
        self.commands.lock().unwrap().push((xlref, command));
    }*/

    /*fn register_events(&self) {
        let events = [(xlcOnRecalc, "xlOnRecalc")]; // , "xlOnTime", "xlOnHideProtect", "xlOnUnhideProtec", "xlOnSheet", "xlOnOpen", "xlOnWindow", "xlOnEntry", "xlOnDel", "xlOnDC", ];
        let xll_name = Xll::instance().path();
        for  (event, func,) in events.iter() { ;
            let mut opers = vec![
                Variant::from(xll_name.clone()),
                Variant::from("xlOnRecalc"),
                Variant::from("A#"),
                Variant::from("xlOnRecalc"),
                Variant::from(""),
                Variant::from(2),
                Variant::from("xlscala[events]"),
                Variant::missing(),
                Variant::missing(),
                Variant::from(""),
            ];
            let result = XLSDK::api_call(xlfRegister, opers.as_mut_slice());
            Logger::debug(&format!("Registered event handler {}: result = {}", func, result));
            let result = XLSDK::api_call(xlcOnRecalc, vec![Variant::missing(), Variant::from("xlOnRecalc")].as_mut_slice());
            Logger::debug(&format!("Attached event handler {}: result = {}", func, result));
        }
    }*/

    /*pub fn freeze(display: bool) -> Report {
        XLSession::api_call(Xlfn::Echo, vec![Variant::from(false)].as_mut_slice())?;
        if (!self.frozen) {
            let result = self.api_call(Xlfn::GetDocument, vec![Variant::from(14)].as_mut_slice())?;
            let calc_settings = f64::from(&result) as i32;
            self.api_call(Xlfn::Calculation, vec![Variant::from(3)].as_mut_slice())?;
            self.frozen = true;
            self.calc_settings = calc_settings;
        }
        Validated::successful()
    }

    pub fn unfreeze(&mut self) -> Validated {
        if(self.frozen) {
            self.api_call(Xlfn::Calculation, vec![Variant::from(self.calc_settings)].as_mut_slice())?;
            self.frozen = false;
        }
        Validated::successful()
    }*/

    /*pub fn on_recalc(&mut self) {
        self.freeze(false);
        {
            let mut cmds = self.commands.lock().unwrap();
            //Logger::info("on_recalc".to_string());
            for (xlref, cmd) in cmds.iter() {
                //Logger::info("on_recalc1".to_string());
                cmd.execute(xlref);
                //Logger::info("on_recalc2".to_string());
            }
            cmds.clear();
        }
        self.unfreeze();

        /*Logger::info("on_recalc");
        Logger::info("on_recalc");
        Logger::info("on_recalc");
        Logger::info("on_recalc");
        Logger::info("on_recalc");*/

    }*/

    /*pub fn add_event(mut self, new_event: XLEvent) {
        self.guard.lock().unwrap();
        self.registered_events.push(new_event);
    }*/

}