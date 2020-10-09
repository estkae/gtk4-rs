// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Expression;
use Filter;

glib_wrapper! {
    pub struct BoolFilter(Object<gtk_sys::GtkBoolFilter, gtk_sys::GtkBoolFilterClass, BoolFilterClass>) @extends Filter;

    match fn {
        get_type => || gtk_sys::gtk_bool_filter_get_type(),
    }
}

impl BoolFilter {
    pub fn new<P: IsA<Expression>>(expression: Option<&P>) -> BoolFilter {
        assert_initialized_main_thread!();
        unsafe {
            Filter::from_glib_full(gtk_sys::gtk_bool_filter_new(
                expression.map(|p| p.as_ref()).to_glib_full(),
            ))
            .unsafe_cast()
        }
    }
}

#[derive(Clone, Default)]
pub struct BoolFilterBuilder {
    expression: Option<Expression>,
    invert: Option<bool>,
}

impl BoolFilterBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> BoolFilter {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref expression) = self.expression {
            properties.push(("expression", expression));
        }
        if let Some(ref invert) = self.invert {
            properties.push(("invert", invert));
        }
        let ret = glib::Object::new(BoolFilter::static_type(), &properties)
            .expect("object new")
            .downcast::<BoolFilter>()
            .expect("downcast");
        ret
    }

    pub fn expression<P: IsA<Expression>>(mut self, expression: &P) -> Self {
        self.expression = Some(expression.clone().upcast());
        self
    }

    pub fn invert(mut self, invert: bool) -> Self {
        self.invert = Some(invert);
        self
    }
}

pub const NONE_BOOL_FILTER: Option<&BoolFilter> = None;

pub trait BoolFilterExt: 'static {
    fn get_expression(&self) -> Option<Expression>;

    fn get_invert(&self) -> bool;

    fn set_expression<P: IsA<Expression>>(&self, expression: &P);

    fn set_invert(&self, invert: bool);

    fn connect_property_expression_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_invert_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<BoolFilter>> BoolFilterExt for O {
    fn get_expression(&self) -> Option<Expression> {
        unsafe {
            from_glib_none(gtk_sys::gtk_bool_filter_get_expression(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_invert(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_bool_filter_get_invert(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_expression<P: IsA<Expression>>(&self, expression: &P) {
        unsafe {
            gtk_sys::gtk_bool_filter_set_expression(
                self.as_ref().to_glib_none().0,
                expression.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_invert(&self, invert: bool) {
        unsafe {
            gtk_sys::gtk_bool_filter_set_invert(self.as_ref().to_glib_none().0, invert.to_glib());
        }
    }

    fn connect_property_expression_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_expression_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkBoolFilter,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BoolFilter>,
        {
            let f: &F = &*(f as *const F);
            f(&BoolFilter::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::expression\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_expression_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_invert_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_invert_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkBoolFilter,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BoolFilter>,
        {
            let f: &F = &*(f as *const F);
            f(&BoolFilter::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::invert\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_invert_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for BoolFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BoolFilter")
    }
}
