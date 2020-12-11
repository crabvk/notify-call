#![allow(clippy::all)]
// This code was autogenerated with `dbus-codegen-rust -g -m None -d org.freedesktop.Notifications -p /org/freedesktop/Notifications`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopDBusPeer {
    fn ping(&self) -> Result<(), dbus::Error>;
    fn get_machine_id(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusPeer for blocking::Proxy<'a, C> {

    fn ping(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "Ping", ())
    }

    fn get_machine_id(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "GetMachineId", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }
}

pub trait OrgFreedesktopDBusIntrospectable {
    fn introspect(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusIntrospectable for blocking::Proxy<'a, C> {

    fn introspect(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .and_then(|r: (String, )| Ok(r.0, ))
    }
}

pub trait OrgFreedesktopDBusProperties {
    fn get<R0: for<'b> arg::Get<'b> + 'static>(&self, interface: &str, property: &str) -> Result<R0, dbus::Error>;
    fn get_all(&self, interface: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error>;
    fn set<I2: arg::Arg + arg::Append>(&self, interface: &str, property: &str, value: I2) -> Result<(), dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusProperties for blocking::Proxy<'a, C> {

    fn get<R0: for<'b> arg::Get<'b> + 'static>(&self, interface: &str, property: &str) -> Result<R0, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Get", (interface, property, ))
            .and_then(|r: (arg::Variant<R0>, )| Ok((r.0).0, ))
    }

    fn get_all(&self, interface: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "GetAll", (interface, ))
            .and_then(|r: (::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, )| Ok(r.0, ))
    }

    fn set<I2: arg::Arg + arg::Append>(&self, interface: &str, property: &str, value: I2) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Set", (interface, property, arg::Variant(value), ))
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
    pub interface: String,
    pub changed_properties: ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopDBusPropertiesPropertiesChanged {
            interface: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

pub trait OrgFreedesktopNotifications {
    fn get_capabilities(&self) -> Result<Vec<String>, dbus::Error>;
    fn notify(&self, arg0: &str, arg1: u32, arg2: &str, arg3: &str, arg4: &str, arg5: Vec<&str>, arg6: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>, arg7: i32) -> Result<u32, dbus::Error>;
    fn close_notification(&self, arg0: u32) -> Result<(), dbus::Error>;
    fn get_server_information(&self) -> Result<(String, String, String, String), dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopNotifications for blocking::Proxy<'a, C> {

    fn get_capabilities(&self) -> Result<Vec<String>, dbus::Error> {
        self.method_call("org.freedesktop.Notifications", "GetCapabilities", ())
            .and_then(|r: (Vec<String>, )| Ok(r.0, ))
    }

    fn notify(&self, arg0: &str, arg1: u32, arg2: &str, arg3: &str, arg4: &str, arg5: Vec<&str>, arg6: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>, arg7: i32) -> Result<u32, dbus::Error> {
        self.method_call("org.freedesktop.Notifications", "Notify", (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, ))
            .and_then(|r: (u32, )| Ok(r.0, ))
    }

    fn close_notification(&self, arg0: u32) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.Notifications", "CloseNotification", (arg0, ))
    }

    fn get_server_information(&self) -> Result<(String, String, String, String), dbus::Error> {
        self.method_call("org.freedesktop.Notifications", "GetServerInformation", ())
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopNotificationsActionInvoked {
    pub arg0: u32,
    pub arg1: String,
}

impl arg::AppendAll for OrgFreedesktopNotificationsActionInvoked {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.arg0, i);
        arg::RefArg::append(&self.arg1, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNotificationsActionInvoked {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNotificationsActionInvoked {
            arg0: i.read()?,
            arg1: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNotificationsActionInvoked {
    const NAME: &'static str = "ActionInvoked";
    const INTERFACE: &'static str = "org.freedesktop.Notifications";
}

#[derive(Debug)]
pub struct OrgFreedesktopNotificationsNotificationClosed {
    pub arg0: u32,
    pub arg1: u32,
}

impl arg::AppendAll for OrgFreedesktopNotificationsNotificationClosed {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.arg0, i);
        arg::RefArg::append(&self.arg1, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNotificationsNotificationClosed {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNotificationsNotificationClosed {
            arg0: i.read()?,
            arg1: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNotificationsNotificationClosed {
    const NAME: &'static str = "NotificationClosed";
    const INTERFACE: &'static str = "org.freedesktop.Notifications";
}