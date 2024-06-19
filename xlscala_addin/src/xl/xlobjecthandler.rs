use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::Mutex;
use crate::semantics::failed::failed;
use crate::semantics::successful::successful;
use crate::xl::scalar::{Handle, XLHandle};
use crate::xl::error::{Ergo, Error};


const BASE_SHIFT:                   usize = 7;
const BASE_MASK:                    usize = 2^BASE_SHIFT - 1;
const BASE_ALPHABET:                &'static [u8] = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ£§ÇÉáÓüæíÐßéÆóÊÔâôúËÒäöñÈõ¶àòÑÁÕåçûÂãÍµùÀÃÎþêÿÏÞëÖÚèÜÛïÙîýìØÝÄ¥ÌÅ©".as_bytes();

pub struct IdManager<A, const MaxId: u32, const BlockSize: u32> {
    last_id: u32,
    free_ids: Vec<u32>,
    ids: Mutex<HashMap<A, u32>>
}

impl <A:  Eq + Hash + Clone, const MaxId: u32, const BlockSize: u32> IdManager<A, MaxId, BlockSize> {

    pub fn new() -> IdManager<A, MaxId, BlockSize> {
        IdManager {
            last_id: 0,
            free_ids: Vec::with_capacity(BlockSize as usize),
            ids: Mutex::new(HashMap::new()),
        }
    }

    pub fn get(&mut self, key: A) -> Option<u32> {
        let mut ids = self.ids.lock().unwrap();
        match ids.get(&key) {
            Some(id) => {
                Some(id.clone())
            }
            None => {
                if self.free_ids.len() == 0 {
                    for i in 0..BlockSize {
                        self.free_ids.push(i);
                    }
                }
                let id = self.free_ids.pop().unwrap();
                if id > MaxId {
                    None
                } else {
                    ids.insert(key.clone(), id);
                    Some(id)
                }
            }
        }
    }

    pub fn delete(&mut self, key: A) {
        let mut ids = self.ids.lock().unwrap();
        match ids.get(&key) {
            Some(id) => {
                self.free_ids.push(id.clone());
                ids.remove(&key);
            }
            None => {}
        }
    }

}



pub struct XLObjectHandler {
    id_manager: IdManager<(u32, u32, u32), {2 << 21}, 64>,
    xl_handle_to_handle_map: Mutex<HashMap<XLHandle, Box<Handle>>>
}

impl XLObjectHandler {
    pub(crate) fn new() -> XLObjectHandler {
        XLObjectHandler {
            id_manager: IdManager::new(),
            xl_handle_to_handle_map: Mutex::new(HashMap::new()),
        }
    }

    pub(crate) fn from_xlhandle_to_handle(&mut self, xl_handle: &XLHandle) -> Ergo<Box<Handle>> {
        match self.xl_handle_to_handle_map.lock().unwrap().get(xl_handle) {
            None => {
                failed::with(Error::InvalidXLHandle(format!("{}", xl_handle.0))).of()
            }
            Some(handle) => {
                successful::with(handle.clone()).of::<Ergo<Box<Handle>>>()
            }
        }
    }

    pub(crate) fn from_handle_to_xlhandle(&mut self, handle: Box<Handle>, page: u32, x:u32, y: u32) -> Ergo<XLHandle> {
        match self.id_manager.get((page, x, y)) {
            Some(xlid) => {
                let mut tmp = xlid as usize;
                let c1 = BASE_ALPHABET[tmp & BASE_MASK];
                tmp = tmp >> BASE_SHIFT;
                let c2 = BASE_ALPHABET[tmp &BASE_MASK];
                tmp = tmp >> BASE_SHIFT;
                let c3 = BASE_ALPHABET[tmp & BASE_MASK];
                let mut ret: String = String::with_capacity(3);
                ret.push(c1 as char);
                ret.push(c2 as char);
                ret.push(c3 as char);
                /*let tpe = match handle.tpe.to_lowercase().deref() {
                    "string" => { "str" }
                    "boolean" => { "bool" }
                    "double" => { "real" }
                    //"short" => { "u16" }
                    _ => handle.tpe.deref()
                };*/

                let tpe = &handle.tpe;
                /*let xl_handle = if tpe.len() > 4 {
                    XLHandle(format!("_obj:{}", ret))
                } else {
                    XLHandle(format!("_{}:{}", tpe, ret))
                };*/

                let xl_handle = XLHandle(format!("_{}:{}", tpe, ret));
                self.xl_handle_to_handle_map.lock().unwrap().insert(xl_handle.clone(), handle);
                successful::with(xl_handle).of::<Ergo<XLHandle>>()

            }
            None => {
                failed::with(Error::ObjectNumberOverflow).of()
            }
        }
    }

}