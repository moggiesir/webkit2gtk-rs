// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_2", feature = "dox"))]
use AuthenticationScheme;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use Credential;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use glib::GString;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use glib::signal::connect_raw;
use glib::translate::*;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use glib_sys;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v2_2", feature = "dox"))]
use std::mem::transmute;
use webkit2_sys;

glib_wrapper! {
    pub struct AuthenticationRequest(Object<webkit2_sys::WebKitAuthenticationRequest, webkit2_sys::WebKitAuthenticationRequestClass, AuthenticationRequestClass>);

    match fn {
        get_type => || webkit2_sys::webkit_authentication_request_get_type(),
    }
}

pub const NONE_AUTHENTICATION_REQUEST: Option<&AuthenticationRequest> = None;

pub trait AuthenticationRequestExt: 'static {
    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn can_save_credentials(&self) -> bool;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn cancel(&self);

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_host(&self) -> Option<GString>;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_port(&self) -> u32;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_proposed_credential(&self) -> Option<Credential>;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_realm(&self) -> Option<GString>;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_scheme(&self) -> AuthenticationScheme;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn is_for_proxy(&self) -> bool;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn is_retry(&self) -> bool;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AuthenticationRequest>> AuthenticationRequestExt for O {
    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn can_save_credentials(&self) -> bool {
        unsafe {
            from_glib(webkit2_sys::webkit_authentication_request_can_save_credentials(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn cancel(&self) {
        unsafe {
            webkit2_sys::webkit_authentication_request_cancel(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_host(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_authentication_request_get_host(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_port(&self) -> u32 {
        unsafe {
            webkit2_sys::webkit_authentication_request_get_port(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_proposed_credential(&self) -> Option<Credential> {
        unsafe {
            from_glib_full(webkit2_sys::webkit_authentication_request_get_proposed_credential(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_realm(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_authentication_request_get_realm(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn get_scheme(&self) -> AuthenticationScheme {
        unsafe {
            from_glib(webkit2_sys::webkit_authentication_request_get_scheme(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn is_for_proxy(&self) -> bool {
        unsafe {
            from_glib(webkit2_sys::webkit_authentication_request_is_for_proxy(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn is_retry(&self) -> bool {
        unsafe {
            from_glib(webkit2_sys::webkit_authentication_request_is_retry(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"cancelled\0".as_ptr() as *const _,
                Some(transmute(cancelled_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

#[cfg(any(feature = "v2_2", feature = "dox"))]
unsafe extern "C" fn cancelled_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_sys::WebKitAuthenticationRequest, f: glib_sys::gpointer)
where P: IsA<AuthenticationRequest> {
    let f: &F = &*(f as *const F);
    f(&AuthenticationRequest::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for AuthenticationRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AuthenticationRequest")
    }
}
