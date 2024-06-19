use crate::semantics::successful::*;
use crate::xl::range::Range;
use crate::xl::error::*;
use crate::xl::xlcommand::XLCommand;

pub struct Alert(String);

impl Alert {
    pub fn new(message: String) -> Alert {
        Alert(message)
    }
}


impl XLCommand for Alert {
    fn execute(&self, xlrange: &Range) -> Ergo<()> {
        /*XLSession::api_call(Xlfn::Message, vec![Variant::from(1), Variant::from("SDFDSFSghjghfjfgjfhjfgjfgjfgjfgjfgjgfDFSADFSDFSDSDFFSFSD")].as_mut_slice())?;
        let xl_ref = Variant::as_sref(
            xlref.col() as i32, (xlref.col() + xlref.cols()) as i32,
            xlref.row() as i32, (xlref.row() + xlref.rows()) as i32
        );*/
        //Logger::info(format!("xlcalert result {}", ca()));
        /*let xl_value = XLSDK::api_call(xlfGetCell, vec![Variant::from(32), xl_ref.clone()].as_mut_slice());
        XLSDK::api_call(xlcMessage, vec![Variant::from(1), xl_value.clone()].as_mut_slice());
        Logger::info(format!("xlcalert result {}", xl_value.to_string()));
*/
        //let result = XLSDK::api_call(xlfGetDef, vec![Variant::missing(), xl_ref.clone()].as_mut_slice());
        //let result = XLSDK::api_call(xlfGetDef, vec![xl_value, xl_ref.clone()].as_mut_slice());
        //XLSDK::api_call(xlcMessage, vec![Variant::from(1), result].as_mut_slice());

        //let result = XLSDK::api_call(xlcAlert, vec![Variant::from(3), Variant::from("SDFDSFSDFSADFShgjghjjhgfjfhjfjDFSDSDFFSFSD").clone(), ].as_mut_slice());
        //let result = XLSDK::api_call(xlfDialogBox, vec![Variant::from(vec!["toto", "tata"])].as_mut_slice());
        //Logger::info(format!("xlcalert result {}", result));
        //let result = XLSDK::api_call(xlcAlert, vec![Variant::from("SDFDSFSDFSADFShgjghjjhgfjfhjfjDFSDSDFFSFSD"), Variant::from(3) ].as_mut_slice());
        //let result = XLSDK::api_call(xlfDialogBox, vec![Variant::from(vec!["toto", "tata"])].as_mut_slice());
        //Logger::info(format!("xlcalert result {}", result.to_string()));
        successful::of()
    }
}
