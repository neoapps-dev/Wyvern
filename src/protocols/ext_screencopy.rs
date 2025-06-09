use std::os::raw::{c_char, c_void};
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const sys::common::wl_interface; 4] = [
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
    NULLPTR as *const sys::common::wl_interface,
];
#[doc = "manager to inform clients and begin capturing\n\nThis object is a manager which offers requests to start capturing from a\nsource."]
pub mod zwlr_screencopy_manager_v1 {
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::sys::server::*;
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Resource, NULLPTR,
    };
    use std::os::raw::c_char;
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Request {
        #[doc = "capture an output\n\nCapture the next frame of an entire output."]
        CaptureOutput {
            frame: Main<super::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1>,
            overlay_cursor: i32,
            output: super::wl_output::WlOutput,
        },
        #[doc = "capture an output's region\n\nCapture the next frame of an output's region.\n\nThe region is given in output logical coordinates, see\nxdg_output.logical_size. The region will be clipped to the output's\nextents."]
        CaptureOutputRegion {
            frame: Main<super::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1>,
            overlay_cursor: i32,
            output: super::wl_output::WlOutput,
            x: i32,
            y: i32,
            width: i32,
            height: i32,
        },
        #[doc = "destroy the manager\n\nAll objects created by the manager will still remain valid, until their\nappropriate destroy request has been called.\n\nThis is a destructor, once received this object cannot be used any longer."]
        Destroy,
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "capture_output",
                since: 1,
                signature: &[
                    super::ArgumentType::NewId,
                    super::ArgumentType::Int,
                    super::ArgumentType::Object,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "capture_output_region",
                since: 1,
                signature: &[
                    super::ArgumentType::NewId,
                    super::ArgumentType::Int,
                    super::ArgumentType::Object,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                    super::ArgumentType::Int,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "destroy",
                since: 1,
                signature: &[],
                destructor: true,
            },
        ];
        type Map = super::ResourceMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Destroy => true,
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::CaptureOutput { .. } => 0,
                Request::CaptureOutputRegion { .. } => 1,
                Request::Destroy => 2,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::CaptureOutput { .. } => 1,
                Request::CaptureOutputRegion { .. } => 1,
                Request::Destroy => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(Object::from_interface::<
                    super::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1,
                >(version, meta.child())),
                1 => Some(Object::from_interface::<
                    super::zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1,
                >(version, meta.child())),
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                0 => {
                    let mut args = msg.args.into_iter();
                    Ok(Request::CaptureOutput {
                        frame: {
                            if let Some(Argument::NewId(val)) = args.next() {
                                map.get_new(val).ok_or(())?
                            } else {
                                return Err(());
                            }
                        },
                        overlay_cursor: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        output: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get(val).ok_or(())?.into()
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                1 => {
                    let mut args = msg.args.into_iter();
                    Ok(Request::CaptureOutputRegion {
                        frame: {
                            if let Some(Argument::NewId(val)) = args.next() {
                                map.get_new(val).ok_or(())?
                            } else {
                                return Err(());
                            }
                        },
                        overlay_cursor: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        output: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get(val).ok_or(())?.into()
                            } else {
                                return Err(());
                            }
                        },
                        x: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        y: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        width: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                        height: {
                            if let Some(Argument::Int(val)) = args.next() {
                                val
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                2 => Ok(Request::Destroy),
                _ => Err(()),
            }
        }
        fn into_raw(self, sender_id: u32) -> Message {
            panic!("Request::into_raw can not be used Server-side.")
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Request, ()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Request::CaptureOutput {
                        frame: {
                            let me = Resource::<ZwlrScreencopyManagerV1>::from_c_ptr(obj as *mut _);
                            me . make_child_for :: < super :: zwlr_screencopy_frame_v1 :: ZwlrScreencopyFrameV1 > (_args [0] . n) . unwrap ()
                        },
                        overlay_cursor: _args[1].i,
                        output: Resource::<super::wl_output::WlOutput>::from_c_ptr(
                            _args[2].o as *mut _,
                        )
                        .into(),
                    })
                }
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 7);
                    Ok(Request::CaptureOutputRegion {
                        frame: {
                            let me = Resource::<ZwlrScreencopyManagerV1>::from_c_ptr(obj as *mut _);
                            me . make_child_for :: < super :: zwlr_screencopy_frame_v1 :: ZwlrScreencopyFrameV1 > (_args [0] . n) . unwrap ()
                        },
                        overlay_cursor: _args[1].i,
                        output: Resource::<super::wl_output::WlOutput>::from_c_ptr(
                            _args[2].o as *mut _,
                        )
                        .into(),
                        x: _args[3].i,
                        y: _args[4].i,
                        width: _args[5].i,
                        height: _args[6].i,
                    })
                }
                2 => Ok(Request::Destroy),
                _ => return Err(()),
            }
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            panic!("Request::as_raw_c_in can not be used Server-side.")
        }
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Event {}
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[];
        type Map = super::ResourceMap;
        fn is_destructor(&self) -> bool {
            match *self {}
        }
        fn opcode(&self) -> u16 {
            match *self {}
        }
        fn since(&self) -> u32 {
            match *self {}
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Event::from_raw can not be used Server-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {}
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Event, ()> {
            panic!("Event::from_raw_c can not be used Server-side.")
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            match self {}
        }
    }
    #[derive(Clone, Eq, PartialEq)]
    pub struct ZwlrScreencopyManagerV1(Resource<ZwlrScreencopyManagerV1>);
    impl AsRef<Resource<ZwlrScreencopyManagerV1>> for ZwlrScreencopyManagerV1 {
        #[inline]
        fn as_ref(&self) -> &Resource<Self> {
            &self.0
        }
    }
    impl From<Resource<ZwlrScreencopyManagerV1>> for ZwlrScreencopyManagerV1 {
        #[inline]
        fn from(value: Resource<Self>) -> Self {
            ZwlrScreencopyManagerV1(value)
        }
    }
    impl From<ZwlrScreencopyManagerV1> for Resource<ZwlrScreencopyManagerV1> {
        #[inline]
        fn from(value: ZwlrScreencopyManagerV1) -> Self {
            value.0
        }
    }
    impl std::fmt::Debug for ZwlrScreencopyManagerV1 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{:?}", self.0))
        }
    }
    impl Interface for ZwlrScreencopyManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwlr_screencopy_manager_v1";
        const VERSION: u32 = 3;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwlr_screencopy_manager_v1_interface }
        }
    }
    impl ZwlrScreencopyManagerV1 {}
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_CAPTURE_OUTPUT_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_CAPTURE_OUTPUT_REGION_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    static mut zwlr_screencopy_manager_v1_requests_capture_output_types: [*const wl_interface; 3] = [
        unsafe {
            &super::zwlr_screencopy_frame_v1::zwlr_screencopy_frame_v1_interface
                as *const wl_interface
        },
        NULLPTR as *const wl_interface,
        unsafe { &super::wl_output::wl_output_interface as *const wl_interface },
    ];
    static mut zwlr_screencopy_manager_v1_requests_capture_output_region_types:
        [*const wl_interface; 7] = [
        unsafe {
            &super::zwlr_screencopy_frame_v1::zwlr_screencopy_frame_v1_interface
                as *const wl_interface
        },
        NULLPTR as *const wl_interface,
        unsafe { &super::wl_output::wl_output_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
        NULLPTR as *const wl_interface,
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwlr_screencopy_manager_v1_requests: [wl_message; 3] = [
        wl_message {
            name: b"capture_output\0" as *const u8 as *const c_char,
            signature: b"nio\0" as *const u8 as *const c_char,
            types: unsafe { &zwlr_screencopy_manager_v1_requests_capture_output_types as *const _ },
        },
        wl_message {
            name: b"capture_output_region\0" as *const u8 as *const c_char,
            signature: b"nioiiii\0" as *const u8 as *const c_char,
            types: unsafe {
                &zwlr_screencopy_manager_v1_requests_capture_output_region_types as *const _
            },
        },
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwlr_screencopy_manager_v1_interface: wl_interface = wl_interface {
        name: b"zwlr_screencopy_manager_v1\0" as *const u8 as *const c_char,
        version: 3,
        request_count: 3,
        requests: unsafe { &zwlr_screencopy_manager_v1_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
#[doc = "a frame ready for copy\n\nThis object represents a single frame.\n\nWhen created, a series of buffer events will be sent, each representing a\nsupported buffer type. The \"buffer_done\" event is sent afterwards to\nindicate that all supported buffer types have been enumerated. The client\nwill then be able to send a \"copy\" request. If the capture is successful,\nthe compositor will send a \"flags\" followed by a \"ready\" event.\n\nFor objects version 2 or lower, wl_shm buffers are always supported, ie.\nthe \"buffer\" event is guaranteed to be sent.\n\nIf the capture failed, the \"failed\" event is sent. This can happen anytime\nbefore the \"ready\" event.\n\nOnce either a \"ready\" or a \"failed\" event is received, the client should\ndestroy the frame."]
pub mod zwlr_screencopy_frame_v1 {
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::sys::server::*;
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Resource, NULLPTR,
    };
    use std::os::raw::c_char;
    #[repr(u32)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum Error {
        #[doc = "the object has already been used to copy a wl_buffer"]
        AlreadyUsed = 0,
        #[doc = "buffer attributes are invalid"]
        InvalidBuffer = 1,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::AlreadyUsed),
                1 => Some(Error::InvalidBuffer),
                _ => Option::None,
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }
    bitflags! { pub struct Flags : u32 { # [doc = "contents are y-inverted"] const YInvert = 1 ; } }
    impl Flags {
        pub fn from_raw(n: u32) -> Option<Flags> {
            Some(Flags::from_bits_truncate(n))
        }
        pub fn to_raw(&self) -> u32 {
            self.bits()
        }
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Request {
        #[doc = "copy the frame\n\nCopy the frame to the supplied buffer. The buffer must have a the\ncorrect size, see zwlr_screencopy_frame_v1.buffer and\nzwlr_screencopy_frame_v1.linux_dmabuf. The buffer needs to have a\nsupported format.\n\nIf the frame is successfully copied, a \"flags\" and a \"ready\" events are\nsent. Otherwise, a \"failed\" event is sent."]
        Copy { buffer: super::wl_buffer::WlBuffer },
        #[doc = "delete this object, used or not\n\nDestroys the frame. This request can be sent at any time by the client.\n\nThis is a destructor, once received this object cannot be used any longer."]
        Destroy,
        #[doc = "copy the frame when it's damaged\n\nSame as copy, except it waits until there is damage to copy.\n\nOnly available since version 2 of the interface"]
        CopyWithDamage { buffer: super::wl_buffer::WlBuffer },
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "copy",
                since: 1,
                signature: &[super::ArgumentType::Object],
                destructor: false,
            },
            super::MessageDesc {
                name: "destroy",
                since: 1,
                signature: &[],
                destructor: true,
            },
            super::MessageDesc {
                name: "copy_with_damage",
                since: 2,
                signature: &[super::ArgumentType::Object],
                destructor: false,
            },
        ];
        type Map = super::ResourceMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Destroy => true,
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::Copy { .. } => 0,
                Request::Destroy => 1,
                Request::CopyWithDamage { .. } => 2,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Copy { .. } => 1,
                Request::Destroy => 1,
                Request::CopyWithDamage { .. } => 2,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                0 => {
                    let mut args = msg.args.into_iter();
                    Ok(Request::Copy {
                        buffer: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get(val).ok_or(())?.into()
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                1 => Ok(Request::Destroy),
                2 => {
                    let mut args = msg.args.into_iter();
                    Ok(Request::CopyWithDamage {
                        buffer: {
                            if let Some(Argument::Object(val)) = args.next() {
                                map.get(val).ok_or(())?.into()
                            } else {
                                return Err(());
                            }
                        },
                    })
                }
                _ => Err(()),
            }
        }
        fn into_raw(self, sender_id: u32) -> Message {
            panic!("Request::into_raw can not be used Server-side.")
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Request, ()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Request::Copy {
                        buffer: Resource::<super::wl_buffer::WlBuffer>::from_c_ptr(
                            _args[0].o as *mut _,
                        )
                        .into(),
                    })
                }
                1 => Ok(Request::Destroy),
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Request::CopyWithDamage {
                        buffer: Resource::<super::wl_buffer::WlBuffer>::from_c_ptr(
                            _args[0].o as *mut _,
                        )
                        .into(),
                    })
                }
                _ => return Err(()),
            }
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            panic!("Request::as_raw_c_in can not be used Server-side.")
        }
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Event {
        #[doc = "wl_shm buffer information\n\nProvides information about wl_shm buffer parameters that need to be\nused for this frame. This event is sent once after the frame is created\nif wl_shm buffers are supported."]
        Buffer {
            format: super::wl_shm::Format,
            width: u32,
            height: u32,
            stride: u32,
        },
        #[doc = "frame flags\n\nProvides flags about the frame. This event is sent once before the\n\"ready\" event."]
        Flags { flags: Flags },
        #[doc = "indicates frame is available for reading\n\nCalled as soon as the frame is copied, indicating it is available\nfor reading. This event includes the time at which presentation happened\nat.\n\nThe timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,\neach component being an unsigned 32-bit value. Whole seconds are in\ntv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,\nand the additional fractional part in tv_nsec as nanoseconds. Hence,\nfor valid timestamps tv_nsec must be in [0, 999999999]. The seconds part\nmay have an arbitrary offset at start.\n\nAfter receiving this event, the client should destroy the object."]
        Ready {
            tv_sec_hi: u32,
            tv_sec_lo: u32,
            tv_nsec: u32,
        },
        #[doc = "frame copy failed\n\nThis event indicates that the attempted frame copy has failed.\n\nAfter receiving this event, the client should destroy the object."]
        Failed,
        #[doc = "carries the coordinates of the damaged region\n\nThis event is sent right before the ready event when copy_with_damage is\nrequested. It may be generated multiple times for each copy_with_damage\nrequest.\n\nThe arguments describe a box around an area that has changed since the\nlast copy request that was derived from the current screencopy manager\ninstance.\n\nThe union of all regions received between the call to copy_with_damage\nand a ready event is the total damage since the prior ready event.\n\nOnly available since version 2 of the interface"]
        Damage {
            x: u32,
            y: u32,
            width: u32,
            height: u32,
        },
        #[doc = "linux-dmabuf buffer information\n\nProvides information about linux-dmabuf buffer parameters that need to\nbe used for this frame. This event is sent once after the frame is\ncreated if linux-dmabuf buffers are supported.\n\nOnly available since version 3 of the interface"]
        LinuxDmabuf {
            format: u32,
            width: u32,
            height: u32,
        },
        #[doc = "all buffer types reported\n\nThis event is sent once after all buffer events have been sent.\n\nThe client should proceed to create a buffer of one of the supported\ntypes, and send a \"copy\" request.\n\nOnly available since version 3 of the interface"]
        BufferDone,
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "buffer",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "flags",
                since: 1,
                signature: &[super::ArgumentType::Uint],
                destructor: false,
            },
            super::MessageDesc {
                name: "ready",
                since: 1,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "failed",
                since: 1,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "damage",
                since: 2,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "linux_dmabuf",
                since: 3,
                signature: &[
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                    super::ArgumentType::Uint,
                ],
                destructor: false,
            },
            super::MessageDesc {
                name: "buffer_done",
                since: 3,
                signature: &[],
                destructor: false,
            },
        ];
        type Map = super::ResourceMap;
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Event::Buffer { .. } => 0,
                Event::Flags { .. } => 1,
                Event::Ready { .. } => 2,
                Event::Failed => 3,
                Event::Damage { .. } => 4,
                Event::LinuxDmabuf { .. } => 5,
                Event::BufferDone => 6,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Buffer { .. } => 1,
                Event::Flags { .. } => 1,
                Event::Ready { .. } => 1,
                Event::Failed => 1,
                Event::Damage { .. } => 2,
                Event::LinuxDmabuf { .. } => 3,
                Event::BufferDone => 3,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Event::from_raw can not be used Server-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Event::Buffer {
                    format,
                    width,
                    height,
                    stride,
                } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![
                        Argument::Uint(format.to_raw()),
                        Argument::Uint(width),
                        Argument::Uint(height),
                        Argument::Uint(stride),
                    ],
                },
                Event::Flags { flags } => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![Argument::Uint(flags.to_raw()),],
                },
                Event::Ready {
                    tv_sec_hi,
                    tv_sec_lo,
                    tv_nsec,
                } => Message {
                    sender_id: sender_id,
                    opcode: 2,
                    args: smallvec![
                        Argument::Uint(tv_sec_hi),
                        Argument::Uint(tv_sec_lo),
                        Argument::Uint(tv_nsec),
                    ],
                },
                Event::Failed => Message {
                    sender_id: sender_id,
                    opcode: 3,
                    args: smallvec![],
                },
                Event::Damage {
                    x,
                    y,
                    width,
                    height,
                } => Message {
                    sender_id: sender_id,
                    opcode: 4,
                    args: smallvec![
                        Argument::Uint(x),
                        Argument::Uint(y),
                        Argument::Uint(width),
                        Argument::Uint(height),
                    ],
                },
                Event::LinuxDmabuf {
                    format,
                    width,
                    height,
                } => Message {
                    sender_id: sender_id,
                    opcode: 5,
                    args: smallvec![
                        Argument::Uint(format),
                        Argument::Uint(width),
                        Argument::Uint(height),
                    ],
                },
                Event::BufferDone => Message {
                    sender_id: sender_id,
                    opcode: 6,
                    args: smallvec![],
                },
            }
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Event, ()> {
            panic!("Event::from_raw_c can not be used Server-side.")
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            match self {
                Event::Buffer {
                    format,
                    width,
                    height,
                    stride,
                } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = format.to_raw();
                    _args_array[1].u = width;
                    _args_array[2].u = height;
                    _args_array[3].u = stride;
                    f(0, &mut _args_array)
                }
                Event::Flags { flags } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = flags.to_raw();
                    f(1, &mut _args_array)
                }
                Event::Ready {
                    tv_sec_hi,
                    tv_sec_lo,
                    tv_nsec,
                } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = tv_sec_hi;
                    _args_array[1].u = tv_sec_lo;
                    _args_array[2].u = tv_nsec;
                    f(2, &mut _args_array)
                }
                Event::Failed => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(3, &mut _args_array)
                }
                Event::Damage {
                    x,
                    y,
                    width,
                    height,
                } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = x;
                    _args_array[1].u = y;
                    _args_array[2].u = width;
                    _args_array[3].u = height;
                    f(4, &mut _args_array)
                }
                Event::LinuxDmabuf {
                    format,
                    width,
                    height,
                } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = format;
                    _args_array[1].u = width;
                    _args_array[2].u = height;
                    f(5, &mut _args_array)
                }
                Event::BufferDone => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(6, &mut _args_array)
                }
            }
        }
    }
    #[derive(Clone, Eq, PartialEq)]
    pub struct ZwlrScreencopyFrameV1(Resource<ZwlrScreencopyFrameV1>);
    impl AsRef<Resource<ZwlrScreencopyFrameV1>> for ZwlrScreencopyFrameV1 {
        #[inline]
        fn as_ref(&self) -> &Resource<Self> {
            &self.0
        }
    }
    impl From<Resource<ZwlrScreencopyFrameV1>> for ZwlrScreencopyFrameV1 {
        #[inline]
        fn from(value: Resource<Self>) -> Self {
            ZwlrScreencopyFrameV1(value)
        }
    }
    impl From<ZwlrScreencopyFrameV1> for Resource<ZwlrScreencopyFrameV1> {
        #[inline]
        fn from(value: ZwlrScreencopyFrameV1) -> Self {
            value.0
        }
    }
    impl std::fmt::Debug for ZwlrScreencopyFrameV1 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{:?}", self.0))
        }
    }
    impl Interface for ZwlrScreencopyFrameV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwlr_screencopy_frame_v1";
        const VERSION: u32 = 3;
        fn c_interface() -> *const wl_interface {
            unsafe { &zwlr_screencopy_frame_v1_interface }
        }
    }
    impl ZwlrScreencopyFrameV1 {
        #[doc = "wl_shm buffer information\n\nProvides information about wl_shm buffer parameters that need to be\nused for this frame. This event is sent once after the frame is created\nif wl_shm buffers are supported."]
        pub fn buffer(
            &self,
            format: super::wl_shm::Format,
            width: u32,
            height: u32,
            stride: u32,
        ) -> () {
            let msg = Event::Buffer {
                format: format,
                width: width,
                height: height,
                stride: stride,
            };
            self.0.send(msg);
        }
        #[doc = "frame flags\n\nProvides flags about the frame. This event is sent once before the\n\"ready\" event."]
        pub fn flags(&self, flags: Flags) -> () {
            let msg = Event::Flags { flags: flags };
            self.0.send(msg);
        }
        #[doc = "indicates frame is available for reading\n\nCalled as soon as the frame is copied, indicating it is available\nfor reading. This event includes the time at which presentation happened\nat.\n\nThe timestamp is expressed as tv_sec_hi, tv_sec_lo, tv_nsec triples,\neach component being an unsigned 32-bit value. Whole seconds are in\ntv_sec which is a 64-bit value combined from tv_sec_hi and tv_sec_lo,\nand the additional fractional part in tv_nsec as nanoseconds. Hence,\nfor valid timestamps tv_nsec must be in [0, 999999999]. The seconds part\nmay have an arbitrary offset at start.\n\nAfter receiving this event, the client should destroy the object."]
        pub fn ready(&self, tv_sec_hi: u32, tv_sec_lo: u32, tv_nsec: u32) -> () {
            let msg = Event::Ready {
                tv_sec_hi: tv_sec_hi,
                tv_sec_lo: tv_sec_lo,
                tv_nsec: tv_nsec,
            };
            self.0.send(msg);
        }
        #[doc = "frame copy failed\n\nThis event indicates that the attempted frame copy has failed.\n\nAfter receiving this event, the client should destroy the object."]
        pub fn failed(&self) -> () {
            let msg = Event::Failed;
            self.0.send(msg);
        }
        #[doc = "carries the coordinates of the damaged region\n\nThis event is sent right before the ready event when copy_with_damage is\nrequested. It may be generated multiple times for each copy_with_damage\nrequest.\n\nThe arguments describe a box around an area that has changed since the\nlast copy request that was derived from the current screencopy manager\ninstance.\n\nThe union of all regions received between the call to copy_with_damage\nand a ready event is the total damage since the prior ready event.\n\nOnly available since version 2 of the interface."]
        pub fn damage(&self, x: u32, y: u32, width: u32, height: u32) -> () {
            let msg = Event::Damage {
                x: x,
                y: y,
                width: width,
                height: height,
            };
            self.0.send(msg);
        }
        #[doc = "linux-dmabuf buffer information\n\nProvides information about linux-dmabuf buffer parameters that need to\nbe used for this frame. This event is sent once after the frame is\ncreated if linux-dmabuf buffers are supported.\n\nOnly available since version 3 of the interface."]
        pub fn linux_dmabuf(&self, format: u32, width: u32, height: u32) -> () {
            let msg = Event::LinuxDmabuf {
                format: format,
                width: width,
                height: height,
            };
            self.0.send(msg);
        }
        #[doc = "all buffer types reported\n\nThis event is sent once after all buffer events have been sent.\n\nThe client should proceed to create a buffer of one of the supported\ntypes, and send a \"copy\" request.\n\nOnly available since version 3 of the interface."]
        pub fn buffer_done(&self) -> () {
            let msg = Event::BufferDone;
            self.0.send(msg);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_COPY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_COPY_WITH_DAMAGE_SINCE: u32 = 2u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_BUFFER_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_FLAGS_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_READY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_FAILED_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_DAMAGE_SINCE: u32 = 2u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_LINUX_DMABUF_SINCE: u32 = 3u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_BUFFER_DONE_SINCE: u32 = 3u32;
    static mut zwlr_screencopy_frame_v1_requests_copy_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_buffer::wl_buffer_interface as *const wl_interface }];
    static mut zwlr_screencopy_frame_v1_requests_copy_with_damage_types: [*const wl_interface; 1] =
        [unsafe { &super::wl_buffer::wl_buffer_interface as *const wl_interface }];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwlr_screencopy_frame_v1_requests: [wl_message; 3] = [
        wl_message {
            name: b"copy\0" as *const u8 as *const c_char,
            signature: b"o\0" as *const u8 as *const c_char,
            types: unsafe { &zwlr_screencopy_frame_v1_requests_copy_types as *const _ },
        },
        wl_message {
            name: b"destroy\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"copy_with_damage\0" as *const u8 as *const c_char,
            signature: b"2o\0" as *const u8 as *const c_char,
            types: unsafe { &zwlr_screencopy_frame_v1_requests_copy_with_damage_types as *const _ },
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut zwlr_screencopy_frame_v1_events: [wl_message; 7] = [
        wl_message {
            name: b"buffer\0" as *const u8 as *const c_char,
            signature: b"uuuu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"flags\0" as *const u8 as *const c_char,
            signature: b"u\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"ready\0" as *const u8 as *const c_char,
            signature: b"uuu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"failed\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"damage\0" as *const u8 as *const c_char,
            signature: b"2uuuu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"linux_dmabuf\0" as *const u8 as *const c_char,
            signature: b"3uuu\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"buffer_done\0" as *const u8 as *const c_char,
            signature: b"3\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut zwlr_screencopy_frame_v1_interface: wl_interface = wl_interface {
        name: b"zwlr_screencopy_frame_v1\0" as *const u8 as *const c_char,
        version: 3,
        request_count: 3,
        requests: unsafe { &zwlr_screencopy_frame_v1_requests as *const _ },
        event_count: 7,
        events: unsafe { &zwlr_screencopy_frame_v1_events as *const _ },
    };
}
