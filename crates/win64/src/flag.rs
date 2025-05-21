use bitflags::bitflags;
use windows::Win32::UI::WindowsAndMessaging::{
  self, PEEK_MESSAGE_REMOVE_TYPE, WINDOW_EX_STYLE, WINDOW_LONG_PTR_INDEX, WINDOW_STYLE, WNDCLASS_STYLES,
};

use crate::handle::Win32Type;

// pub const GWLP_HINSTANCE: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-6i32);
// pub const GWLP_HWNDPARENT: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-8i32);
// pub const GWLP_ID: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-12i32);
// pub const GWLP_USERDATA: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-21i32);
// pub const GWLP_WNDPROC: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-4i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum LongPointerIndex {
  Instance,
  Parent,
  Id,
  UserData,
  WndProc,
}

impl From<LongPointerIndex> for WINDOW_LONG_PTR_INDEX {
  fn from(value: LongPointerIndex) -> Self {
    match value {
      LongPointerIndex::Instance => WindowsAndMessaging::GWLP_HINSTANCE,
      LongPointerIndex::Parent => WindowsAndMessaging::GWLP_HWNDPARENT,
      LongPointerIndex::Id => WindowsAndMessaging::GWLP_ID,
      LongPointerIndex::UserData => WindowsAndMessaging::GWLP_USERDATA,
      LongPointerIndex::WndProc => WindowsAndMessaging::GWLP_WNDPROC,
    }
  }
}

impl Win32Type for LongPointerIndex {
  type Type = WINDOW_LONG_PTR_INDEX;

  fn to_win32(self) -> Self::Type {
    self.into()
  }
}

bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct PeekMessageFlags: u32 {
    const NoRemove = WindowsAndMessaging::PM_NOREMOVE.0;
    const Remove = WindowsAndMessaging::PM_REMOVE.0;
    const NoYield = WindowsAndMessaging::PM_NOYIELD.0;
    const Input = WindowsAndMessaging::PM_QS_INPUT.0;
    const Paint = WindowsAndMessaging::PM_QS_PAINT.0;
    const PostMessage = WindowsAndMessaging::PM_QS_POSTMESSAGE.0;
    const SendMessage = WindowsAndMessaging::PM_QS_SENDMESSAGE.0;
  }
}

impl From<PeekMessageFlags> for PEEK_MESSAGE_REMOVE_TYPE {
  fn from(value: PeekMessageFlags) -> Self {
    PEEK_MESSAGE_REMOVE_TYPE(value.bits())
  }
}

bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct WindowClassStyle: u32 {
    const ByteAlignClient = WindowsAndMessaging::CS_BYTEALIGNCLIENT.0;
    const ByteAlignWindow = WindowsAndMessaging::CS_BYTEALIGNWINDOW.0;
    const DeviceContext = WindowsAndMessaging::CS_CLASSDC.0;
    const DoubleClicks = WindowsAndMessaging::CS_DBLCLKS.0;
    const DropShadow = WindowsAndMessaging::CS_DROPSHADOW.0;
    const Global = WindowsAndMessaging::CS_GLOBALCLASS.0;
    const HorizontalRedraw = WindowsAndMessaging::CS_HREDRAW.0;
    const NoClose = WindowsAndMessaging::CS_NOCLOSE.0;
    const OwnDeviceContext = WindowsAndMessaging::CS_OWNDC.0;
    const ParentDeviceContext = WindowsAndMessaging::CS_PARENTDC.0;
    const SaveBits = WindowsAndMessaging::CS_SAVEBITS.0;
    const VerticalRedraw = WindowsAndMessaging::CS_VREDRAW.0;
  }
}

impl From<WindowClassStyle> for WNDCLASS_STYLES {
  fn from(value: WindowClassStyle) -> Self {
    WNDCLASS_STYLES(value.bits())
  }
}

bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct WindowStyle: u32 {
    const Border = WindowsAndMessaging::WS_BORDER.0;
    const Caption = WindowsAndMessaging::WS_CAPTION.0;
    const Child = WindowsAndMessaging::WS_CHILD.0;
    const ChildWindow = WindowsAndMessaging::WS_CHILDWINDOW.0;
    const ClipChildren = WindowsAndMessaging::WS_CLIPCHILDREN.0;
    const ClipSiblings = WindowsAndMessaging::WS_CLIPSIBLINGS.0;
    const Disabled = WindowsAndMessaging::WS_DISABLED.0;
    const DialogFrame = WindowsAndMessaging::WS_DLGFRAME.0;
    const Group = WindowsAndMessaging::WS_GROUP.0;
    const HorizontalScroll = WindowsAndMessaging::WS_HSCROLL.0;
    const Iconic = WindowsAndMessaging::WS_ICONIC.0;
    const Maximize = WindowsAndMessaging::WS_MAXIMIZE.0;
    const MaximizeBox = WindowsAndMessaging::WS_MAXIMIZEBOX.0;
    const Minimize = WindowsAndMessaging::WS_MINIMIZE.0;
    const MinimizeBox = WindowsAndMessaging::WS_MINIMIZEBOX.0;
    const Overlapped = WindowsAndMessaging::WS_OVERLAPPED.0;
    const OverlappedWindow = WindowsAndMessaging::WS_OVERLAPPEDWINDOW.0;
    const PopUp = WindowsAndMessaging::WS_POPUP.0;
    const PopUpWindow = WindowsAndMessaging::WS_POPUPWINDOW.0;
    const SizeBox = WindowsAndMessaging::WS_SIZEBOX.0;
    const SystemMenu = WindowsAndMessaging::WS_SYSMENU.0;
    const TabStop = WindowsAndMessaging::WS_TABSTOP.0;
    const ThickFrame = WindowsAndMessaging::WS_THICKFRAME.0;
    const Tiled = WindowsAndMessaging::WS_TILED.0;
    const TiledWindow = WindowsAndMessaging::WS_TILEDWINDOW.0;
    const Visible = WindowsAndMessaging::WS_VISIBLE.0;
    const VerticalScroll = WindowsAndMessaging::WS_VSCROLL.0;
  }
}

impl From<WindowStyle> for WINDOW_STYLE {
  fn from(value: WindowStyle) -> Self {
    WINDOW_STYLE(value.bits())
  }
}

bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub struct ExtendedWindowStyle: u32 {
    const AcceptFiles = WindowsAndMessaging::WS_EX_ACCEPTFILES.0;
    const AppWindow = WindowsAndMessaging::WS_EX_APPWINDOW.0;
    const ClientEdge = WindowsAndMessaging::WS_EX_CLIENTEDGE.0;
    const Composited = WindowsAndMessaging::WS_EX_COMPOSITED.0;
    const ContextHelp = WindowsAndMessaging::WS_EX_CONTEXTHELP.0;
    const ControlParent = WindowsAndMessaging::WS_EX_CONTROLPARENT.0;
    const DialogModalFrame = WindowsAndMessaging::WS_EX_DLGMODALFRAME.0;
    const Layered = WindowsAndMessaging::WS_EX_LAYERED.0;
    const LayoutRightToLeft = WindowsAndMessaging::WS_EX_LAYOUTRTL.0;
    const Left = WindowsAndMessaging::WS_EX_LEFT.0;
    const LeftScrollbar = WindowsAndMessaging::WS_EX_LEFTSCROLLBAR.0;
    const LeftToRightReading = WindowsAndMessaging::WS_EX_LTRREADING.0;
    const MDIChild = WindowsAndMessaging::WS_EX_MDICHILD.0;
    const NoActivate = WindowsAndMessaging::WS_EX_NOACTIVATE.0;
    const NoInheritLayout = WindowsAndMessaging::WS_EX_NOINHERITLAYOUT.0;
    const NoParentNotify = WindowsAndMessaging::WS_EX_NOPARENTNOTIFY.0;
    const NoRedirectionBitmap = WindowsAndMessaging::WS_EX_NOREDIRECTIONBITMAP.0;
    const OverlappedWindow = WindowsAndMessaging::WS_EX_OVERLAPPEDWINDOW.0;
    const PaletteWindow = WindowsAndMessaging::WS_EX_PALETTEWINDOW.0;
    const Right = WindowsAndMessaging::WS_EX_RIGHT.0;
    const RightScrollbar = WindowsAndMessaging::WS_EX_RIGHTSCROLLBAR.0;
    const RightToLeftReading = WindowsAndMessaging::WS_EX_RTLREADING.0;
    const StaticEdge = WindowsAndMessaging::WS_EX_STATICEDGE.0;
    const ToolWindow = WindowsAndMessaging::WS_EX_TOOLWINDOW.0;
    const TopMost = WindowsAndMessaging::WS_EX_TOPMOST.0;
    const Transparent = WindowsAndMessaging::WS_EX_TRANSPARENT.0;
    const WindowEdge = WindowsAndMessaging::WS_EX_WINDOWEDGE.0;
  }
}

impl From<ExtendedWindowStyle> for WINDOW_EX_STYLE {
  fn from(value: ExtendedWindowStyle) -> Self {
    WINDOW_EX_STYLE(value.bits())
  }
}
