use std::{mem, slice};
use std::ops::Deref;
use crate::semantics::cause::cause;
use crate::semantics::coalesce::{coalesce, Coalesce};
use crate::semantics::cons::cons;
use crate::semantics::convert::compiletime;
use crate::semantics::failed::failed;
use crate::semantics::monoid::MonoidBox;
use crate::semantics::mute::mute;
use crate::semantics::successful::successful;
use crate::xlsdk::variant::{Variant, xltypeMask};
use crate::xl::error::{Error, Cause, Ergo};
use crate::xl::range::Range;
use crate::xl::xlsession::XLSession;
use crate::xlsdk::xlcall::{LPXLOPER12, RW, xlbitDLLFree, xlerrNA, xlerrNull, xlerrValue, XLOPER12, xloper12__bindgen_ty_1, xloper12__bindgen_ty_1__bindgen_ty_3, xltypeMulti};

/*#[derive(Clone)]
pub struct Handle {
    pub tpe: String,
    pub page: u32,
    pub x: u16,
    pub y: u16,
    pub id: u32,
}*/

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Handle {
    pub tpe: String,
    pub id: u32
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct XLHandle(pub String);

#[derive(Clone)]
pub struct Array<A>(Box<Vec<A>>);

impl <A: Clone> Array<A> {
    #[inline(always)]
    pub fn len(&self) -> usize  {
        self.0.len()
    }

    #[inline(always)]
    pub fn data(&self) -> Vec<A>  {
        self.0.deref().clone()
    }

}

impl <A> From<Vec<A>> for Array<A> {
    fn from(value: Vec<A>) -> Self {
        Array(Box::new(value))
    }
}

#[derive(Clone)]
pub struct Matrix<A>(Box<(usize, Vec<A>)>);

impl <A: Clone> Matrix<A> {

    pub fn new(rows: usize, cols: usize) -> Matrix<A> {
        Matrix(Box::new((cols, Vec::with_capacity(rows * cols))))
    }

    pub fn new_unchecked_with_data(rows: usize, cols: usize, data: Vec<A>) -> Matrix<A> {
        Matrix(Box::new((cols, data)))
    }

    #[inline(always)]
    pub fn rows(&self) -> usize  {
        self.0.1.len() / self.0.0
    }

    #[inline(always)]
    pub fn columns(&self) -> usize  {
        self.0.0
    }

    #[inline(always)]
    pub fn data(&self) -> Vec<A>  {
        self.0.1.clone()
    }

    /*pub unsafe fn get_unchecked(&self, i: usize, j: usize) -> &A {
        let cols = self.columns();
        unsafe {
            self.0.1.get_unchecked(i * cols + j)
        }
    }

    pub unsafe fn set_unchecked(&self, i: usize, j: usize, value: A) -> () {
        let cols = self.columns();
        self.0.1[i * cols + j] = value;
    }*/

}



#[derive(Clone)]
pub enum Scalar {
    NA,
    Error(Cause),

    Int(i32),
    Real(f64),
    Bool(bool),
    Text(String),

    Handle(Box<Handle>),

    Array(Array<Scalar>),
    Matrix(Matrix<Scalar>),
}


pub mod scalar {
    use std::iter::Scan;
    use std::ops::Deref;
    use crate::semantics::convert::compiletime;
    use crate::xl::error::{Cause, Error};
    use crate::xl::scalar::{Array, Matrix, Scalar};

    pub struct na_reduce;
    pub struct transpose;
    pub struct force_dim_0d;
    pub struct force_dim_1d;
    pub struct force_dim_2d;

    impl compiletime::Converter<Scalar> for na_reduce {
        fn convert(scalar: Scalar) -> Option<Scalar> {
            match scalar {
                Scalar::NA => { None }
                Scalar::Matrix(..) => { Some ( Scalar::Error(Cause::Terminal(
                    Error::Conversion(String::from("reduce na conversion not possible"))
                )))}
                _ => { Some(scalar) }
            }
        }
    }

    impl compiletime::Converter<Scalar> for transpose {
        fn convert(scalar: Scalar) -> Option<Scalar> {
            Some(
                match scalar {
                    Scalar::Matrix(matrix) => {
                        let rows = matrix.rows();
                        let cols = matrix.columns();
                        let new_matrix = Matrix::<Scalar>::new(cols, rows);
                        let mut offset = 0;
                        for i in 0..rows {
                            for j in 0..cols {
                                unsafe {
                                    let value = matrix.data().get_unchecked(offset).clone();
                                    new_matrix.data()[j * rows + i] = value;
                                    offset += 1;
                                };
                            }
                        }
                        Scalar::Matrix(new_matrix)
                    }
                    Scalar::Array(data) => {
                        let rows = data.len();
                        let cols = 1;
                        let array = data.data();
                        let new_matrix = Matrix::<Scalar>::new_unchecked_with_data(cols, rows, array);
                        Scalar::Matrix(new_matrix)
                    }
                    _ => scalar,
                }
            )
        }
    }

    impl compiletime::Converter<Scalar> for force_dim_0d {
        fn convert(scalar: Scalar) -> Option<Scalar> {
            Some(
                match &scalar {
                    Scalar::Array(data) => { if (data.len() > 0) { data.data()[0].clone() }  else { Scalar::NA } }
                    Scalar::Matrix(data)=> {
                        if (data.rows() * data.columns() > 0) {
                            unsafe { data.data().get_unchecked(0).clone()} }  else { Scalar::NA } }
                    _ => { scalar }
                }
            )
        }
    }

    impl compiletime::Converter<Scalar> for force_dim_1d {
        fn convert(scalar: Scalar) -> Option<Scalar> {
            Some(
                match scalar {
                    Scalar:: Error(..) => scalar,
                    Scalar::Matrix(data)=> { Scalar::Array(Array::from(data.data())) } ,
                    _ => { Scalar::Array(Array::from(vec![scalar])) }
                }
            )
        }
    }

    impl compiletime::Converter<Scalar> for force_dim_2d {
        fn convert(scalar: Scalar) -> Option<Scalar> {
            Some(
                match &scalar {
                    Scalar::Error(..) => { scalar }
                    Scalar::Matrix(matrix) => { scalar }
                    Scalar::Array(array) => {
                        let rows = array.len();
                        let cols = 1;
                        let array = array.data().clone();
                        let new_matrix = Matrix::<Scalar>::new_unchecked_with_data(rows, cols, array);
                        Scalar::Matrix(new_matrix)
                    }
                    _ => {
                        Scalar::Matrix(
                            Matrix::<Scalar>::new_unchecked_with_data(1, 1, vec![scalar])
                        )
                    }
                }
            )
        }
    }
}

impl <A: compiletime::Converter<Scalar>> compiletime::With<Scalar, A> {
    fn scalar_from_variant(&self, variant: &Variant) -> Scalar {
        self._scalar_from_variant(variant).unwrap_or_else(|| Scalar::NA)
    }

    fn _scalar_from_variant(&self, variant: &Variant) -> Option<Scalar> {
        let v = &variant.0;
        let ret = match v.xltype & xltypeMask {
            xltypeErr => match unsafe { v.val.err } as u32 {
                xlerrNull => { Scalar::Error( failed::with(Error::Cell("#NULL!".to_string())).of()) }
                xlerrDiv0 => { Scalar::Error( failed::with(Error::Cell("#DIV!".to_string())).of()) }
                xlerrValue => { Scalar::Error( failed::with(Error::Cell("#VALUE!".to_string())).of()) }
                xlerrRef =>  { Scalar::Error( failed::with(Error::Cell("#REF!".to_string())).of()) }
                xlerrName =>  { Scalar::Error( failed::with(Error::Cell("#NULL!".to_string())).of()) }
                xlerrNum =>  { Scalar::Error( failed::with(Error::Cell("#NUM!".to_string())).of()) }
                xlerrNA => Scalar::NA,
                xlerrGettingData => { Scalar::Error( failed::with(Error::Cell("#DATA!".to_string())).of()) }
                v => { Scalar::Error( failed::with(Error::Cell("#UNKNOWN!".to_string())).of()) }
            },
            xltypeMissing => Scalar::NA,
            xltypeNil => Scalar::NA,
            xltypeNum => unsafe { Scalar::Real(v.val.num) },
            xltypeInt => unsafe { Scalar::Int(v.val.w) },
            xltype_str => {
                let cstr_slice = unsafe {
                    let cstr: *const u16 = v.val.str;
                    let cstr_len = *cstr.offset(0) as usize;
                    slice::from_raw_parts(cstr.offset(1), cstr_len)
                };
                match String::from_utf16(cstr_slice) {
                    Ok(s) => {
                        if s.len() > 1 {
                            if s.chars().next().unwrap() == '_' {
                                let xl_handle = XLHandle(s);
                                let handle = XLSession::from_xlhandle_to_handle(&xl_handle);
                                match handle {
                                    Ok(handle) => Scalar::Handle(handle),
                                    Err(c) => Scalar::Error(c)
                                }

                            } else {
                                Scalar::Text(s)
                            }
                        } else {
                            Scalar::NA
                        }
                    },
                    Err(e) => Scalar::Error(failed::with(Error::Conversion(e.to_string())).of()),
                }
            },
            xltypeBool => unsafe { Scalar::Bool(v.val.xbool == 1) },
            xltypeMulti => unsafe {
                let cols = v.val.array.columns;
                let rows = v.val.array.rows;
                let size = (cols * rows) as isize;
                if (size == 0) {
                    Scalar::NA
                } else {
                    let p = v.val.array.lparray;
                    let mut data = Vec::with_capacity(size as usize);
                    let mut errors = mute::of::<Cause>();
                    for i in 0..size {
                        let value = Variant::from(p.offset(i));
                        let oscalar = self._scalar_from_variant(&value);
                        match oscalar {
                            None => {}
                            Some(scalar) => {
                                match scalar {
                                    Scalar::Error(other_error) => {
                                        errors = cons::with((errors, other_error)).of::<Cause>();
                                    },
                                    _ => { data.push(scalar) }
                                }
                            }
                        };
                    }
                    if (errors.is_mute()) {
                        let matrix = Matrix::new_unchecked_with_data(rows as usize, cols as usize, data);
                        Scalar::Matrix(matrix)
                    } else {
                        Scalar::Error(errors)
                    }
                }
            },
            v => Scalar::Error( failed::with(Error::Cell("#NUM!".to_string())).of()),
        };
        A::convert(ret)
    }
}

impl Scalar {

    fn _from_matrix_to_variant(rows: usize, columns: usize, data: Vec<Scalar>, page: u32, x: u32, y: u32) -> Ergo<Variant> {
        if rows > 1_048_575 || columns > 16383 {
            failed::with(Error::RangeOverflow).of()
        } else if rows == 0 || columns == 0 {
            successful::with(Variant::from_err(xlerrNA)).of()
        } else {
            let mut valid = coalesce::of::<Cause>();
            let mut array: Vec<Variant> = Vec::with_capacity(rows * columns);
            let mut offset = 0;
            for i in 0 ..rows {
                for j in 0 .. columns {
                    let s = unsafe { data.get_unchecked(offset) };
                    let res = Self::_from_scalar_to_variant(s.clone(), page, x+j as u32, y+i as u32)
                        .coalesce(&mut valid)
                        .map( |variant| array.push(variant) );
                    offset += 1;
                }
            }
            match valid.unwrap() {
                Cause::Mute => {
                    let lparray = array.as_mut_ptr() as LPXLOPER12;
                    mem::forget(array);
                    let variant = Variant(XLOPER12 {
                        xltype: xltypeMulti | xlbitDLLFree,
                        val: xloper12__bindgen_ty_1 {
                            array: xloper12__bindgen_ty_1__bindgen_ty_3 {
                                lparray,
                                rows: rows as RW,
                                columns: columns as RW,
                            },
                        },
                    });
                    successful::with(variant).of()
                }
                c => {
                    Err(c)
                }
            }
            }
    }

    fn _from_scalar_to_variant(scalar: Scalar, page: u32, x: u32, y: u32) -> Ergo<Variant> {
        match scalar {
            Scalar::NA => { Ok(Variant::from_err(xlerrNA)) }
            Scalar::Error(c) => { Err(c)  }
            Scalar::Int(value) => { Ok(Variant::from(value)) }
            Scalar::Real(value) => { Ok(Variant::from(value)) }
            Scalar::Bool(value) => { Ok(Variant::from(value)) }
            Scalar::Text(value) => { Ok(Variant::from(value)) }
            Scalar::Handle(handle) => {
                match XLSession::from_handle_to_xlhandle(handle, page, x, y) {
                    Ok(xl_handle) => Ok(Variant::from(xl_handle.0)),
                    Err(c) => Err(c)
                }
            }
            Scalar::Array(array) => {
                Scalar::_from_matrix_to_variant(array.len(), 1, array.data(), page, x, y)
            }
            Scalar::Matrix(matrix) => {
                Scalar::_from_matrix_to_variant(matrix.rows(), matrix.columns(), matrix.data(), page, x, y)
            }
        }
    }

    pub(crate) fn to_variant(self, caller: Range) -> Variant {
        match Scalar::_from_scalar_to_variant(self, caller.id_sheet(), caller.x(), caller.y()) {
            Ok(variant) => { variant }
            Err(c) => { Variant::from_err(xlerrValue) }
        }
    }
}