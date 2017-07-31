// This file was generated by gir (f00d658) from gir-files (0bcaef9)
// DO NOT EDIT

use MainContext;
use ffi;
use ffi as glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;
use translate::*;

glib_wrapper! {
    pub struct MainLoop(Shared<ffi::GMainLoop>);

    match fn {
        ref => |ptr| ffi::g_main_loop_ref(ptr),
        unref => |ptr| ffi::g_main_loop_unref(ptr),
        get_type => || ffi::g_main_loop_get_type(),
    }
}

impl MainLoop {
    pub fn new<'a, P: Into<Option<&'a MainContext>>>(context: P, is_running: bool) -> MainLoop {
        let context = context.into();
        let context = context.to_glib_none();
        unsafe {
            from_glib_full(ffi::g_main_loop_new(context.0, is_running.to_glib()))
        }
    }

    pub fn get_context(&self) -> Option<MainContext> {
        unsafe {
            from_glib_none(ffi::g_main_loop_get_context(self.to_glib_none().0))
        }
    }

    pub fn is_running(&self) -> bool {
        unsafe {
            from_glib(ffi::g_main_loop_is_running(self.to_glib_none().0))
        }
    }

    pub fn quit(&self) {
        unsafe {
            ffi::g_main_loop_quit(self.to_glib_none().0);
        }
    }

    pub fn run(&self) {
        unsafe {
            ffi::g_main_loop_run(self.to_glib_none().0);
        }
    }
}

unsafe impl Send for MainLoop {}
unsafe impl Sync for MainLoop {}
