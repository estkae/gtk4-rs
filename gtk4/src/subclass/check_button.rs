// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`CheckButton`](crate::CheckButton).

use crate::{prelude::*, subclass::prelude::*, CheckButton};
use glib::translate::*;

pub trait CheckButtonImpl: CheckButtonImplExt + WidgetImpl {
    fn toggled(&self) {
        self.parent_toggled()
    }

    #[cfg(any(feature = "v4_2", docsrs))]
    fn activate(&self) {
        self.parent_activate()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::CheckButtonImplExt> Sealed for T {}
}

pub trait CheckButtonImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_toggled(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCheckButtonClass;
            if let Some(f) = (*parent_class).toggled {
                f(self.obj().unsafe_cast_ref::<CheckButton>().to_glib_none().0)
            }
        }
    }

    #[cfg(any(feature = "v4_2", docsrs))]
    fn parent_activate(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkCheckButtonClass;
            if let Some(f) = (*parent_class).activate {
                f(self.obj().unsafe_cast_ref::<CheckButton>().to_glib_none().0)
            }
        }
    }
}

impl<T: CheckButtonImpl> CheckButtonImplExt for T {}

unsafe impl<T: CheckButtonImpl> IsSubclassable<T> for CheckButton {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.toggled = Some(check_button_toggled::<T>);

        #[cfg(any(feature = "v4_2", docsrs))]
        {
            klass.activate = Some(check_button_activate::<T>);
        };
    }
}

unsafe extern "C" fn check_button_toggled<T: CheckButtonImpl>(ptr: *mut ffi::GtkCheckButton) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.toggled()
}

#[cfg(any(feature = "v4_2", docsrs))]
unsafe extern "C" fn check_button_activate<T: CheckButtonImpl>(ptr: *mut ffi::GtkCheckButton) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.activate()
}
