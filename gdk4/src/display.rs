// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{keys::Key, Display, Event, KeymapKey, ModifierType};
use glib::translate::*;
use glib::IsA;
use std::{mem, ptr};

pub trait DisplayExtManual: 'static {
    #[doc(alias = "gdk_display_translate_key")]
    fn translate_key(
        &self,
        keycode: u32,
        state: ModifierType,
        group: i32,
    ) -> Option<(Key, i32, i32, ModifierType)>;

    #[doc(alias = "gdk_display_get_setting")]
    fn get_setting(&self, name: &str) -> Option<glib::Value>;

    #[doc(alias = "gdk_display_map_keyval")]
    fn map_keyval(&self, keyval: Key) -> Option<Vec<KeymapKey>>;

    #[doc(alias = "gdk_display_map_keycode")]
    fn map_keycode(&self, keycode: u32) -> Option<Vec<(KeymapKey, Key)>>;

    #[doc(alias = "gdk_display_put_event")]
    fn put_event<P: AsRef<Event>>(&self, event: &P);
}

impl<O: IsA<Display>> DisplayExtManual for O {
    fn translate_key(
        &self,
        keycode: u32,
        state: ModifierType,
        group: i32,
    ) -> Option<(Key, i32, i32, ModifierType)> {
        unsafe {
            let mut keyval = mem::MaybeUninit::uninit();
            let mut effective_group = mem::MaybeUninit::uninit();
            let mut level = mem::MaybeUninit::uninit();
            let mut consumed = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_display_translate_key(
                self.as_ref().to_glib_none().0,
                keycode,
                state.into_glib(),
                group,
                keyval.as_mut_ptr(),
                effective_group.as_mut_ptr(),
                level.as_mut_ptr(),
                consumed.as_mut_ptr(),
            ));
            if ret {
                let keyval: Key = keyval.assume_init().into();
                let effective_group = effective_group.assume_init();
                let level = level.assume_init();
                let consumed = consumed.assume_init();
                Some((keyval, effective_group, level, from_glib(consumed)))
            } else {
                None
            }
        }
    }

    fn get_setting(&self, name: &str) -> Option<glib::Value> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let ret = ffi::gdk_display_get_setting(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            if from_glib(ret) {
                Some(value)
            } else {
                None
            }
        }
    }

    fn map_keyval(&self, keyval: Key) -> Option<Vec<KeymapKey>> {
        unsafe {
            let mut keys = ptr::null_mut();
            let mut n_keys = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_display_map_keyval(
                self.as_ref().to_glib_none().0,
                keyval.into_glib(),
                &mut keys,
                n_keys.as_mut_ptr(),
            ));
            if ret {
                Some(FromGlibContainer::from_glib_full_num(
                    keys,
                    n_keys.assume_init() as usize,
                ))
            } else {
                None
            }
        }
    }

    fn map_keycode(&self, keycode: u32) -> Option<Vec<(KeymapKey, Key)>> {
        unsafe {
            let mut keys = ptr::null_mut();
            let mut keyvals = ptr::null_mut();
            let mut n_entries = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_display_map_keycode(
                self.as_ref().to_glib_none().0,
                keycode,
                &mut keys,
                &mut keyvals,
                n_entries.as_mut_ptr(),
            ));
            if ret {
                let n_keys = n_entries.assume_init() as usize;
                let keyvals: Vec<u32> = FromGlibContainer::from_glib_full_num(keyvals, n_keys);
                let keyvals = keyvals.into_iter().map(|k| k.into());
                let keys: Vec<KeymapKey> = FromGlibContainer::from_glib_full_num(keys, n_keys);

                Some(keys.into_iter().zip(keyvals).collect())
            } else {
                None
            }
        }
    }

    fn put_event<P: AsRef<Event>>(&self, event: &P) {
        unsafe {
            ffi::gdk_display_put_event(
                self.as_ref().to_glib_none().0,
                event.as_ref().to_glib_none().0,
            );
        }
    }
}
