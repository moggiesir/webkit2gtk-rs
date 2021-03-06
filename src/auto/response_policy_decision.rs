// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use PolicyDecision;
use URIRequest;
use URIResponse;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use webkit2_sys;

glib_wrapper! {
    pub struct ResponsePolicyDecision(Object<webkit2_sys::WebKitResponsePolicyDecision, webkit2_sys::WebKitResponsePolicyDecisionClass, ResponsePolicyDecisionClass>) @extends PolicyDecision;

    match fn {
        get_type => || webkit2_sys::webkit_response_policy_decision_get_type(),
    }
}

pub const NONE_RESPONSE_POLICY_DECISION: Option<&ResponsePolicyDecision> = None;

pub trait ResponsePolicyDecisionExt: 'static {
    fn get_request(&self) -> Option<URIRequest>;

    fn get_response(&self) -> Option<URIResponse>;

    #[cfg(any(feature = "v2_4", feature = "dox"))]
    fn is_mime_type_supported(&self) -> bool;

    fn connect_property_request_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_response_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ResponsePolicyDecision>> ResponsePolicyDecisionExt for O {
    fn get_request(&self) -> Option<URIRequest> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_response_policy_decision_get_request(self.as_ref().to_glib_none().0))
        }
    }

    fn get_response(&self) -> Option<URIResponse> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_response_policy_decision_get_response(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_4", feature = "dox"))]
    fn is_mime_type_supported(&self) -> bool {
        unsafe {
            from_glib(webkit2_sys::webkit_response_policy_decision_is_mime_type_supported(self.as_ref().to_glib_none().0))
        }
    }

    fn connect_property_request_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::request\0".as_ptr() as *const _,
                Some(transmute(notify_request_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_response_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::response\0".as_ptr() as *const _,
                Some(transmute(notify_response_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_request_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_sys::WebKitResponsePolicyDecision, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<ResponsePolicyDecision> {
    let f: &F = &*(f as *const F);
    f(&ResponsePolicyDecision::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_response_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_sys::WebKitResponsePolicyDecision, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<ResponsePolicyDecision> {
    let f: &F = &*(f as *const F);
    f(&ResponsePolicyDecision::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for ResponsePolicyDecision {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ResponsePolicyDecision")
    }
}
