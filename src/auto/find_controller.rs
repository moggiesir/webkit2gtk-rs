// This file was generated by gir (0d368d6) from gir-files (???)
// DO NOT EDIT

use WebView;
use ffi;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct FindController(Object<ffi::WebKitFindController>);

    match fn {
        get_type => || ffi::webkit_find_controller_get_type(),
    }
}

impl FindController {
    pub fn count_matches(&self, search_text: &str, find_options: u32, max_match_count: u32) {
        unsafe {
            ffi::webkit_find_controller_count_matches(self.to_glib_none().0, search_text.to_glib_none().0, find_options, max_match_count);
        }
    }

    pub fn get_max_match_count(&self) -> u32 {
        unsafe {
            ffi::webkit_find_controller_get_max_match_count(self.to_glib_none().0)
        }
    }

    pub fn get_options(&self) -> u32 {
        unsafe {
            ffi::webkit_find_controller_get_options(self.to_glib_none().0)
        }
    }

    pub fn get_search_text(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_find_controller_get_search_text(self.to_glib_none().0))
        }
    }

    pub fn get_web_view(&self) -> Option<WebView> {
        unsafe {
            from_glib_none(ffi::webkit_find_controller_get_web_view(self.to_glib_none().0))
        }
    }

    pub fn search(&self, search_text: &str, find_options: u32, max_match_count: u32) {
        unsafe {
            ffi::webkit_find_controller_search(self.to_glib_none().0, search_text.to_glib_none().0, find_options, max_match_count);
        }
    }

    pub fn search_finish(&self) {
        unsafe {
            ffi::webkit_find_controller_search_finish(self.to_glib_none().0);
        }
    }

    pub fn search_next(&self) {
        unsafe {
            ffi::webkit_find_controller_search_next(self.to_glib_none().0);
        }
    }

    pub fn search_previous(&self) {
        unsafe {
            ffi::webkit_find_controller_search_previous(self.to_glib_none().0);
        }
    }

    pub fn connect_counted_matches<F: Fn(&FindController, u32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&FindController, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "counted-matches",
                transmute(counted_matches_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_failed_to_find_text<F: Fn(&FindController) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&FindController) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "failed-to-find-text",
                transmute(failed_to_find_text_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_found_text<F: Fn(&FindController, u32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&FindController, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "found-text",
                transmute(found_text_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn counted_matches_trampoline(this: *mut ffi::WebKitFindController, match_count: libc::c_uint, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&FindController, u32) + 'static> = transmute(f);
    f(&from_glib_none(this), match_count)
}

unsafe extern "C" fn failed_to_find_text_trampoline(this: *mut ffi::WebKitFindController, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&FindController) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn found_text_trampoline(this: *mut ffi::WebKitFindController, match_count: libc::c_uint, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&FindController, u32) + 'static> = transmute(f);
    f(&from_glib_none(this), match_count)
}