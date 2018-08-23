#![allow(unknown_lints)]
#![allow(clippy)]
#![allow(missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces,
        unused_qualifications,
        unused_imports)]
// This code was autogenerated with dbus-codegen-rust, see https://github.com/diwic/dbus-rs

use dbus as dbus;
use dbus::arg;

pub trait OrgMprisMediaPlayer2 {
    type Err;
    fn raise(&self) -> Result<(), Self::Err>;
    fn quit(&self) -> Result<(), Self::Err>;
    fn get_can_quit(&self) -> Result<bool, Self::Err>;
    fn get_fullscreen(&self) -> Result<bool, Self::Err>;
    fn set_fullscreen(&self, value: bool) -> Result<(), Self::Err>;
    fn get_can_set_fullscreen(&self) -> Result<bool, Self::Err>;
    fn get_can_raise(&self) -> Result<bool, Self::Err>;
    fn get_has_track_list(&self) -> Result<bool, Self::Err>;
    fn get_identity(&self) -> Result<String, Self::Err>;
    fn get_desktop_entry(&self) -> Result<String, Self::Err>;
    fn get_supported_uri_schemes(&self) -> Result<Vec<String>, Self::Err>;
    fn get_supported_mime_types(&self) -> Result<Vec<String>, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgMprisMediaPlayer2 for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn raise(&self) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.mpris.MediaPlayer2".into(), &"Raise".into(), |_| {
        }));
        try!(m.as_result());
        Ok(())
    }

    fn quit(&self) -> Result<(), Self::Err> {
        let mut m = try!(self.method_call_with_args(&"org.mpris.MediaPlayer2".into(), &"Quit".into(), |_| {
        }));
        try!(m.as_result());
        Ok(())
    }

    fn get_can_quit(&self) -> Result<bool, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.mpris.MediaPlayer2", "CanQuit")
    }

    fn get_fullscreen(&self) -> Result<bool, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.mpris.MediaPlayer2", "Fullscreen")
    }

    fn get_can_set_fullscreen(&self) -> Result<bool, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.mpris.MediaPlayer2", "CanSetFullscreen")
    }

    fn get_can_raise(&self) -> Result<bool, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.mpris.MediaPlayer2", "CanRaise")
    }

    fn get_has_track_list(&self) -> Result<bool, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.mpris.MediaPlayer2", "HasTrackList")
    }

    fn get_identity(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.mpris.MediaPlayer2", "Identity")
    }

    fn get_desktop_entry(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.mpris.MediaPlayer2", "DesktopEntry")
    }

    fn get_supported_uri_schemes(&self) -> Result<Vec<String>, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.mpris.MediaPlayer2", "SupportedUriSchemes")
    }

    fn get_supported_mime_types(&self) -> Result<Vec<String>, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.mpris.MediaPlayer2", "SupportedMimeTypes")
    }

    fn set_fullscreen(&self, value: bool) -> Result<(), Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.mpris.MediaPlayer2", "Fullscreen", value)
    }
}
