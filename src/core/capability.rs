use model::data::{Component, U16, U32, DynOption, MessageOption, Message, DataType, Check, Trame};
use std::collections::HashMap;
use model::error::{RdpResult, Error, RdpError, RdpErrorKind};
use std::io::Cursor;
use core::gcc::{KeyboardLayout, KeyboardType};

#[repr(u16)]
#[derive(Eq, PartialEq, Hash, Debug)]
pub enum CapabilitySetType {
    CapstypeGeneral = 0x0001,
    CapstypeBitmap = 0x0002,
    CapstypeOrder = 0x0003,
    CapstypeBitmapcache = 0x0004,
    CapstypeControl = 0x0005,
    CapstypeActivation = 0x0007,
    CapstypePointer = 0x0008,
    CapstypeShare = 0x0009,
    CapstypeColorcache = 0x000A,
    CapstypeSound = 0x000C,
    CapstypeInput = 0x000D,
    CapstypeFont = 0x000E,
    CapstypeBrush = 0x000F,
    CapstypeGlyphcache = 0x0010,
    CapstypeOffscreencache = 0x0011,
    CapstypeBitmapcacheHostsupport = 0x0012,
    CapstypeBitmapcacheRev2 = 0x0013,
    CapstypeVirtualchannel = 0x0014,
    CapstypeDrawninegridcache = 0x0015,
    CapstypeDrawgdiplus = 0x0016,
    CapstypeRail = 0x0017,
    CapstypeWindow = 0x0018,
    CapsettypeCompdesk = 0x0019,
    CapsettypeMultifragmentupdate = 0x001A,
    CapsettypeLargePointer = 0x001B,
    CapsettypeSurfaceCommands = 0x001C,
    CapsettypeBitmapCodecs = 0x001D,
    CapssettypeFrameAcknowledge = 0x001E,
    Unknown
}

impl From<u16> for CapabilitySetType {
    fn from(e: u16) -> Self {
        match e {
            0x0001 => CapabilitySetType::CapstypeGeneral,
            0x0002 => CapabilitySetType::CapstypeBitmap,
            0x0003 => CapabilitySetType::CapstypeOrder,
            0x0004 => CapabilitySetType::CapstypeBitmapcache,
            0x0005 => CapabilitySetType::CapstypeControl,
            0x0007 => CapabilitySetType::CapstypeActivation,
            0x0008 => CapabilitySetType::CapstypePointer,
            0x0009 => CapabilitySetType::CapstypeShare,
            0x000A => CapabilitySetType::CapstypeColorcache,
            0x000C => CapabilitySetType::CapstypeSound,
            0x000D => CapabilitySetType::CapstypeInput,
            0x000E => CapabilitySetType::CapstypeFont,
            0x000F => CapabilitySetType::CapstypeBrush,
            0x0010 => CapabilitySetType::CapstypeGlyphcache,
            0x0011 => CapabilitySetType::CapstypeOffscreencache,
            0x0012 => CapabilitySetType::CapstypeBitmapcacheHostsupport,
            0x0013 => CapabilitySetType::CapstypeBitmapcacheRev2,
            0x0014 => CapabilitySetType::CapstypeVirtualchannel,
            0x0015 => CapabilitySetType::CapstypeDrawninegridcache,
            0x0016 => CapabilitySetType::CapstypeDrawgdiplus,
            0x0017 => CapabilitySetType::CapstypeRail,
            0x0018 => CapabilitySetType::CapstypeWindow,
            0x0019 => CapabilitySetType::CapsettypeCompdesk,
            0x001A => CapabilitySetType::CapsettypeMultifragmentupdate,
            0x001B => CapabilitySetType::CapsettypeLargePointer,
            0x001C => CapabilitySetType::CapsettypeSurfaceCommands,
            0x001D => CapabilitySetType::CapsettypeBitmapCodecs,
            0x001E => CapabilitySetType::CapssettypeFrameAcknowledge,
            _ => CapabilitySetType::Unknown
        }
    }
}

/// General capability header
/// https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-rdpbcgr/d705c3b6-a392-4b32-9610-391f6af62323?redirectedfrom=MSDN
pub fn capability_set(capability_set_type: Option<CapabilitySetType>, blob: Option<Vec<u8>>) -> Component {
    let default_blob = blob.unwrap_or(vec![]);
    component![
        "capabilitySetType" => U16::LE(capability_set_type.unwrap_or(CapabilitySetType::CapstypeBitmap) as u16),
        "lengthCapability" => DynOption::new(U16::LE(default_blob.length() as u16 + 4), |length| MessageOption::Size("capabilitySet".to_string(), length.get() as usize - 4)),
        "capabilitySet" => default_blob
    ]
}

#[repr(u16)]
#[allow(dead_code)]
enum MajorType {
    OsmajortypeUnspecified = 0x0000,
    OsmajortypeWindows = 0x0001,
    OsmajortypeOs2 = 0x0002,
    OsmajortypeMacintosh = 0x0003,
    OsmajortypeUnix = 0x0004,
    OsmajortypeIos = 0x0005,
    OsmajortypeOsx = 0x0006,
    OsmajortypeAndroid = 0x0007
}

#[allow(dead_code)]
enum MinorType {
    OsminortypeUnspecified = 0x0000,
    OsminortypeWindows31x = 0x0001,
    OsminortypeWindows95 = 0x0002,
    OsminortypeWindowsNt = 0x0003,
    OsminortypeOs2V21 = 0x0004,
    OsminortypePowerPc = 0x0005,
    OsminortypeMacintosh = 0x0006,
    OsminortypeNativeXserver = 0x0007,
    OsminortypePseudoXserver = 0x0008,
    OsminortypeWindowsRt = 0x0009
}

#[repr(u16)]
pub enum GeneralExtraFlag {
    FastpathOutputSupported = 0x0001,
    NoBitmapCompressionHdr = 0x0400,
    LongCredentialsSupported = 0x0004,
    AutoreconnectSupported = 0x0008,
    EncSaltedChecksum = 0x0010
}

/// General capability
/// This capability is send by both side
///
/// https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-rdpbcgr/41dc6845-07dc-4af6-bc14-d8281acd4877?redirectedfrom=MSDN
pub fn ts_general_capability_set(extra_flags: Option<u16>) -> Component {
    component![
        "osMajorType" => U16::LE(MajorType::OsmajortypeWindows as u16),
        "osMinorType" => U16::LE(MinorType::OsminortypeWindowsNt as u16),
        "protocolVersion" => Check::new(U16::LE(0x0200)),
        "pad2octetsA" => U16::LE(0),
        "generalCompressionTypes" => Check::new(U16::LE(0)),
        "extraFlags" => U16::LE(extra_flags.unwrap_or(0)),
        "updateCapabilityFlag" => Check::new(U16::LE(0)),
        "remoteUnshareFlag" => Check::new(U16::LE(0)),
        "generalCompressionLevel" => Check::new(U16::LE(0)),
        "refreshRectSupport" => 0 as u8,
        "suppressOutputSupport" => 0 as u8
    ]
}

pub fn ts_bitmap_capability_set(preferred_bits_per_pixel: Option<u16>, desktop_width: Option<u16>, desktop_height: Option<u16>) -> Component {
    component![
        "preferredBitsPerPixel" => U16::LE(preferred_bits_per_pixel.unwrap_or(0)),
        "receive1BitPerPixel" => Check::new(U16::LE(0x0001)),
        "receive4BitsPerPixel" => Check::new(U16::LE(0x0001)),
        "receive8BitsPerPixel" => Check::new(U16::LE(0x0001)),
        "desktopWidth" => U16::LE(desktop_width.unwrap_or(0)),
        "desktopHeight" => U16::LE(desktop_height.unwrap_or(0)),
        "pad2octets" => U16::LE(0),
        "desktopResizeFlag" => U16::LE(0),
        "bitmapCompressionFlag" => Check::new(U16::LE(0x0001)),
        "highColorFlags" => Check::new(0 as u8),
        "drawingFlags" => 0 as u8,
        "multipleRectangleSupport" => Check::new(U16::LE(0x0001)),
        "pad2octetsB" => U16::LE(0)
    ]
}

#[repr(u16)]
pub enum OrderFlag {
    NEGOTIATEORDERSUPPORT = 0x0002,
    ZEROBOUNDSDELTASSUPPORT = 0x0008,
    COLORINDEXSUPPORT = 0x0020,
    SOLIDPATTERNBRUSHONLY = 0x0040,
    ORDERFLAGS_EXTRA_FLAGS = 0x0080
}

pub fn ts_order_capability_set(order_flags: Option<u16>) -> Component {
    component![
        "terminalDescriptor" => vec![0 as u8; 16],
        "pad4octetsA" => U32::LE(0),
        "desktopSaveXGranularity" => U16::LE(1),
        "desktopSaveYGranularity" => U16::LE(20),
        "pad2octetsA" => U16::LE(0),
        "maximumOrderLevel" => U16::LE(1),
        "numberFonts" => U16::LE(0),
        "orderFlags" => U16::LE(order_flags.unwrap_or(OrderFlag::NEGOTIATEORDERSUPPORT as u16)),
        "orderSupport" => vec![0 as u8; 32],
        "textFlags" => U16::LE(0),
        "orderSupportExFlags" => U16::LE(0),
        "pad4octetsB" => U32::LE(0),
        "desktopSaveSize" => U32::LE(480*480),
        "pad2octetsC" => U16::LE(0),
        "pad2octetsD" => U16::LE(0),
        "textANSICodePage" => U16::LE(0),
        "pad2octetsE" => U16::LE(0)
    ]
}

pub fn ts_bitmap_cache_capability_set() -> Component {
    component![
        "pad1" => U32::LE(0),
        "pad2" => U32::LE(0),
        "pad3" => U32::LE(0),
        "pad4" => U32::LE(0),
        "pad5" => U32::LE(0),
        "pad6" => U32::LE(0),
        "cache0Entries" => U16::LE(0),
        "cache0MaximumCellSize" => U16::LE(0),
        "cache1Entries" => U16::LE(0),
        "cache1MaximumCellSize" => U16::LE(0),
        "cache2Entries" => U16::LE(0),
        "cache2MaximumCellSize" => U16::LE(0)
    ]
}

/// Pointer capability
/// send by both client and server
///
/// https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-rdpbcgr/925e2c05-c13f-44b1-aa20-23082051fef9?redirectedfrom=MSDN
pub fn ts_pointer_capability_set() -> Component {
    component![
        "colorPointerFlag" => U16::LE(0),
        "colorPointerCacheSize" => U16::LE(20)
    ]
}

#[repr(u16)]
pub enum InputFlags {
    InputFlagScancodes = 0x0001,
    InputFlagMousex = 0x0004,
    InputFlagFastpathInput = 0x0008,
    InputFlagUnicode = 0x0010,
    InputFlagFastpathInput2 = 0x0020,
    InputFlagUnused1 = 0x0040,
    InputFlagUnused2 = 0x0080,
    TsInputFlagMouseHwheel = 0x0100
}

pub fn ts_input_capability_set(input_flags: Option<u16>, keyboard_layout: Option<KeyboardLayout>) -> Component {
    component![
        "inputFlags" => U16::LE(input_flags.unwrap_or(0)),
        "pad2octetsA" => U16::LE(0),
        "keyboardLayout" => U32::LE(keyboard_layout.unwrap_or(KeyboardLayout::French) as u32),
        "keyboardType" => U32::LE(KeyboardType::Ibm101102Keys as u32),
        "keyboardSubType" => U32::LE(0),
        "keyboardFunctionKey" => U32::LE(12),
        "imeFileName" => vec![0 as u8; 64]
    ]
}

/// Brush capability
/// send from client to server
///
/// https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-rdpbcgr/8b6a830f-3dde-4a84-9250-21ffa7d2e342?redirectedfrom=MSDN
pub fn ts_brush_capability_set() -> Component {
    component![
        "brushSupportLevel" => U32::LE(0)
    ]
}

/// Glyph cache entry
/// https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-rdpbcgr/cae26830-263c-4c1e-97c2-b561faded3d9?redirectedfrom=MSDN
fn cache_entry() -> Component {
    component![
        "cacheEntries" => U16::LE(0),
        "cacheMaximumCellSize" => U16::LE(0)
    ]
}


/// Glyph capability set
/// send from client to server
pub fn ts_glyph_capability_set() -> Component {
    component![
        "glyphCache" => trame![
            cache_entry(), cache_entry(), cache_entry(), cache_entry(), cache_entry(),
            cache_entry(), cache_entry(), cache_entry(), cache_entry(), cache_entry()
        ],
        "fragCache" => U32::LE(0),
        "glyphSupportLevel" => U16::LE(0),
        "pad2octets" => U16::LE(0)
    ]
}

/// Offscreen capability
/// send from client to server
///
/// https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-rdpbcgr/412fa921-2faa-4f1b-ab5f-242cdabc04f9?redirectedfrom=MSDN
pub fn ts_offscreen_capability_set() -> Component {
    component![
        "offscreenSupportLevel" => U32::LE(0),
        "offscreenCacheSize" => U16::LE(0),
        "offscreenCacheEntries" => U16::LE(0)
    ]
}

/// Virtual channel capability
/// send by both side (client server)
///
/// https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-rdpbcgr/a8593178-80c0-4b80-876c-cb77e62cecfc?redirectedfrom=MSDN
pub fn ts_virtualchannel_capability_set() -> Component {
    component![
        "flags" => U32::LE(0),
        "VCChunkSize" => Some(U32::LE(0))
    ]
}

/// Sound capability
/// send from client server
///
/// https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-rdpbcgr/fadb6a2c-18fa-4fa7-a155-e970d9b1ac59?redirectedfrom=MSDN
pub fn ts_sound_capability_set() -> Component {
    component![
        "soundFlags" => U16::LE(0),
        "pad2octetsA" => U16::LE(0)
    ]
}

/// Multi fragment capability
/// send by both side (client, server)
///
/// https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-rdpbcgr/01717954-716a-424d-af35-28fb2b86df89?redirectedfrom=MSDN
pub fn ts_multifragment_update_capability_ts() -> Component {
    component![
        "MaxRequestSize" => U32::LE(0)
    ]
}

/// Parse the capability array into an indexed array
pub fn parse_capability_set(capabilities: &Trame) -> RdpResult<HashMap<CapabilitySetType, Component>> {
    let mut result = HashMap::new();

    for element in capabilities.iter() {
        let capability = cast!(DataType::Component, element)?;
        let cap_type = CapabilitySetType::from(cast!(DataType::U16, capability["capabilitySetType"])?);
        let mut parsed_capability = match cap_type{
            CapabilitySetType::CapstypeGeneral => ts_general_capability_set(None),
            CapabilitySetType::CapstypeBitmap => ts_bitmap_capability_set(None, None, None),
            CapabilitySetType::CapstypeOrder => ts_order_capability_set(None),
            CapabilitySetType::CapstypeBitmapcache => ts_bitmap_cache_capability_set(),
            CapabilitySetType::CapstypePointer => ts_pointer_capability_set(),
            CapabilitySetType::CapstypeInput => ts_input_capability_set(None, None),
            CapabilitySetType::CapstypeBrush => ts_brush_capability_set(),
            CapabilitySetType::CapstypeGlyphcache => ts_glyph_capability_set(),
            CapabilitySetType::CapstypeOffscreencache => ts_offscreen_capability_set(),
            CapabilitySetType::CapstypeVirtualchannel => ts_virtualchannel_capability_set(),
            CapabilitySetType::CapstypeSound => ts_sound_capability_set(),
            CapabilitySetType::CapsettypeMultifragmentupdate => ts_multifragment_update_capability_ts(),
            _ => { println!("Unknown capability set {:?}", cap_type); continue }
        };

        parsed_capability.read(&mut Cursor::new(cast!(DataType::Slice, capability["capabilitySet"])?))?;
        result.insert(cap_type, parsed_capability);
    }
    Ok(result)
}