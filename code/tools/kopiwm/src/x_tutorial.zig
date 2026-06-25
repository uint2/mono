//! X Library functions with extra notes/docs attached.
//!
//! https://x.org/releases/X11R7.7/doc/man/man3/

const X = @import("c_lib.zig").X;
const Coordinates = @import("enums.zig").Coordinates;
const log = @import("std").log;
const Rect = @import("rect.zig").Rect;

// -----------------------------------------------------------------------------
// ++ XID aliases
// -----------------------------------------------------------------------------

pub const Colormap = X.Colormap;
/// See the XC_* defines in X11. The usual cursor would be `XC_left_ptr`.
pub const Cursor = X.Cursor;
pub const Drawable = X.Drawable;
pub const KeySym = X.KeySym;
pub const Pixmap = X.Pixmap;
/// To specify a null state, use `None`.
pub const Window = X.Window;

// -----------------------------------------------------------------------------
// ++ Integer type aliases
// -----------------------------------------------------------------------------

pub const FcMatchKind = X.FcMatchKind;
pub const KeyCode = X.KeyCode;
pub const Time = X.Time;
pub const XID = X.XID;
pub const XftResult = X.XftResult;

// -----------------------------------------------------------------------------
// ++ Structs
// -----------------------------------------------------------------------------

/// The `Display` structure serves as the connection to the X server and that
/// contains all the information about that X server.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XOpenDisplay.3.xhtml
pub const Display = X.Display;

pub const FcCharSet = X.FcCharSet;
pub const FcConfig = X.FcConfig;
pub const FcPattern = X.FcPattern;
pub const FcResult = X.FcResult;
/// X Graphics Context.
pub const GC = X.GC;
pub const Visual = X.Visual;

/// ```c
/// typedef struct {
///     int type;             /* ButtonPress or ButtonRelease */
///     unsigned long serial; /* # of last request processed by server */
///     Bool send_event;      /* true if this came from a SendEvent request */
///     Display *display;     /* Display the event was read from */
///     Window window;        /* "event" window it is reported relative to */
///     Window root;          /* root window that the event occurred on */
///     Window subwindow;     /* child window */
///     Time time;            /* milliseconds */
///     int x, y;             /* pointer x, y coordinates in event window */
///     int x_root, y_root;   /* coordinates relative to root */
///     unsigned int state;   /* key or button mask */
///     unsigned int button;  /* detail */
///     Bool same_screen;     /* same screen flag */
/// } XButtonEvent;
/// ```
///
/// The type member is set to the event type constant name that uniquely
/// identifies it. For example, when the X server reports a GraphicsExpose
/// event to a client application, it sends an XGraphicsExposeEvent structure
/// with the type member set to GraphicsExpose. The display member is set to a
/// pointer to the display the event was read on. The send_event member is set
/// to True if the event came from a SendEvent protocol request. The serial
/// member is set from the serial number reported in the protocol but expanded
/// from the 16-bit least-significant bits to a full 32-bit value. The window
/// member is set to the window that is most useful to toolkit dispatchers.
///
/// These structures have the following common members: window, root,
/// subwindow, time, x, y, x_root, y_root, state, and same_screen. The window
/// member is set to the window on which the event was generated and is
/// referred to as the event window. As long as the conditions previously
/// discussed are met, this is the window used by the X server to report the
/// event. The root member is set to the source window's root window. The
/// x_root and y_root members are set to the pointer's coordinates relative to
/// the root window's origin at the time of the event.
///
/// The same_screen member is set to indicate whether the event window is on
/// the same screen as the root window and can be either True or False. If
/// True, the event and root windows are on the same screen. If False, the
/// event and root windows are not on the same screen.
///
/// If the source window is an inferior of the event window, the subwindow
/// member of the structure is set to the child of the event window that is the
/// source window or the child of the event window that is an ancestor of the
/// source window. Otherwise, the X server sets the subwindow member to None.
/// The time member is set to the time when the event was generated and is
/// expressed in milliseconds.
///
/// If the event window is on the same screen as the root window, the x and y
/// members are set to the coordinates relative to the event window's origin.
/// Otherwise, these members are set to zero.
///
/// The state member is set to indicate the logical state of the pointer
/// buttons and modifier keys just prior to the event, which is the bitwise
/// inclusive OR of one or more of the button or modifier key masks:
/// Button1Mask, Button2Mask, Button3Mask, Button4Mask, Button5Mask, ShiftMask,
/// LockMask, ControlMask, Mod1Mask, Mod2Mask, Mod3Mask, Mod4Mask, and
/// Mod5Mask.
///
/// Each of these structures also has a member that indicates the detail. For
/// the XKeyPressedEvent and XKeyReleasedEvent structures, this member is
/// called a keycode. It is set to a number that represents a physical key on
/// the keyboard. The keycode is an arbitrary representation for any key on the
/// keyboard (see sections 12.7 and 16.1).
///
/// For the XButtonPressedEvent and XButtonReleasedEvent structures, this
/// member is called button. It represents the pointer button that changed
/// state and can be the Button1, Button2, Button3, Button4, or Button5 value.
/// For the XPointerMovedEvent structure, this member is called is_hint. It can
/// be set to NotifyNormal or NotifyHint.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XButtonEvent.3.xhtml
pub const XButtonEvent = X.XButtonEvent;

/// For more information, see the docs for `XButtonEvent`.
pub const XButtonPressedEvent = XButtonEvent;

/// The res_name member contains the application name, and the res_class member
/// contains the application class. Note that the name set in this property may
/// differ from the name set as WM_NAME. That is, WM_NAME specifies what should
/// be displayed in the title bar and, therefore, can contain temporal
/// information (for example, the name of a file currently in an editor's
/// buffer). On the other hand, the name specified as part of WM_CLASS is the
/// formal name of the application that should be used when retrieving the
/// application's resources from the resource database.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XAllocClassHint.3.xhtml
pub const XClassHint = X.XClassHint;

/// The structure for ClientMessage events contains:
///
/// ```c
/// typedef struct {
///     int type;             /* ClientMessage */
///     unsigned long serial; /* # of last request processed by server */
///     Bool send_event;      /* true if this came from a SendEvent request */
///     Display *display;     /* Display the event was read from */
///     Window window;
///     Atom message_type;
///     int format;
///     union {
///         char b[20];
///         short s[10];
///         long l[5];
///     } data;
/// } XClientMessageEvent;
/// ```
///
/// The type member is set to the event type constant name that uniquely
/// identifies it. For example, when the X server reports a GraphicsExpose
/// event to a client application, it sends an XGraphicsExposeEvent structure
/// with the type member set to GraphicsExpose. The display member is set to a
/// pointer to the display the event was read on. The send_event member is set
/// to True if the event came from a SendEvent protocol request. The serial
/// member is set from the serial number reported in the protocol but expanded
/// from the 16-bit least-significant bits to a full 32-bit value. The window
/// member is set to the window that is most useful to toolkit dispatchers.
///
/// The message_type member is set to an atom that indicates how the data
/// should be interpreted by the receiving client. The format member is set to
/// 8, 16, or 32 and specifies whether the data should be viewed as a list of
/// bytes, shorts, or longs. The data member is a union that contains the
/// members b, s, and l. The b, s, and l members represent data of twenty 8-bit
/// values, ten 16-bit values, and five 32-bit values. Particular message types
/// might not make use of all these values. The X server places no
/// interpretation on the values in the window, message_type, or data members.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XClientMessageEvent.3.xhtml
pub const XClientMessageEvent = X.XClientMessageEvent;

/// The structure for ConfigureNotify events contains:
///
/// ```c
/// typedef struct {
///     int type;             /* ConfigureNotify */
///     unsigned long serial; /* # of last request processed by server */
///     Bool send_event;      /* true if this came from a SendEvent request */
///     Display *display;     /* Display the event was read from */
///     Window event;
///     Window window;
///     int x, y;
///     int width, height;
///     int border_width;
///     Window above;
///     Bool override_redirect;
/// } XConfigureEvent;
/// ```
///
/// The type member is set to the event type constant name that uniquely
/// identifies it. For example, when the X server reports a GraphicsExpose
/// event to a client application, it sends an XGraphicsExposeEvent structure
/// with the type member set to GraphicsExpose. The display member is set to a
/// pointer to the display the event was read on. The send_event member is set
/// to True if the event came from a SendEvent protocol request. The serial
/// member is set from the serial number reported in the protocol but expanded
/// from the 16-bit least-significant bits to a full 32-bit value. The window
/// member is set to the window that is most useful to toolkit dispatchers.
///
/// The event member is set either to the reconfigured window or to its parent,
/// depending on whether StructureNotify or SubstructureNotify was selected.
/// The window member is set to the window whose size, position, border, and/or
/// stacking order was changed.
///
/// The x and y members are set to the coordinates relative to the parent
/// window's origin and indicate the position of the upper-left outside corner
/// of the window. The width and height members are set to the inside size of
/// the window, not including the border. The border_width member is set to the
/// width of the window's border, in pixels.
///
/// The above member is set to the sibling window and is used for stacking
/// operations. If the X server sets this member to None, the window whose
/// state was changed is on the bottom of the stack with respect to sibling
/// windows. However, if this member is set to a sibling window, the window
/// whose state was changed is placed on top of this sibling window.
///
/// The override_redirect member is set to the override-redirect attribute of
/// the window. Window manager clients normally should ignore this window if
/// the override_redirect member is True.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XConfigureEvent.3.xhtml
pub const XConfigureEvent = X.XConfigureEvent;

/// ```c
/// typedef struct {
///     int type;             /* EnterNotify or LeaveNotify */
///     unsigned long serial; /* # of last request processed by server */
///     Bool send_event;      /* true if this came from a SendEvent request */
///     Display *display;     /* Display the event was read from */
///     Window window;        /* "event" window reported relative to */
///     Window root;          /* root window that the event occurred on */
///     Window subwindow;     /* child window */
///     Time time;            /* milliseconds */
///     int x, y;             /* pointer x, y coordinates in event window */
///     int x_root, y_root;   /* coordinates relative to root */
///     int mode;             /* NotifyNormal, NotifyGrab, NotifyUngrab */
///     int detail;           /*
///                            * NotifyAncestor, NotifyVirtual, NotifyInferior,
///                            * NotifyNonlinear,NotifyNonlinearVirtual
///                            */
///     Bool same_screen;     /* same screen flag */
///     Bool focus;           /* boolean focus */
///     unsigned int state;   /* key or button mask */
/// } XCrossingEvent;
/// ```
///
/// The type member is set to the event type constant name that uniquely
/// identifies it. For example, when the X server reports a GraphicsExpose
/// event to a client application, it sends an XGraphicsExposeEvent structure
/// with the type member set to GraphicsExpose. The display member is set to a
/// pointer to the display the event was read on. The send_event member is set
/// to True if the event came from a SendEvent protocol request. The serial
/// member is set from the serial number reported in the protocol but expanded
/// from the 16-bit least-significant bits to a full 32-bit value. The window
/// member is set to the window that is most useful to toolkit dispatchers.
///
/// The window member is set to the window on which the EnterNotify or
/// LeaveNotify event was generated and is referred to as the event window.
/// This is the window used by the X server to report the event, and is
/// relative to the root window on which the event occurred. The root member is
/// set to the root window of the screen on which the event occurred.
///
/// For a LeaveNotify event, if a child of the event window contains the
/// initial position of the pointer, the subwindow component is set to that
/// child. Otherwise, the X server sets the subwindow member to None. For an
/// EnterNotify event, if a child of the event window contains the final
/// pointer position, the subwindow component is set to that child or None.
///
/// The time member is set to the time when the event was generated and is
/// expressed in milliseconds. The x and y members are set to the coordinates
/// of the pointer position in the event window. This position is always the
/// pointer's final position, not its initial position. If the event window is
/// on the same screen as the root window, x and y are the pointer coordinates
/// relative to the event window's origin. Otherwise, x and y are set to zero.
/// The x_root and y_root members are set to the pointer's coordinates relative
/// to the root window's origin at the time of the event.
///
/// The same_screen member is set to indicate whether the event window is on
/// the same screen as the root window and can be either True or False. If
/// True, the event and root windows are on the same screen. If False, the
/// event and root windows are not on the same screen.
///
/// The focus member is set to indicate whether the event window is the focus
/// window or an inferior of the focus window. The X server can set this member
/// to either True or False. If True, the event window is the focus window or
/// an inferior of the focus window. If False, the event window is not the
/// focus window or an inferior of the focus window.
///
/// The state member is set to indicate the state of the pointer buttons and
/// modifier keys just prior to the event. The X server can set this member to
/// the bitwise inclusive OR of one or more of the button or modifier key
/// masks: Button1Mask, Button2Mask, Button3Mask, Button4Mask, Button5Mask,
/// ShiftMask, LockMask, ControlMask, Mod1Mask, Mod2Mask, Mod3Mask, Mod4Mask,
/// Mod5Mask.
///
/// The mode member is set to indicate whether the events are normal events,
/// pseudo-motion events when a grab activates, or pseudo-motion events when a
/// grab deactivates. The X server can set this member to NotifyNormal,
/// NotifyGrab, or NotifyUngrab.
///
/// The detail member is set to indicate the notify detail and can be
/// NotifyAncestor, NotifyVirtual, NotifyInferior, NotifyNonlinear, or
/// NotifyNonlinearVirtual.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XCrossingEvent.3.xhtml
pub const XCrossingEvent = X.XCrossingEvent;

/// The structure for DestroyNotify events contains:
///
/// ```c
/// typedef struct {
///     int type;             /* DestroyNotify */
///     unsigned long serial; /* # of last request processed by server */
///     Bool send_event;      /* true if this came from a SendEvent request */
///     Display *display;     /* Display the event was read from */
///     Window event;
///     Window window;
/// } XDestroyWindowEvent;
/// ```
///
/// The type member is set to the event type constant name that uniquely
/// identifies it. For example, when the X server reports a GraphicsExpose
/// event to a client application, it sends an XGraphicsExposeEvent structure
/// with the type member set to GraphicsExpose. The display member is set to a
/// pointer to the display the event was read on. The send_event member is set
/// to True if the event came from a SendEvent protocol request. The serial
/// member is set from the serial number reported in the protocol but expanded
/// from the 16-bit least-significant bits to a full 32-bit value. The window
/// member is set to the window that is most useful to toolkit dispatchers.
///
/// The event member is set either to the destroyed window or to its parent,
/// depending on whether StructureNotify or SubstructureNotify was selected.
/// The window member is set to the window that is destroyed.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XDestroyWindowEvent.3.xhtml
pub const XDestroyWindowEvent = X.XDestroyWindowEvent;

/// The XErrorEvent structure contains:
///
/// ```c
/// typedef struct {
///     int type;
///     Display *display;           /* Display the event was read from */
///     unsigned long serial;       /* serial number of failed request */
///     unsigned char error_code;   /* error code of failed request */
///     unsigned char request_code; /* Major op-code of failed request */
///     unsigned char minor_code;   /* Minor op-code of failed request */
///     XID resourceid;             /* resource id */
/// } XErrorEvent;
/// ```
///
/// The serial member is the number of requests, starting from one, sent over
/// the network connection since it was opened. It is the number that was the
/// value of NextRequest immediately before the failing call was made. The
/// request_code member is a protocol request of the procedure that failed, as
/// defined in <X11/Xproto.h>.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XErrorEvent.3.xhtml
pub const XErrorEvent = X.XErrorEvent;

/// An XEvent structure's first entry always is the type member, which is set
/// to the event type. The second member always is the serial number of the
/// protocol request that generated the event. The third member always is
/// send_event, which is a Bool that indicates if the event was sent by a
/// different client. The fourth member always is a display, which is the
/// display that the event was read from. Except for keymap events, the fifth
/// member always is a window, which has been carefully selected to be useful
/// to toolkit dispatchers. To avoid breaking toolkits, the order of these
/// first five entries is not to change. Most events also contain a time
/// member, which is the time at which an event occurred. In addition, a
/// pointer to the generic event must be cast before it is used to access any
/// other information in the structure.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XAnyEvent.3.xhtml
pub const XEvent = X.XEvent;

/// The structure for Expose events contains:
///
/// ```c
/// typedef struct {
///     int type;             /* Expose */
///     unsigned long serial; /* # of last request processed by server */
///     Bool send_event;      /* true if this came from a SendEvent request */
///     Display *display;     /* Display the event was read from */
///     Window window;
///     int x, y;
///     int width, height;
///     int count;            /* if nonzero, at least this many more */
/// } XExposeEvent;
/// ```
///
/// The type member is set to the event type constant name that uniquely
/// identifies it. For example, when the X server reports a GraphicsExpose
/// event to a client application, it sends an XGraphicsExposeEvent structure
/// with the type member set to GraphicsExpose. The display member is set to a
/// pointer to the display the event was read on. The send_event member is set
/// to True if the event came from a SendEvent protocol request. The serial
/// member is set from the serial number reported in the protocol but expanded
/// from the 16-bit least-significant bits to a full 32-bit value. The window
/// member is set to the window that is most useful to toolkit dispatchers.
///
/// The window member is set to the exposed (damaged) window. The x and y
/// members are set to the coordinates relative to the window's origin and
/// indicate the upper-left corner of the rectangle. The width and height
/// members are set to the size (extent) of the rectangle. The count member is
/// set to the number of Expose events that are to follow. If count is zero, no
/// more Expose events follow for this window. However, if count is nonzero, at
/// least that number of Expose events (and possibly more) follow for this
/// window. Simple applications that do not want to optimize redisplay by
/// distinguishing between subareas of its window can just ignore all Expose
/// events with nonzero counts and perform full redisplays on events with zero
/// counts.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XExposeEvent.3.xhtml
pub const XExposeEvent = X.XExposeEvent;

/// The structure for FocusIn and FocusOut events contains:
///
/// ```c
/// typedef struct {
///     int type;             /* FocusIn or FocusOut */
///     unsigned long serial; /* # of last request processed by server */
///     Bool send_event;      /* true if this came from a SendEvent request */
///     Display *display;     /* Display the event was read from */
///     Window window;        /* window of event */
///     int mode;             /* NotifyNormal, NotifyGrab, NotifyUngrab */
///     int detail;           /*
///                            * NotifyAncestor, NotifyVirtual, NotifyInferior,
///                            * NotifyNonlinear,NotifyNonlinearVirtual, NotifyPointer,
///                            * NotifyPointerRoot, NotifyDetailNone
///                            */
/// } XFocusChangeEvent;
/// ```
///
/// The type member is set to the event type constant name that uniquely
/// identifies it. For example, when the X server reports a GraphicsExpose
/// event to a client application, it sends an XGraphicsExposeEvent structure
/// with the type member set to GraphicsExpose. The display member is set to a
/// pointer to the display the event was read on. The send_event member is set
/// to True if the event came from a SendEvent protocol request. The serial
/// member is set from the serial number reported in the protocol but expanded
/// from the 16-bit least-significant bits to a full 32-bit value. The window
/// member is set to the window that is most useful to toolkit dispatchers.
///
/// The window member is set to the window on which the FocusIn or FocusOut
/// event was generated. This is the window used by the X server to report the
/// event. The mode member is set to indicate whether the focus events are
/// normal focus events, focus events while grabbed, focus events when a grab
/// activates, or focus events when a grab deactivates. The X server can set
/// the mode member to NotifyNormal, NotifyWhileGrabbed, NotifyGrab, or
/// NotifyUngrab.
///
/// All FocusOut events caused by a window unmap are generated after any
/// UnmapNotify event; however, the X protocol does not constrain the ordering
/// of FocusOut events with respect to generated EnterNotify, LeaveNotify,
/// VisibilityNotify, and Expose events.
///
/// Depending on the event mode, the detail member is set to indicate the
/// notify detail and can be NotifyAncestor, NotifyVirtual, NotifyInferior,
/// NotifyNonlinear, NotifyNonlinearVirtual, NotifyPointer, NotifyPointerRoot,
/// or NotifyDetailNone.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XFocusChangeEvent.3.xhtml
pub const XFocusChangeEvent = X.XFocusChangeEvent;

/// The XGCValues structure contains:
///
/// ```c
/// typedef struct {
///     int function;             /* logical operation */
///     unsigned long plane_mask; /* plane mask */
///     unsigned long foreground; /* foreground pixel */
///     unsigned long background; /* background pixel */
///     int line_width;           /* line width (in pixels) */
///     int line_style;           /* LineSolid, LineOnOffDash, LineDoubleDash */
///     int cap_style;            /* CapNotLast, CapButt, CapRound, CapProjecting */
///     int join_style;           /* JoinMiter, JoinRound, JoinBevel */
///     int fill_style;           /* FillSolid, FillTiled, FillStippled FillOpaqueStippled*/
///     int fill_rule;            /* EvenOddRule, WindingRule */
///     int arc_mode;             /* ArcChord, ArcPieSlice */
///     Pixmap tile;              /* tile pixmap for tiling operations */
///     Pixmap stipple;           /* stipple 1 plane pixmap for stippling */
///     int ts_x_origin;          /* offset for tile or stipple operations */
///     int ts_y_origin;
///     Font font;                /* default text font for text operations */
///     int subwindow_mode;       /* ClipByChildren, IncludeInferiors */
///     Bool graphics_exposures;  /* boolean, should exposures be generated */
///     int clip_x_origin;        /* origin for clipping */
///     int clip_y_origin;
///     Pixmap clip_mask;         /* bitmap clipping; other calls for rects */
///     int dash_offset;          /* patterned/dashed line information */
///     char dashes;
/// } XGCValues;
///
/// The function attributes of a GC are used when you update a section of a
/// drawable (the destination) with bits from somewhere else (the source). The
/// function in a GC defines how the new destination bits are to be computed
/// from the source bits and the old destination bits. GXcopy is typically the
/// most useful because it will work on a color display, but special
/// applications may use other functions, particularly in concert with
/// particular planes of a color display.
///
/// Many graphics operations depend on either pixel values or planes in a GC.
/// The planes attribute is of type long, and it specifies which planes of the
/// destination are to be modified, one bit per plane. A monochrome display has
/// only one plane and will be the least significant bit of the word. As planes
/// are added to the display hardware, they will occupy more significant bits
/// in the plane mask.
///
/// In graphics operations, given a source and destination pixel, the result is
/// computed bitwise on corresponding bits of the pixels. That is, a Boolean
/// operation is performed in each bit plane. The plane_mask restricts the
/// operation to a subset of planes. A macro constant AllPlanes can be used to
/// refer to all planes of the screen simultaneously. The result is computed by
/// the following:
///
/// ((src FUNC dst) AND plane-mask) OR (dst AND (NOT plane-mask))
///
/// Range checking is not performed on the values for foreground, background,
/// or plane_mask. They are simply truncated to the appropriate number of bits.
/// The line-width is measured in pixels and either can be greater than or
/// equal to one (wide line) or can be the special value zero (thin line).
///
/// Wide lines are drawn centered on the path described by the graphics
/// request. Unless otherwise specified by the join-style or cap-style, the
/// bounding box of a wide line with endpoints [x1, y1], [x2, y2] and width w
/// is a rectangle with vertices at the following real coordinates:
///
/// [x1-(w*sn/2), y1+(w*cs/2)], [x1+(w*sn/2), y1-(w*cs/2)], [x2-(w*sn/2),
/// y2+(w*cs/2)], [x2+(w*sn/2), y2-(w*cs/2)]
///
/// Here sn is the sine of the angle of the line, and cs is the cosine of the
/// angle of the line. A pixel is part of the line and so is drawn if the
/// center of the pixel is fully inside the bounding box (which is viewed as
/// having infinitely thin edges). If the center of the pixel is exactly on the
/// bounding box, it is part of the line if and only if the interior is
/// immediately to its right (x increasing direction). Pixels with centers on a
/// horizontal edge are a special case and are part of the line if and only if
/// the interior or the boundary is immediately below (y increasing direction)
/// and the interior or the boundary is immediately to the right (x increasing
/// direction).
///
/// Thin lines (zero line-width) are one-pixel-wide lines drawn using an
/// unspecified, device-dependent algorithm. There are only two constraints on
/// this algorithm.
///
/// 1. If a line is drawn unclipped from [x1,y1] to [x2,y2] and if another line is
///    drawn unclipped from [x1+dx,y1+dy] to [x2+dx,y2+dy], a point [x,y] is touched
///    by drawing the first line if and only if the point [x+dx,y+dy] is touched by
///    drawing the second line.
///
/// 2. The effective set of points comprising a line cannot be affected by clipping.
///    That is, a point is touched in a clipped line if and only if the point lies
///    inside the clipping region and the point would be touched by the line when
///    drawn unclipped.
///
/// A wide line drawn from [x1,y1] to [x2,y2] always draws the same pixels as a
/// wide line drawn from [x2,y2] to [x1,y1], not counting cap-style and
/// join-style. It is recommended that this property be true for thin lines,
/// but this is not required. A line-width of zero may differ from a line-width
/// of one in which pixels are drawn. This permits the use of many
/// manufacturers' line drawing hardware, which may run many times faster than
/// the more precisely specified wide lines.
///
/// In general, drawing a thin line will be faster than drawing a wide line of
/// width one. However, because of their different drawing algorithms, thin
/// lines may not mix well aesthetically with wide lines. If it is desirable to
/// obtain precise and uniform results across all displays, a client should
/// always use a line-width of one rather than a line-width of zero.
///
/// For a line with coincident endpoints (x1=x2, y1=y2), when the join-style is
/// applied at one or both endpoints, the effect is as if the line was removed
/// from the overall path. However, if the total path consists of or is reduced
/// to a single point joined with itself, the effect is the same as when the
/// cap-style is applied at both endpoints.
///
/// The tile/stipple represents an infinite two-dimensional plane, with the
/// tile/stipple replicated in all dimensions. When that plane is superimposed
/// on the drawable for use in a graphics operation, the upper-left corner of
/// some instance of the tile/stipple is at the coordinates within the drawable
/// specified by the tile/stipple origin. The tile/stipple and clip origins are
/// interpreted relative to the origin of whatever destination drawable is
/// specified in a graphics request. The tile pixmap must have the same root
/// and depth as the GC, or a BadMatch error results. The stipple pixmap must
/// have depth one and must have the same root as the GC, or a BadMatch error
/// results. For stipple operations where the fill-style is FillStippled but
/// not FillOpaqueStippled, the stipple pattern is tiled in a single plane and
/// acts as an additional clip mask to be ANDed with the clip-mask. Although
/// some sizes may be faster to use than others, any size pixmap can be used
/// for tiling or stippling.
///
/// Storing a pixmap in a GC might or might not result in a copy being made. If
/// the pixmap is later used as the destination for a graphics request, the
/// change might or might not be reflected in the GC. If the pixmap is used
/// simultaneously in a graphics request both as a destination and as a tile or
/// stipple, the results are undefined.
///
/// For optimum performance, you should draw as much as possible with the same
/// GC (without changing its components). The costs of changing GC components
/// relative to using different GCs depend on the display hardware and the
/// server implementation. It is quite likely that some amount of GC
/// information will be cached in display hardware and that such hardware can
/// only cache a small number of GCs.
///
/// The dashes value is actually a simplified form of the more general patterns
/// that can be set with XSetDashes. Specifying a value of N is equivalent to
/// specifying the two-element list [N, N] in XSetDashes. The value must be
/// nonzero, or a BadValue error results.
///
/// The clip-mask restricts writes to the destination drawable. If the
/// clip-mask is set to a pixmap, it must have depth one and have the same root
/// as the GC, or a BadMatch error results. If clip-mask is set to None, the
/// pixels are always drawn regardless of the clip origin. The clip-mask also
/// can be set by calling the XSetClipRectangles or XSetRegion functions. Only
/// pixels where the clip-mask has a bit set to 1 are drawn. Pixels are not
/// drawn outside the area covered by the clip-mask or where the clip-mask has
/// a bit set to 0. The clip-mask affects all graphics requests. The clip-mask
/// does not clip sources. The clip-mask origin is interpreted relative to the
/// origin of whatever destination drawable is specified in a graphics request.
///
/// You can set the subwindow-mode to ClipByChildren or IncludeInferiors. For
/// ClipByChildren, both source and destination windows are additionally
/// clipped by all viewable InputOutput children. For IncludeInferiors, neither
/// source nor destination window is clipped by inferiors. This will result in
/// including subwindow contents in the source and drawing through subwindow
/// boundaries of the destination. The use of IncludeInferiors on a window of
/// one depth with mapped inferiors of differing depth is not illegal, but the
/// semantics are undefined by the core protocol.
///
/// The fill-rule defines what pixels are inside (drawn) for paths given in
/// XFillPolygon requests and can be set to EvenOddRule or WindingRule. For
/// EvenOddRule, a point is inside if an infinite ray with the point as origin
/// crosses the path an odd number of times. For WindingRule, a point is inside
/// if an infinite ray with the point as origin crosses an unequal number of
/// clockwise and counterclockwise directed path segments. A clockwise directed
/// path segment is one that crosses the ray from left to right as observed
/// from the point. A counterclockwise segment is one that crosses the ray from
/// right to left as observed from the point. The case where a directed line
/// segment is coincident with the ray is uninteresting because you can simply
/// choose a different ray that is not coincident with a segment.
///
/// For both EvenOddRule and WindingRule, a point is infinitely small, and the
/// path is an infinitely thin line. A pixel is inside if the center point of
/// the pixel is inside and the center point is not on the boundary. If the
/// center point is on the boundary, the pixel is inside if and only if the
/// polygon interior is immediately to its right (x increasing direction).
/// Pixels with centers on a horizontal edge are a special case and are inside
/// if and only if the polygon interior is immediately below (y increasing
/// direction).
///
/// The arc-mode controls filling in the XFillArcs function and can be set to
/// ArcPieSlice or ArcChord. For ArcPieSlice, the arcs are pie-slice filled.
/// For ArcChord, the arcs are chord filled.
///
/// The graphics-exposure flag controls GraphicsExpose event generation for
/// XCopyArea and XCopyPlane requests (and any similar requests defined by
/// extensions).
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XCreateGC.3.xhtml
pub const XGCValues = X.XGCValues;

/// Glyphs are stored in the server, so these definitions are passed from the
/// client to the library and on to the server as glyphs are rasterized and
/// transmitted over the wire.
///
/// ```c
/// typedef struct _XGlyphInfo {
///     unsigned short  width;
///     unsigned short  height;
///     short           x;
///     short           y;
///     short           xOff;
///     short           yOff;
/// } XGlyphInfo;
/// ```
///
/// source: https://x.org/releases/X11R7.7/doc/libXrender/libXrender.txt
pub const XGlyphInfo = X.XGlyphInfo;

/// ```c
/// typedef struct {
///     int type;             /* KeyPress or KeyRelease */
///     unsigned long serial; /* # of last request processed by server */
///     Bool send_event;      /* true if this came from a SendEvent request */
///     Display *display;     /* Display the event was read from */
///     Window window;        /* "event" window it is reported relative to */
///     Window root;          /* root window that the event occurred on */
///     Window subwindow;     /* child window */
///     Time time;            /* milliseconds */
///     int x, y;             /* pointer x, y coordinates in event window */
///     int x_root, y_root;   /* coordinates relative to root */
///     unsigned int state;   /* key or button mask */
///     unsigned int keycode; /* detail */
///     Bool same_screen;     /* same screen flag */
/// } XKeyEvent;
/// ```
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XButtonEvent.3.xhtml
pub const XKeyEvent = X.XKeyEvent;

/// The structure for MappingNotify events is:
///
/// ```c
/// typedef struct {
///     int type;             /* MappingNotify */
///     unsigned long serial; /* # of last request processed by server */
///     Bool send_event;      /* true if this came from a SendEvent request */
///     Display *display;     /* Display the event was read from */
///     Window window;        /* unused */
///     int request;          /* one of MappingModifier, MappingKeyboard, MappingPointer */
///     int first_keycode;    /* first keycode */
///     int count;            /* defines range of change w. first_keycode*/
/// } XMappingEvent;
/// ```
///
/// The type member is set to the event type constant name that uniquely
/// identifies it. For example, when the X server reports a GraphicsExpose
/// event to a client application, it sends an XGraphicsExposeEvent structure
/// with the type member set to GraphicsExpose. The display member is set to a
/// pointer to the display the event was read on. The send_event member is set
/// to True if the event came from a SendEvent protocol request. The serial
/// member is set from the serial number reported in the protocol but expanded
/// from the 16-bit least-significant bits to a full 32-bit value. The window
/// member is set to the window that is most useful to toolkit dispatchers.
///
/// The request member is set to indicate the kind of mapping change that
/// occurred and can be MappingModifier, MappingKeyboard, MappingPointer. If it
/// is MappingModifier, the modifier mapping was changed. If it is
/// MappingKeyboard, the keyboard mapping was changed. If it is MappingPointer,
/// the pointer button mapping was changed. The first_keycode and count members
/// are set only if the request member was set to MappingKeyboard. The number
/// in first_keycode represents the first number in the range of the altered
/// mapping, and count represents the number of keycodes altered.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XMapEvent.3.xhtml
pub const XMappingEvent = X.XMappingEvent;

/// The structure for MapRequest events contains:
///
/// ```c
/// typedef struct {
///     int type;             /* MapRequest */
///     unsigned long serial; /* # of last request processed by server */
///     Bool send_event;      /* true if this came from a SendEvent request */
///     Display *display;     /* Display the event was read from */
///     Window parent;
///     Window window;
/// } XMapRequestEvent;
/// ```
///
/// When you receive this event, the structure members are set as follows.
///
/// The type member is set to the event type constant name that uniquely
/// identifies it. For example, when the X server reports a GraphicsExpose
/// event to a client application, it sends an XGraphicsExposeEvent structure
/// with the type member set to GraphicsExpose. The display member is set to a
/// pointer to the display the event was read on. The send_event member is set
/// to True if the event came from a SendEvent protocol request. The serial
/// member is set from the serial number reported in the protocol but expanded
/// from the 16-bit least-significant bits to a full 32-bit value. The window
/// member is set to the window that is most useful to toolkit dispatchers.
///
/// The parent member is set to the parent window. The window member is set to
/// the window to be mapped.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XMapRequestEvent.3.xhtml
pub const XMapRequestEvent = X.XMapRequestEvent;

/// The XModifierKeymap structure contains:
///
/// ```c
/// typedef struct {
///     int max_keypermod;    /* This server's max number of keys per modifier */
///     KeyCode *modifiermap; /* An 8 by max_keypermod array of the modifiers */
/// } XModifierKeymap;
/// ```
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XChangeKeyboardMapping.3.xhtml
pub const XModifierKeymap = X.XModifierKeymap;

/// ```c
/// typedef struct {
///     int type;             /* MotionNotify */
///     unsigned long serial; /* # of last request processed by server */
///     Bool send_event;      /* true if this came from a SendEvent request */
///     Display *display;     /* Display the event was read from */
///     Window window;        /* "event" window reported relative to */
///     Window root;          /* root window that the event occurred on */
///     Window subwindow;     /* child window */
///     Time time;            /* milliseconds */
///     int x, y;             /* pointer x, y coordinates in event window */
///     int x_root, y_root;   /* coordinates relative to root */
///     unsigned int state;   /* key or button mask */
///     char is_hint;         /* detail */
///     Bool same_screen;     /* same screen flag */
/// } XMotionEvent;
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XButtonEvent.3.xhtml
pub const XMotionEvent = X.XMotionEvent;

/// The structure for PropertyNotify events contains:
///
/// ```c
/// typedef struct {
///     int type;             /* PropertyNotify */
///     unsigned long serial; /* # of last request processed by server */
///     Bool send_event;      /* true if this came from a SendEvent request */
///     Display *display;     /* Display the event was read from */
///     Window window;
///     Atom atom;
///     Time time;
///     int state;            /* PropertyNewValue or PropertyDelete */
/// } XPropertyEvent;
/// ```
///
/// When you receive this event, the structure members are set as follows.
///
/// The type member is set to the event type constant name that uniquely
/// identifies it. For example, when the X server reports a GraphicsExpose
/// event to a client application, it sends an XGraphicsExposeEvent structure
/// with the type member set to GraphicsExpose. The display member is set to a
/// pointer to the display the event was read on. The send_event member is set
/// to True if the event came from a SendEvent protocol request. The serial
/// member is set from the serial number reported in the protocol but expanded
/// from the 16-bit least-significant bits to a full 32-bit value. The window
/// member is set to the window that is most useful to toolkit dispatchers.
///
/// The window member is set to the window whose associated property was
/// changed. The atom member is set to the property's atom and indicates which
/// property was changed or desired. The time member is set to the server time
/// when the property was changed. The state member is set to indicate whether
/// the property was changed to a new value or deleted and can be
/// PropertyNewValue or PropertyDelete. The state member is set to
/// PropertyNewValue when a property of the window is changed using
/// XChangeProperty or XRotateWindowProperties (even when adding zero-length
/// data using XChangeProperty) and when replacing all or part of a property
/// with identical data using XChangeProperty or XRotateWindowProperties. The
/// state member is set to PropertyDelete when a property of the window is
/// deleted using XDeleteProperty or, if the delete argument is True,
/// XGetWindowProperty.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XPropertyEvent.3.xhtml
pub const XPropertyEvent = X.XPropertyEvent;

/// The XSetWindowAttributes structure contains:
///
/// ```c
/// typedef struct {
///     Pixmap background_pixmap;/* background, None, or ParentRelative */
///     unsigned long background_pixel;/* background pixel */
///     Pixmap border_pixmap; /* border of the window or CopyFromParent */
///     unsigned long border_pixel;/* border pixel value */
///     int bit_gravity; /* one of bit gravity values */
///     int win_gravity; /* one of the window gravity values */
///     int backing_store; /* NotUseful, WhenMapped, Always */
///     unsigned long backing_planes;/* planes to be preserved if possible */
///     unsigned long backing_pixel;/* value to use in restoring planes */
///     Bool save_under; /* should bits under be saved? (popups) */
///     long event_mask; /* set of events that should be saved */
///     long do_not_propagate_mask;/* set of events that should not propagate */
///     Bool override_redirect; /* boolean value for override_redirect */
///     Colormap colormap; /* color map to be associated with window */
///     Cursor cursor; /* cursor to be displayed (or None) */
/// } XSetWindowAttributes;
/// ```
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XCreateWindow.3.xhtml
pub const XSetWindowAttributes = X.XSetWindowAttributes;

/// The XSizeHints structure contains:
///
/// ```c
/// typedef struct {
///     long flags;        /* marks which fields in this structure are defined */
///     int x, y;          /* Obsolete */
///     int width, height; /* Obsolete */
///     int min_width, min_height;
///     int max_width, max_height;
///     int width_inc, height_inc;
///     struct {
///         int x;         /* numerator */
///         int y;         /* denominator */
///     } min_aspect, max_aspect;
///     int base_width, base_height;
///     int win_gravity;   /* this structure may be extended in the future */
/// } XSizeHints;
/// ```
///
/// The x, y, width, and height members are now obsolete and are left solely
/// for compatibility reasons. The min_width and min_height members specify the
/// minimum window size that still allows the application to be useful. The
/// max_width and max_height members specify the maximum window size. The
/// width_inc and height_inc members define an arithmetic progression of sizes
/// (minimum to maximum) into which the window prefers to be resized. The
/// min_aspect and max_aspect members are expressed as ratios of x and y, and
/// they allow an application to specify the range of aspect ratios it prefers.
/// The base_width and base_height members define the desired size of the
/// window. The window manager will interpret the position of the window and
/// its border width to position the point of the outer rectangle of the
/// overall window specified by the win_gravity member. The outer rectangle of
/// the window includes any borders or decorations supplied by the window
/// manager. In other words, if the window manager decides to place the window
/// where the client asked, the position on the parent window's border named by
/// the win_gravity will be placed where the client window would have been
/// placed in the absence of a window manager.
///
/// Note that use of the PAllHints macro is highly discouraged.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XAllocSizeHints.3.xhtml
pub const XSizeHints = X.XSizeHints;

/// The XTextProperty structure contains:
///
/// ```c
/// typedef struct {
///     unsigned char *value; /* property data */
///     Atom encoding;        /* type of property */
///     int format;           /* 8, 16, or 32 */
///     unsigned long nitems; /* number of items in value */
/// } XTextProperty;
/// ```
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XStringListToTextProperty.3.xhtml
pub const XTextProperty = X.XTextProperty;

/// The structure for UnmapNotify events contains:
///
/// ```c
/// typedef struct {
///     int type; /* UnmapNotify */
///     unsigned long serial; /* # of last request processed by server */
///     Bool send_event; /* true if this came from a SendEvent request */
///     Display *display; /* Display the event was read from */
///     Window event;
///     Window window;
///     Bool from_configure;
/// } XUnmapEvent;
/// ```
///
/// The type member is set to the event type constant name that uniquely
/// identifies it. For example, when the X server reports a GraphicsExpose
/// event to a client application, it sends an XGraphicsExposeEvent structure
/// with the type member set to GraphicsExpose. The display member is set to a
/// pointer to the display the event was read on. The send_event member is set
/// to True if the event came from a SendEvent protocol request. The serial
/// member is set from the serial number reported in the protocol but expanded
/// from the 16-bit least-significant bits to a full 32-bit value. The window
/// member is set to the window that is most useful to toolkit dispatchers.
///
/// The event member is set either to the unmapped window or to its parent,
/// depending on whether StructureNotify or SubstructureNotify was selected.
/// This is the window used by the X server to report the event. The window
/// member is set to the window that was unmapped. The from_configure member is
/// set to True if the event was generated as a result of a resizing of the
/// window's parent when the window itself had a win_gravity of UnmapGravity.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XUnmapEvent.3.xhtml
pub const XUnmapEvent = X.XUnmapEvent;

/// The XWindowAttributes structure contains:
///
/// ```c
/// typedef struct {
///     int x, y; /* location of window */
///     int width, height; /* width and height of window */
///     int border_width; /* border width of window */
///     int depth; /* depth of window */
///     Visual *visual; /* the associated visual structure */
///     Window root; /* root of screen containing window */
///     int class; /* InputOutput, InputOnly*/
///     int bit_gravity; /* one of the bit gravity values */
///     int win_gravity; /* one of the window gravity values */
///     int backing_store; /* NotUseful, WhenMapped, Always */
///     unsigned long backing_planes;/* planes to be preserved if possible */
///     unsigned long backing_pixel;/* value to be used when restoring planes */
///     Bool save_under; /* boolean, should bits under be saved? */
///     Colormap colormap; /* color map to be associated with window */
///     Bool map_installed; /* boolean, is color map currently installed*/
///     int map_state; /* IsUnmapped, IsUnviewable, IsViewable */
///     long all_event_masks; /* set of events all people have interest in*/
///     long your_event_mask; /* my event mask */
///     long do_not_propagate_mask;/* set of events that should not propagate */
///     Bool override_redirect; /* boolean value for override-redirect */
///     Screen *screen; /* back pointer to correct screen */
/// } XWindowAttributes;
/// ```
///
/// The x and y members are set to the upper-left outer corner relative to the
/// parent window's origin. The width and height members are set to the inside
/// size of the window, not including the border. The border_width member is
/// set to the window's border width in pixels. The depth member is set to the
/// depth of the window (that is, bits per pixel for the object). The visual
/// member is a pointer to the screen's associated Visual structure. The root
/// member is set to the root window of the screen containing the window. The
/// class member is set to the window's class and can be either InputOutput or
/// InputOnly.
///
/// For additional information on gravity, see section 3.3.
///
/// The backing_store member is set to indicate how the X server should
/// maintain the contents of a window and can be WhenMapped, Always, or
/// NotUseful. The backing_planes member is set to indicate (with bits set to
/// 1) which bit planes of the window hold dynamic data that must be preserved
/// in backing_stores and during save_unders. The backing_pixel member is set
/// to indicate what values to use for planes not set in backing_planes.
///
/// The save_under member is set to True or False. The colormap member is set
/// to the colormap for the specified window and can be a colormap ID or None.
/// The map_installed member is set to indicate whether the colormap is
/// currently installed and can be True or False. The map_state member is set
/// to indicate the state of the window and can be IsUnmapped, IsUnviewable, or
/// IsViewable. IsUnviewable is used if the window is mapped but some ancestor
/// is unmapped.
///
/// The all_event_masks member is set to the bitwise inclusive OR of all event
/// masks selected on the window by all clients. The your_event_mask member is
/// set to the bitwise inclusive OR of all event masks selected by the querying
/// client. The do_not_propagate_mask member is set to the bitwise inclusive OR
/// of the set of events that should not propagate.
///
/// The override_redirect member is set to indicate whether this window
/// overrides structure control facilities and can be True or False. Window
/// manager clients should ignore the window if this member is True.
///
/// The screen member is set to a screen pointer that gives you a back pointer
/// to the correct screen. This makes it easier to obtain the screen
/// information without having to loop over the root window fields to see which
/// field matches.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XGetWindowAttributes.3.xhtml
pub const XWindowAttributes = X.XWindowAttributes;

/// The XWindowChanges structure contains:
///
/// ```c
/// typedef struct {
///     int x, y;
///     int width, height;
///     int border_width;
///     Window sibling;
///     int stack_mode;
/// } XWindowChanges;
/// ```
///
/// The x and y members are used to set the window's x and y coordinates, which
/// are relative to the parent's origin and indicate the position of the
/// upper-left outer corner of the window. The width and height members are
/// used to set the inside size of the window, not including the border, and
/// must be nonzero, or a BadValue error results. Attempts to configure a root
/// window have no effect.
///
/// The border_width member is used to set the width of the border in pixels.
/// Note that setting just the border width leaves the outer-left corner of the
/// window in a fixed position but moves the absolute position of the window's
/// origin. If you attempt to set the border-width attribute of an InputOnly
/// window nonzero, a BadMatch error results.
///
/// The sibling member is used to set the sibling window for stacking
/// operations. The stack_mode member is used to set how the window is to be
/// restacked and can be set to Above, Below, TopIf, BottomIf, or Opposite.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XConfigureWindow.3.xhtml
pub const XWindowChanges = X.XWindowChanges;

/// X Window Manager Hints.
///
/// ```c
/// typedef struct {
///     long flags; /* marks which fields in this structure are defined */
///     Bool input; /* does this application rely on the window manager to get keyboard input? */
///     int initial_state; /* see below */
///     Pixmap icon_pixmap; /* pixmap to be used as icon */
///     Window icon_window; /* window to be used as icon */
///     int icon_x, icon_y; /* initial position of icon */
///     Pixmap icon_mask; /* pixmap to be used as mask for icon_pixmap */
///     XID window_group; /* id of related window group */
///     /* this structure may be extended in the future */
/// } XWMHints;
/// ```
///
/// The input member is used to communicate to the window manager the input
/// focus model used by the application. Applications that expect input but
/// never explicitly set focus to any of their subwindows (that is, use the
/// push model of focus management), such as X Version 10 style applications
/// that use real-estate driven focus, should set this member to True.
/// Similarly, applications that set input focus to their subwindows only when
/// it is given to their top-level window by a window manager should also set
/// this member to True. Applications that manage their own input focus by
/// explicitly setting focus to one of their subwindows whenever they want
/// keyboard input (that is, use the pull model of focus management) should set
/// this member to False. Applications that never expect any keyboard input
/// also should set this member to False.
///
/// Pull model window managers should make it possible for push model
/// applications to get input by setting input focus to the top-level windows
/// of applications whose input member is True. Push model window managers
/// should make sure that pull model applications do not break them by
/// resetting input focus to PointerRoot when it is appropriate (for example,
/// whenever an application whose input member is False sets input focus to one
/// of its subwindows).
///
/// The icon_mask specifies which pixels of the icon_pixmap should be used as
/// the icon. This allows for nonrectangular icons. Both icon_pixmap and
/// icon_mask must be bitmaps. The icon_window lets an application provide a
/// window for use as an icon for window managers that support such use. The
/// window_group lets you specify that this window belongs to a group of other
/// windows. For example, if a single application manipulates multiple
/// top-level windows, this allows you to provide enough information that a
/// window manager can iconify all of the windows rather than just the one
/// window.
///
/// The UrgencyHint flag, if set in the flags field, indicates that the client
/// deems the window contents to be urgent, requiring the timely response of
/// the user. The window manager will make some effort to draw the user's
/// attention to this window while this flag is set. The client must provide
/// some means by which the user can cause the urgency flag to be cleared
/// (either mitigating the condition that made the window urgent or merely
/// shutting off the alarm) or the window to be withdrawn.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XAllocWMHints.3.xhtml
pub const XWMHints = X.XWMHints;

/// An XftColor object permits text and other items to be rendered in a
/// particular color (or the closest approximation offered by the X visual in
/// use). The XRenderColor data type is defined by the X Render Extension
/// library.
///
/// XftColorAllocName() and XftColorAllocValue() request a color allocation
/// from the X server (if necessary) and initialize the members of XftColor.
/// XftColorFree() instructs the X server to free the color currently allocated
/// for an XftColor.
///
/// One an XftColor has been initialized, XftDrawSrcPicture(), XftDrawGlyphs(),
/// the XftDrawString*() family, XftDrawCharSpec(), XftDrawCharFontSpec(),
/// XftDrawGlyphSpec(), XftDrawGlyphFontSpec(), and XftDrawRect() may be used
/// to draw various objects using it.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/Xft.3.xhtml
pub const XftColor = X.XftColor;

/// It's an opaque object which holds information used to render to an X
/// drawable using either the core protocol or the X Rendering extension.
///
/// XftDraw objects are created with any of XftDrawCreate() (which associates
/// an XftDraw with an existing X drawable), XftDrawCreateBitmap(), or
/// XftDrawCreateAlpha(), and destroyed with XftDrawDestroy(). The X drawable
/// associated with an XftDraw can be changed with XftDrawChange(). XftDraw
/// objects are internally allocated and freed by Xft; the programmer does not
/// ordinarily need to allocate or free storage for them.
///
/// The X Display, Drawable, Colormap, and Visual properties of an XftDraw can
/// be queried with XftDrawDisplay(), XftDrawDrawable(), XftDrawColormap(), and
/// XftDrawVisual(), respectively.
///
/// Several functions use XftDraw objects: XftDrawCharFontSpec(),
/// XftDrawCharSpec(), XftDrawGlyphFontSpec(), XftDrawGlyphSpec(),
/// XftDrawGlyphs(), XftDrawRect(), XftDrawSetClip(),
/// XftDrawSetClipRectangles(), XftDrawSetSubwindowMode(), and the
/// XftDrawString*() family.
///
/// The X Rendering Extension Picture associated with an XftDraw is returned by
/// XftDrawPicture(), and XftDrawSrcPicture(). It is used by
/// XftCharFontSpecRender(), XftCharSpecRender(), XftGlyphFontSpecRender(),
/// XftGlyphRender(), XftGlyphSpecRender(), and the XftTextRender*() family.
///
/// source: https://man.archlinux.org/man/XftColorAllocName.3
pub const XftDraw = X.XftDraw;

/// An XftFont is the primary data structure of interest to programmers using
/// Xft; it contains general font metrics and pointers to the Fontconfig
/// character set and pattern associated with the font. The FcCharSet and
/// FcPattern data types are defined by the Fontconfig library.
///
/// XftFonts are populated with any of XftFontOpen(), XftFontOpenName(),
/// XftFontOpenXlfd(), XftFontOpenInfo(), or XftFontOpenPattern().
/// XftFontCopy() is used to duplicate XftFonts, and XftFontClose() is used to
/// mark an XftFont as unused. XftFonts are internally allocated,
/// reference-counted, and freed by Xft; the programmer does not ordinarily
/// need to allocate or free storage for them.
///
/// XftDrawGlyphs(), the XftDrawString*() family, XftDrawCharSpec(), and
/// XftDrawGlyphSpec() use XftFonts to render text to an XftDraw object, which
/// may correspond to either a core X drawable or an X Rendering Extension
/// drawable.
///
/// XftGlyphExtents() and the XftTextExtents*() family are used to determine
/// the extents (maximum dimensions) of an XftFont.
///
/// An XftFont's glyph or character coverage can be determined with
/// XftFontCheckGlyph() or XftCharExists(). XftCharIndex() returns the
/// XftFont-specific character index corresponding to a given Unicode
/// codepoint.
///
/// XftGlyphRender(), XftGlyphSpecRender(), XftCharSpecRender(), and the
/// XftTextRender*() family use XftFonts to draw into X Rendering Extension
/// Picture structures. Note: XftDrawGlyphs(), the XftDrawString*() family,
/// XftDrawCharSpec(), and XftDrawGlyphSpec() provide a means of rendering
/// fonts that is independent of the availability of the X Rendering Extension
/// on the X server.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/Xft.3.xhtml
pub const XftFont = X.XftFont;

// -----------------------------------------------------------------------------
// ++ Functions
// -----------------------------------------------------------------------------

/// The XAllowEvents function releases some queued events if the client has
/// caused a device to freeze. It has no effect if the specified time is
/// earlier than the last-grab time of the most recent active grab for the
/// client or if the specified time is later than the current X server time.
///
/// XAllowEvents can generate a BadValue error.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XAllowEvents.3.xhtml
pub inline fn XAllowEvents(display: *Display, event_mode: EventMode, time: Time) void {
    // Meaning of return value is not specified in documentation.
    _ = X.XAllowEvents(display, @intFromEnum(event_mode), time);
}

/// The XChangeProperty function alters the property for the specified window
/// and causes the X server to generate a PropertyNotify event on that window.
/// XChangeProperty performs the following:
///
/// * If mode is PropModeReplace, XChangeProperty discards the previous
///   property value and stores the new data.
///
/// * If mode is PropModePrepend or PropModeAppend, XChangeProperty inserts the
///   specified data before the beginning of the existing data or onto the end
///   of the existing data, respectively. The type and format must match the
///   existing property value, or a BadMatch error results. If the property is
///   undefined, it is treated as defined with the correct type and format with
///   zero-length data.
///
/// If the specified format is 8, the property data must be a char array. If
/// the specified format is 16, the property data must be a short array. If the
/// specified format is 32, the property data must be a long array.
///
/// The lifetime of a property is not tied to the storing client. Properties
/// remain until explicitly deleted, until the window is destroyed, or until
/// the server resets. For a discussion of what happens when the connection to
/// the X server is closed, see section 2.6. The maximum size of a property is
/// server dependent and can vary dynamically depending on the amount of memory
/// the server has available. (If there is insufficient space, a BadAlloc error
/// results.)
///
/// XChangeProperty can generate BadAlloc, BadAtom, BadMatch, BadValue, and
/// BadWindow errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XGetWindowProperty.3.xhtml
pub inline fn XChangeProperty(
    display: *Display,
    window: Window,
    property: Atom,
    p_type: Atom,
    format: c_int,
    mode: PropMode,
    data: [*c]const u8,
    nelements: c_int,
) void {
    // Meaning of return value is not specified in documentation.
    _ = X.XChangeProperty(
        display,
        window,
        property,
        p_type,
        format,
        @intFromEnum(mode),
        data,
        nelements,
    );
}

/// Depending on the valuemask, the XChangeWindowAttributes function uses the
/// window attributes in the XSetWindowAttributes structure to change the
/// specified window attributes. Changing the background does not cause the
/// window contents to be changed. To repaint the window and its background,
/// use XClearWindow. Setting the border or changing the background such that
/// the border tile origin changes causes the border to be repainted. Changing
/// the background of a root window to None or ParentRelative restores the
/// default background pixmap. Changing the border of a root window to
/// CopyFromParent restores the default border pixmap. Changing the win-gravity
/// does not affect the current position of the window. Changing the
/// backing-store of an obscured window to WhenMapped or Always, or changing
/// the backing-planes, backing-pixel, or save-under of a mapped window may
/// have no immediate effect. Changing the colormap of a window (that is,
/// defining a new map, not changing the contents of the existing map)
/// generates a ColormapNotify event. Changing the colormap of a visible window
/// may have no immediate effect on the screen because the map may not be
/// installed (see XInstallColormap). Changing the cursor of a root window to
/// None restores the default cursor. Whenever possible, you are encouraged to
/// share colormaps.
///
/// Multiple clients can select input on the same window. Their event masks are
/// maintained separately. When an event is generated, it is reported to all
/// interested clients. However, only one client at a time can select for
/// SubstructureRedirectMask, ResizeRedirectMask, and ButtonPressMask. If a
/// client attempts to select any of these event masks and some other client
/// has already selected one, a BadAccess error results. There is only one
/// do-not-propagate-mask for a window, not one per client.
///
/// XChangeWindowAttributes can generate BadAccess, BadColor, BadCursor,
/// BadMatch, BadPixmap, BadValue, and BadWindow errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XChangeWindowAttributes.3.xhtml
pub inline fn XChangeWindowAttributes(
    display: *Display,
    window: Window,
    /// Specifies which window attributes are defined in the attributes
    /// argument. This mask is the bitwise inclusive OR of the valid attribute
    /// mask bits. If valuemask is zero, the attributes are ignored and are not
    /// referenced.
    valuemask: c_ulong,
    /// Specifies the structure from which the values (as specified by the
    /// value mask) are to be taken. The value mask should have the appropriate
    /// bits set to indicate which attributes have been set in the structure.
    attributes: *XSetWindowAttributes,
) void {
    // The meaning of the return value was not specified in documentation.
    _ = X.XChangeWindowAttributes(display, window, valuemask, attributes);
}

/// The XCheckMaskEvent function searches the event queue and then any events
/// available on the server connection for the first event that matches the
/// specified mask. If it finds a match, XCheckMaskEvent removes that event,
/// copies it into the specified XEvent structure, and returns True. The other
/// events stored in the queue are not discarded. If the event you requested is
/// not available, XCheckMaskEvent returns False, and the output buffer will
/// have been flushed.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XNextEvent.3.xhtml
pub inline fn XCheckMaskEvent(
    display: *Display,
    event_mask: c_long,
    event_return: *XEvent,
) bool {
    return X.XCheckMaskEvent(display, event_mask, event_return) != X.False;
}

/// The XCloseDisplay function closes the connection to the X server for the
/// display specified in the Display structure and destroys all windows,
/// resource IDs (Window, Font, Pixmap, Colormap, Cursor, and GContext), or
/// other resources that the client has created on this display, unless the
/// close-down mode of the resource has been changed (see XSetCloseDownMode).
/// Therefore, these windows, resource IDs, and other resources should never be
/// referenced again or an error will be generated. Before exiting, you should
/// call XCloseDisplay explicitly so that any pending errors are reported as
/// XCloseDisplay performs a final XSync operation.
///
/// XCloseDisplay can generate a BadGC error.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XOpenDisplay.3.xhtml
pub inline fn XCloseDisplay(display: *Display) void {
    // There is no mention in the docs on that the return value of XCloseDisplay
    // signifies, hence we discard it.
    _ = X.XCloseDisplay(display);
}

/// The XConfigureWindow function uses the values specified in the
/// XWindowChanges structure to reconfigure a window's size, position, border,
/// and stacking order. Values not specified are taken from the existing
/// geometry of the window.
///
/// If a sibling is specified without a stack_mode or if the window is not
/// actually a sibling, a BadMatch error results. Note that the computations
/// for BottomIf, TopIf, and Opposite are performed with respect to the
/// window's final geometry (as controlled by the other arguments passed to
/// XConfigureWindow), not its initial geometry. Any backing store contents of
/// the window, its inferiors, and other newly visible windows are either
/// discarded or changed to reflect the current screen contents (depending on
/// the implementation).
///
/// XConfigureWindow can generate BadMatch, BadValue, and BadWindow errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XOpenDisplay.3.xhtml
pub inline fn XConfigureWindow(
    display: *Display,
    window: Window,
    value_mask: c_uint,
    changes: *XWindowChanges,
) void {
    // The meaning of the return value was not specified in documentation.
    _ = X.XConfigureWindow(display, window, value_mask, changes);
}

/// The XCopyArea function combines the specified rectangle of src with the
/// specified rectangle of dest. The drawables must have the same root and
/// depth, or a BadMatch error results.
///
/// If regions of the source rectangle are obscured and have not been retained
/// in backing store or if regions outside the boundaries of the source
/// drawable are specified, those regions are not copied. Instead, the
/// following occurs on all corresponding destination regions that are either
/// visible or are retained in backing store. If the destination is a window
/// with a background other than None, corresponding regions of the destination
/// are tiled with that background (with plane-mask of all ones and GXcopy
/// function). Regardless of tiling or whether the destination is a window or a
/// pixmap, if graphics-exposures is True, then GraphicsExpose events for all
/// corresponding destination regions are generated. If graphics-exposures is
/// True but no GraphicsExpose events are generated, a NoExpose event is
/// generated. Note that by default graphics-exposures is True in new GCs.
///
/// This function uses these GC components: function, plane-mask,
/// subwindow-mode, graphics-exposures, clip-x-origin, clip-y-origin, and
/// clip-mask.
///
/// XCopyArea can generate BadDrawable, BadGC, and BadMatch errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XCopyArea.3.xhtml
pub inline fn XCopyArea(
    display: *Display,
    src_drw: Drawable,
    dest_drw: Drawable,
    gc: GC,
    src: Rect,
    dest: Coordinates(c_int),
) void {
    // The meaning of the return value was not specified in documentation.
    _ = X.XCopyArea(
        display,
        src_drw,
        dest_drw,
        gc,
        src.x,
        src.y,
        src.w,
        src.h,
        dest.x,
        dest.y,
    );
}

/// X provides a set of standard cursor shapes in a special font named cursor.
/// Applications are encouraged to use this interface for their cursors because
/// the font can be customized for the individual display type. The shape
/// argument specifies which glyph of the standard fonts to use.
///
/// The hotspot comes from the information stored in the cursor font. The
/// initial colors of a cursor are a black foreground and a white background
/// (see XRecolorCursor).
///
/// XCreateFontCursor can generate BadAlloc and BadValue errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XCreateFontCursor.3.xhtml
pub inline fn XCreateFontCursor(display: *Display, shape: PointerShape) Cursor {
    return X.XCreateFontCursor(display, @intFromEnum(shape));
}

/// The XCreateGC function creates a graphics context and returns a GC. The GC
/// can be used with any destination drawable having the same root and depth as
/// the specified drawable. Use with other drawables results in a BadMatch
/// error.
///
/// XCreateGC can generate BadAlloc, BadDrawable, BadFont, BadMatch, BadPixmap,
/// and BadValue errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XCreateGC.3.xhtml
pub inline fn XCreateGC(
    display: *Display,
    drawable: Drawable,
    valuemask: c_ulong,
    values: *XGCValues,
) GC {
    return X.XCreateGC(display, drawable, valuemask, values);
}

/// The XCreatePixmap function creates a pixmap of the width, height, and depth
/// you specified and returns a pixmap ID that identifies it. It is valid to
/// pass an InputOnly window to the drawable argument. The width and height
/// arguments must be nonzero, or a BadValue error results. The depth argument
/// must be one of the depths supported by the screen of the specified
/// drawable, or a BadValue error results.
///
/// The server uses the specified drawable to determine on which screen to
/// create the pixmap. The pixmap can be used only on this screen and only with
/// other drawables of the same depth (see XCopyPlane for an exception to this
/// rule). The initial contents of the pixmap are undefined.
///
/// XCreatePixmap can generate BadAlloc, BadDrawable, and BadValue errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XCreatePixmap.3.xhtml
pub inline fn XCreatePixmap(
    display: *Display,
    drawable: Drawable,
    width: c_uint,
    height: c_uint,
    depth: c_uint,
) Pixmap {
    return X.XCreatePixmap(display, drawable, width, height, depth);
}

/// The XCreateWindow function creates an unmapped subwindow for a specified
/// parent window, returns the window ID of the created window, and causes the
/// X server to generate a CreateNotify event. The created window is placed on
/// top in the stacking order with respect to siblings.
///
/// The coordinate system has the X axis horizontal and the Y axis vertical
/// with the origin [0, 0] at the upper-left corner. Coordinates are integral,
/// in terms of pixels, and coincide with pixel centers. Each window and pixmap
/// has its own coordinate system. For a window, the origin is inside the
/// border at the inside, upper-left corner.
///
/// The border_width for an InputOnly window must be zero, or a BadMatch error
/// results. For class InputOutput, the visual type and depth must be a
/// combination supported for the screen, or a BadMatch error results. The
/// depth need not be the same as the parent, but the parent must not be a
/// window of class InputOnly, or a BadMatch error results. For an InputOnly
/// window, the depth must be zero, and the visual must be one supported by the
/// screen. If either condition is not met, a BadMatch error results. The
/// parent window, however, may have any depth and class. If you specify any
/// invalid window attribute for a window, a BadMatch error results.
///
/// The created window is not yet displayed (mapped) on the user's display. To
/// display the window, call XMapWindow. The new window initially uses the same
/// cursor as its parent. A new cursor can be defined for the new window by
/// calling XDefineCursor. The window will not be visible on the screen unless
/// it and all of its ancestors are mapped and it is not obscured by any of its
/// ancestors.
///
/// XCreateWindow can generate BadAlloc BadColor, BadCursor, BadMatch,
/// BadPixmap, BadValue, and BadWindow errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XCreateSimpleWindow.3.xhtml
pub inline fn XCreateSimpleWindow(
    display: *Display,
    parent: Window,
    rect: Rect,
    /// Specifies the width of the created window's border in pixels.
    border_width: c_uint,
    /// Specifies the border pixel value of the window.
    border: c_ulong,
    /// Specifies the background pixel value of the window.
    background: c_ulong,
) Window {
    return X.XCreateSimpleWindow(
        display,
        parent,
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        border_width,
        border,
        background,
    );
}

/// The XCreateWindow function creates an unmapped subwindow for a specified
/// parent window, returns the window ID of the created window, and causes the
/// X server to generate a CreateNotify event. The created window is placed on
/// top in the stacking order with respect to siblings.
///
/// The coordinate system has the X axis horizontal and the Y axis vertical
/// with the origin [0,0] at the upper-left corner. Coordinates are integral,
/// in terms of pixels, and coincide with pixel centers. Each window and pixmap
/// has its own coordinate system. For a window, the origin is inside the
/// border at the inside, upper-left corner.
///
/// If you specify any invalid window attribute for a window, a BadMatch error
/// results.
///
/// The created window is not yet displayed (mapped) on the user's display. To
/// display the window, call XMapWindow. The new window initially uses the same
/// cursor as its parent. A new cursor can be defined for the new window by
/// calling XDefineCursor. The window will not be visible on the screen unless
/// it and all of its ancestors are mapped and it is not obscured by any of its
/// ancestors.
///
/// XCreateWindow can generate BadAlloc BadColor, BadCursor, BadMatch,
/// BadPixmap, BadValue, and BadWindow errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XCreateWindow.3.xhtml
pub inline fn XCreateWindow(
    display: *Display,
    parent: Window,
    x: c_int,
    y: c_int,
    width: c_uint,
    height: c_uint,
    border_width: c_uint,
    depth: c_int,
    /// Specifies the created window's class. You can pass InputOutput,
    /// InputOnly, or CopyFromParent. A class of CopyFromParent means the class
    /// is taken from the parent.
    class: c_uint,
    visual: [*c]Visual,
    valuemask: c_ulong,
    attributes: [*c]XSetWindowAttributes,
) Window {
    return X.XCreateWindow(
        display,
        parent,
        x,
        y,
        width,
        height,
        border_width,
        depth,
        class,
        visual,
        valuemask,
        attributes,
    );
}

/// If a cursor is set, it will be used when the pointer is in the window. If
/// the cursor is None, it is equivalent to XUndefineCursor.
///
/// XDefineCursor can generate BadCursor and BadWindow errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XDefineCursor.3.xhtml
pub inline fn XDefineCursor(display: *Display, window: Window, cursor: Cursor) void {
    // The meaning of the return value was not specified in documentation.
    _ = X.XDefineCursor(display, window, cursor);
}

/// The XDeleteProperty function deletes the specified property only if the
/// property was defined on the specified window and causes the X server to
/// generate a PropertyNotify event on the window unless the property does not
/// exist.
///
/// XDeleteProperty can generate BadAtom and BadWindow errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XGetWindowProperty.3.xhtml
pub inline fn XDeleteProperty(display: *Display, window: Window, atom: Atom) void {
    // The meaning of the return value was not specified in documentation.
    _ = X.XDeleteProperty(display, window, atom);
}

/// The XDestroyWindow function destroys the specified window as well as all of
/// its subwindows and causes the X server to generate a DestroyNotify event
/// for each window. The window should never be referenced again. If the window
/// specified by the w argument is mapped, it is unmapped automatically. The
/// ordering of the DestroyNotify events is such that for any given window
/// being destroyed, DestroyNotify is generated on any inferiors of the window
/// before being generated on the window itself. The ordering among siblings
/// and across subhierarchies is not otherwise constrained. If the window you
/// specified is a root window, no windows are destroyed. Destroying a mapped
/// window will generate Expose events on other windows that were obscured by
/// the window being destroyed.
///
/// XDestroyWindow can generate a BadWindow error.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XDestroyWindow.3.xhtml
pub inline fn XDestroyWindow(display: *Display, window: Window) void {
    // The meaning of the return value was not specified in documentation.
    _ = X.XDestroyWindow(display, window);
}

/// The XDisplayKeycodes function returns the min-keycodes and max-keycodes
/// supported by the specified display. The minimum number of KeyCodes returned
/// is never less than 8, and the maximum number of KeyCodes returned is never
/// greater than 255. Not all KeyCodes in this range are required to have
/// corresponding keys.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XChangeKeyboardMapping.3.xhtml
pub inline fn XDisplayKeycodes(
    display: *Display,
    min_keycodes_return: *c_int,
    max_keycodes_return: *c_int,
) void {
    // The meaning of the return value was not specified in documentation.
    _ = X.XDisplayKeycodes(display, min_keycodes_return, max_keycodes_return);
}

/// The XDrawRectangle and XDrawRectangles functions draw the outlines of the
/// specified rectangle or rectangles as if a five-point PolyLine protocol
/// request were specified for each rectangle:
///
/// [x,y] [x+width,y] [x+width,y+height] [x,y+height] [x,y]
///
/// For the specified rectangle or rectangles, these functions do not draw a
/// pixel more than once. XDrawRectangles draws the rectangles in the order
/// listed in the array. If rectangles intersect, the intersecting pixels are
/// drawn multiple times.
///
/// Both functions use these GC components: function, plane-mask, line-width,
/// line-style, cap-style, join-style, fill-style, subwindow-mode,
/// clip-x-origin, clip-y-origin, and clip-mask. They also use these GC
/// mode-dependent components: foreground, background, tile, stipple,
/// tile-stipple-x-origin, tile-stipple-y-origin, dash-offset, and dash-list.
///
/// XDrawRectangle and XDrawRectangles can generate BadDrawable, BadGC, and
/// BadMatch errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XDrawRectangle.3.xhtml
pub inline fn XDrawRectangle(
    display: *Display,
    drawable: Drawable,
    gc: GC,
    rect: Rect,
) void {
    // The meaning of the return value was not specified in documentation.
    _ = X.XDrawRectangle(display, drawable, gc, rect.x, rect.y, rect.w, rect.h);
}

/// The XFillRectangle and XFillRectangles functions fill the specified
/// rectangle or rectangles as if a four-point FillPolygon protocol request
/// were specified for each rectangle:
///
/// [x,y] [x+width,y] [x+width,y+height] [x,y+height]
///
/// Each function uses the x and y coordinates, width and height dimensions,
/// and GC you specify.
///
/// XFillRectangles fills the rectangles in the order listed in the array. For
/// any given rectangle, XFillRectangle and XFillRectangles do not draw a pixel
/// more than once. If rectangles intersect, the intersecting pixels are drawn
/// multiple times.
///
/// Both functions use these GC components: function, plane-mask, fill-style,
/// subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask. They also use
/// these GC mode-dependent components: foreground, background, tile, stipple,
/// tile-stipple-x-origin, and tile-stipple-y-origin.
///
/// XFillRectangle and XFillRectangles can generate BadDrawable, BadGC, and
/// BadMatch errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XFillRectangle.3.xhtml
pub inline fn XFillRectangle(
    display: *Display,
    drawable: Drawable,
    gc: GC,
    rect: Rect,
) void {
    // The meaning of the return value was not specified in documentation.
    _ = X.XFillRectangle(display, drawable, gc, rect.x, rect.y, rect.w, rect.h);
}

/// The XFree function is a general-purpose Xlib routine that frees the
/// specified data. You must use it to free any objects that were allocated by
/// Xlib, unless an alternate function is explicitly specified for the object.
/// A NULL pointer cannot be passed to this function.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XFree.3.xhtml
pub inline fn XFree(ptr: ?*anyopaque) void {
    // The meaning of the return value was not specified in documentation.
    _ = X.XFree(ptr);
}

/// The XFreeCursor function deletes the association between the cursor
/// resource ID and the specified cursor. The cursor storage is freed when no
/// other resource references it. The specified cursor ID should not be
/// referred to again.
///
/// XFreeCursor can generate a BadCursor error.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XRecolorCursor.3.xhtml
pub inline fn XFreeCursor(display: *Display, cursor: Cursor) void {
    // The meaning of the return value was not specified in documentation.
    _ = X.XFreeCursor(display, cursor);
}

/// The XFreeGC function destroys the specified GC as well as all the
/// associated storage.
///
/// XFreeGC can generate a BadGC error.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XCreateGC.3.xhtml
pub inline fn XFreeGC(display: *Display, gc: GC) void {
    // The meaning of the return value was not specified in documentation.
    _ = X.XFreeGC(display, gc);
}

/// The XFreeModifiermap function frees the specified XModifierKeymap structure.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XChangeKeyboardMapping.3.xhtml
pub inline fn XFreeModifiermap(modmap: *XModifierKeymap) void {
    // The meaning of the return value was not specified in documentation.
    _ = X.XFreeModifiermap(modmap);
}

/// The XFreePixmap function first deletes the association between the pixmap
/// ID and the pixmap. Then, the X server frees the pixmap storage when there
/// are no references to it. The pixmap should never be referenced again.
///
/// XFreePixmap can generate a BadPixmap error.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XCreatePixmap.3.xhtml
pub inline fn XFreePixmap(display: *Display, pixmap: Pixmap) void {
    // The meaning of the return value was not specified in documentation.
    _ = X.XFreePixmap(display, pixmap);
}

/// The XFreeStringList function releases memory allocated by
/// XmbTextPropertyToTextList, Xutf8TextPropertyToTextList and
/// XTextPropertyToStringList and the missing charset list allocated by
/// XCreateFontSet.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XStringListToTextProperty.3.xhtml
pub inline fn XFreeStringList(list: [*c][*c]u8) void {
    X.XFreeStringList(list);
}

/// The XGetClassHint function returns the class hint of the specified window
/// to the members of the supplied structure. If the data returned by the
/// server is in the Latin Portable Character Encoding, then the returned
/// strings are in the Host Portable Character Encoding. Otherwise, the result
/// is implementation-dependent. It returns a nonzero status on success;
/// otherwise, it returns a zero status. To free res_name and res_class when
/// finished with the strings, use XFree on each individually.
///
/// XGetClassHint can generate a BadWindow error.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XAllocClassHint.3.xhtml
pub inline fn XGetClassHint(display: *Display, window: Window) ?XClassHint {
    var class_hints_return: XClassHint = undefined;
    const status = X.XGetClassHint(display, window, &class_hints_return);
    if (status == 0) return null;
    return class_hints_return;
}

/// The XGetKeyboardMapping function returns the symbols for the specified
/// number of KeyCodes starting with first_keycode. The value specified in
/// first_keycode must be greater than or equal to min_keycode as returned by
/// XDisplayKeycodes, or a BadValue error results. In addition, the following
/// expression must be less than or equal to max_keycode as returned by
/// XDisplayKeycodes:
///
/// first_keycode + keycode_count − 1
///
/// If this is not the case, a BadValue error results. The number of elements
/// in the KeySyms list is:
///
/// keycode_count * keysyms_per_keycode_return
///
/// KeySym number N, counting from zero, for KeyCode K has the following index
/// in the list, counting from zero: (K − first_code) * keysyms_per_code_return
/// + N
///
/// The X server arbitrarily chooses the keysyms_per_keycode_return value to be
/// large enough to report all requested symbols. A special KeySym value of
/// NoSymbol is used to fill in unused elements for individual KeyCodes. To
/// free the storage returned by XGetKeyboardMapping, use XFree.
///
/// XGetKeyboardMapping can generate a BadValue error.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XChangeKeyboardMapping.3.xhtml
pub inline fn XGetKeyboardMapping(
    display: *Display,
    first_keycode: KeyCode,
    keycode_count: c_int,
    keysyms_per_keycode_return: *c_int,
) ?[*]KeySym {
    // Meaning of return value is not specified in documentation.
    return X.XGetKeyboardMapping(
        display,
        first_keycode,
        keycode_count,
        keysyms_per_keycode_return,
    );
}

/// The XGetModifierMapping function returns a pointer to a newly created
/// XModifierKeymap structure that contains the keys being used as modifiers.
/// The structure should be freed after use by calling XFreeModifiermap. If
/// only zero values appear in the set for any modifier, that modifier is
/// disabled.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XChangeKeyboardMapping.3.xhtml
pub inline fn XGetModifierMapping(display: *Display) ?*XModifierKeymap {
    return X.XGetModifierMapping(display);
}

/// The XGetTextProperty function reads the specified property from the window
/// and stores the data in the returned XTextProperty structure. It stores the
/// data in the value field, the type of the data in the encoding field, the
/// format of the data in the format field, and the number of items of data in
/// the nitems field. An extra byte containing null (which is not included in
/// the nitems member) is stored at the end of the value field of
/// text_prop_return. The particular interpretation of the property's encoding
/// and data as text is left to the calling application. If the specified
/// property does not exist on the window, XGetTextProperty sets the value
/// field to NULL, the encoding field to None, the format field to zero, and
/// the nitems field to zero.
///
/// If it was able to read and store the data in the XTextProperty structure,
/// XGetTextProperty returns a nonzero status; otherwise, it returns a zero
/// status.
///
/// XGetTextProperty can generate BadAtom and BadWindow errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XSetTextProperty.3.xhtml
pub inline fn XGetTextProperty(
    display: *Display,
    window: Window,
    property: Atom,
) ?XTextProperty {
    var ret: XTextProperty = undefined;
    const status = X.XGetTextProperty(display, window, &ret, property);
    return if (status == 0) null else ret;
}

/// The XGetTransientForHint function returns the WM_TRANSIENT_FOR property for
/// the specified window. It returns a nonzero status on success; otherwise, it
/// returns a zero status.
///
/// XGetTransientForHint can generate a BadWindow error.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XSetTransientForHint.3.xhtml
pub inline fn XGetTransientForHint(display: *Display, window: Window) ?Window {
    var prop_window_return: Window = X.None;
    if (X.XGetTransientForHint(display, window, &prop_window_return) == 0) return null;
    return prop_window_return;
}

/// The XGetWindowAttributes function returns the current attributes for the
/// specified window to an XWindowAttributes structure. It returns true upon
/// success.
///
/// XGetWindowAttributes can generate BadDrawable and BadWindow errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XGetWindowAttributes.3.xhtml
pub inline fn XGetWindowAttributes(
    display: *Display,
    window: Window,
    window_attributes_return: *XWindowAttributes,
) bool {
    // It returns a nonzero status on success; otherwise, it returns a zero status.
    return X.XGetWindowAttributes(display, window, window_attributes_return) != 0;
}

pub const YGetWindowPropertyResult = struct {
    const Self = @This();

    /// The atom identifier that defines the actual type of the property.
    type: Atom,
    /// The number of bytes remaining to be read in the property if a partial
    /// read was performed.
    bytes_after: c_ulong,
    /// Returns the data in the specified format. If the returned format is 8,
    /// the returned data is represented as a char array. If the returned
    /// format is 16, the returned data is represented as a array of short int
    /// type and should be cast to that type to obtain the elements. If the
    /// returned format is 32, the property data will be stored as an array of
    /// longs (which in a 64-bit application will be 64-bit values that are
    /// padded in the upper 4 bytes).
    value: FormattedData,

    pub inline fn deinit(self: *const Self) void {
        self.value.deinit();
    }
};

/// The XGetWindowProperty function returns the actual type of the property; the
/// actual format of the property; the number of 8-bit, 16-bit, or 32-bit items
/// transferred; the number of bytes remaining to be read in the property; and a
/// pointer to the data actually returned. XGetWindowProperty sets the return
/// arguments as follows:
///
/// 1) If the specified property does not exist for the specified window,
///    XGetWindowProperty returns None to actual_type_return and the value zero
///    to actual_format_return and bytes_after_return. The nitems_return
///    argument is empty. In this case, the delete argument is ignored.
///
/// 2) If the specified property exists but its type does not match the
///    specified type, XGetWindowProperty returns the actual property type to
///    actual_type_return, the actual property format (never zero) to
///    actual_format_return, and the property length in bytes (even if the
///    actual_format_return is 16 or 32) to bytes_after_return. It also ignores
///    the delete argument. The nitems_return argument is empty.
///
/// 3) If the specified property exists and either you assign AnyPropertyType to
///    the req_type argument or the specified type matches the actual property
///    type, XGetWindowProperty returns the actual property type to
///    actual_type_return and the actual property format (never zero) to
///    actual_format_return. It also returns a value to bytes_after_return and
///    nitems_return, by defining the following values:
///     * N = actual length of the stored property in bytes (even if the format is 16 or 32)
///     * I = 4 * long_offset
///     * T = N - I
///     * L = MINIMUM(T, 4 * long_length)
///     * A = N - (I + L)
///    The returned value starts at byte index I in the property (indexing from
///    zero), and its length in bytes is L. If the value for long_offset causes L
///    to be negative, a BadValue error results. The value of bytes_after_return
///    is A, giving the number of trailing unread bytes in the stored property.
///
/// If the returned format is 8, the returned data is represented as a char
/// array. If the returned format is 16, the returned data is represented as a
/// short array and should be cast to that type to obtain the elements. If the
/// returned format is 32, the returned data is represented as a long array and
/// should be cast to that type to obtain the elements.
///
/// XGetWindowProperty always allocates one extra byte in prop_return (even if
/// the property is zero length) and sets it to zero so that simple properties
/// consisting of characters do not have to be copied into yet another string
/// before use.
///
/// If delete is True and bytes_after_return is zero, XGetWindowProperty deletes
/// the property from the window and generates a PropertyNotify event on the
/// window.
///
/// The function returns true if it executes successfully. To free the resulting
/// data, use XFree.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XGetWindowProperty.3.xhtml
pub inline fn XGetWindowProperty(
    display: *Display,
    /// The window whose property you want to obtain.
    w: Window,
    property: Atom,
    /// The offset in the specified property (in 32-bit quantities) where the
    /// data is to be retrieved.
    long_offset: c_long,
    /// The length in 32-bit multiples of the data to be retrieved.
    long_length: c_long,
    /// Determines whether the property is deleted.
    delete: bool,
    /// The atom identifier associated with the property type or
    /// AnyPropertyType.
    req_type: Atom,
) ?YGetWindowPropertyResult {
    var r_type: c_ulong = 0;
    // The number of 8-bit, 16-bit, or 32-bit items stored in the data.
    var nitems: c_ulong = 0;
    var r_bytes_after: c_ulong = 0;
    var raw_data: [*c]u8 = undefined;
    var format: c_int = 0;
    const status = X.XGetWindowProperty(display, w, property, long_offset, //
        long_length, @intFromBool(delete), req_type, &r_type, &format, //
        &nitems, &r_bytes_after, &raw_data);

    // From the original docs:
    // "The function returns Success if it executes successfully."
    if (status != X.Success) return null;

    // If the specified property does not exist for the specified window,
    // XGetWindowProperty returns None to actual_type_return and the value zero
    // to actual_format_return and bytes_after_return. The nitems_return
    // argument is empty. In this case, the delete argument is ignored.
    if (r_type == X.None or format == 0 or r_bytes_after == 0) return null;

    return .{
        .type = r_type,
        .bytes_after = r_bytes_after,
        .value = blk: {
            const n: usize = @intCast(nitems);
            switch (format) {
                8 => {
                    break :blk .{ .Fmt8 = raw_data[0..n] };
                },
                16 => {
                    const data16: [*c]u16 = @ptrCast(@alignCast(raw_data));
                    break :blk .{ .Fmt16 = data16[0..n] };
                },
                32 => {
                    const data32: [*c]u32 = @ptrCast(@alignCast(raw_data));
                    break :blk .{ .Fmt32 = data32[0..n] };
                },
                else => {
                    log.err("Format value: {d}", .{format});
                    unreachable;
                },
            }
        },
    };
}

/// The XGetWMHints function reads the window manager hints and returns NULL if
/// no WM_HINTS property was set on the window or returns a pointer to a
/// XWMHints structure if it succeeds. When finished with the data, free the
/// space used for it by calling XFree.
///
/// XGetWMHints can generate a BadWindow error.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XAllocWMHints.3.xhtml
pub inline fn XGetWMHints(display: *Display, window: Window) ?*XWMHints {
    return X.XGetWMHints(display, window);
}

/// The XGetWMNormalHints function returns the size hints stored in the
/// WM_NORMAL_HINTS property on the specified window. If the property is of
/// type WM_SIZE_HINTS, is of format 32, and is long enough to contain either
/// an old (pre-ICCCM) or new size hints structure, XGetWMNormalHints sets the
/// various fields of the XSizeHints structure, sets the supplied_return
/// argument to the list of fields that were supplied by the user (whether or
/// not they contained defined values), and returns a nonzero status.
/// Otherwise, it returns a zero status.
///
/// If XGetWMNormalHints returns successfully and a pre-ICCCM size hints
/// property is read, the supplied_return argument will contain the following
/// bits:
///
/// (USPosition|USSize|PPosition|PSize|PMinSize|PMaxSize|PResizeInc|PAspect)
///
/// If the property is large enough to contain the base size and window gravity
/// fields as well, the supplied_return argument will also contain the
/// following bits:
///
/// PBaseSize|PWinGravity
///
/// XGetWMNormalHints can generate a PN BadWindow error.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XAllocSizeHints.3.xhtml
pub inline fn XGetWMNormalHints(display: *Display, window: Window) ?XSizeHints {
    var hints_return: XSizeHints = undefined;
    var supplied_return: c_long = undefined;
    const status = X.XGetWMNormalHints(display, window, &hints_return, &supplied_return);
    if (status == 0) return null;
    return hints_return;
}

/// The XGetWMProtocols function returns the list of atoms stored in the
/// WM_PROTOCOLS property on the specified window. These atoms describe window
/// manager protocols in which the owner of this window is willing to
/// participate. If the property exists, is of type ATOM, is of format 32, and
/// the atom WM_PROTOCOLS can be interned, XGetWMProtocols sets the
/// protocols_return argument to a list of atoms, sets the count_return
/// argument to the number of elements in the list, and returns a nonzero
/// status. Otherwise, it sets neither of the return arguments and returns a
/// zero status. To release the list of atoms, use XFree.
///
/// XGetWMProtocols can generate a BadWindow error.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XSetWMProtocols.3.xhtml
pub inline fn XGetWMProtocols(display: *Display, window: Window) ?[]Atom {
    var atoms: ?[*]Atom = undefined;
    var count_return: c_int = undefined;
    // If the property exists, is of type ATOM, is of format 32, and the atom
    // WM_PROTOCOLS can be interned, XGetWMProtocols sets the protocols_return
    // argument to a list of atoms, sets the count_return argument to the
    // number of elements in the list, and returns a nonzero status. Otherwise,
    // it sets neither of the return arguments and returns a zero status. To
    // release the list of atoms, use XFree.
    const status = X.XGetWMProtocols(display, window, &atoms, &count_return);
    if (status == 0) return null;
    return atoms.?[0..@intCast(count_return)];
}

/// The XGrabButton function establishes a passive grab. In the future, the
/// pointer is actively grabbed (as for XGrabPointer), the last-pointer-grab
/// time is set to the time at which the button was pressed (as transmitted in
/// the ButtonPress event), and the ButtonPress event is reported if all of the
/// following conditions are true:
///
/// 1. The pointer is not grabbed, and the specified button is logically pressed
///    when the specified modifier keys are logically down, and no other buttons
///    or modifier keys are logically down.
/// 2. The grab_window contains the pointer.
/// 3. The confine_to window (if any) is viewable.
/// 4. A passive grab on the same button/key combination does not exist on any
///    ancestor of grab_window.
///
/// The interpretation of the remaining arguments is as for XGrabPointer. The
/// active grab is terminated automatically when the logical state of the
/// pointer has all buttons released (independent of the state of the logical
/// modifier keys), at which point a ButtonRelease event is reported to the
/// grabbing window.
///
/// Note that the logical state of a device (as seen by client applications)
/// may lag the physical state if device event processing is frozen.
///
/// This request overrides all previous grabs by the same client on the same
/// button/key combinations on the same window. A modifiers of AnyModifier is
/// equivalent to issuing the grab request for all possible modifier
/// combinations (including the combination of no modifiers). It is not
/// required that all modifiers specified have currently assigned KeyCodes. A
/// button of AnyButton is equivalent to issuing the request for all possible
/// buttons. Otherwise, it is not required that the specified button currently
/// be assigned to a physical button.
///
/// If some other client has already issued a XGrabButton with the same
/// button/key combination on the same window, a BadAccess error results. When
/// using AnyModifier or AnyButton, the request fails completely, and a
/// BadAccess error results (no grabs are established) if there is a
/// conflicting grab for any combination. XGrabButton has no effect on an
/// active grab.
///
/// XGrabButton can generate BadCursor, BadValue, and BadWindow errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XGrabButton.3.xhtml
pub inline fn XGrabButton(
    display: *Display,
    /// Specifies the pointer button that is to be grabbed or released or AnyButton.
    button: c_uint,
    modifiers: c_uint,
    grab_window: Window,
    owner_events: bool,
    event_mask: c_uint,
    pointer_mode: GrabMode,
    keyboard_mode: GrabMode,
    confine_to: Window,
    cursor: Cursor,
) void {
    // Inferrably, since XGrabButton is very similar to XGrabPointer, the
    // returned integer could mean X.GrabSuccess. However, since it's not
    // explicitly stated in the documentation, we shall discard it.
    _ = X.XGrabButton(
        display,
        button,
        modifiers,
        grab_window,
        @intFromBool(owner_events),
        event_mask,
        @intFromEnum(pointer_mode),
        @intFromEnum(keyboard_mode),
        confine_to,
        cursor,
    );
}

/// The XGrabKey function establishes a passive grab on the keyboard. In the
/// future, the keyboard is actively grabbed (as for XGrabKeyboard), the
/// last-keyboard-grab time is set to the time at which the key was pressed (as
/// transmitted in the KeyPress event), and the KeyPress event is reported if
/// all of the following conditions are true:
///
/// * The keyboard is not grabbed and the specified key (which can itself be a
///   modifier key) is logically pressed when the specified modifier keys are
///   logically down, and no other modifier keys are logically down.
///
/// * Either the grab_window is an ancestor of (or is) the focus window, or the
///   grab_window is a descendant of the focus window and contains the pointer.
///
/// * A passive grab on the same key combination does not exist on any ancestor of
///   grab_window.
///
/// The interpretation of the remaining arguments is as for XGrabKeyboard. The
/// active grab is terminated automatically when the logical state of the
/// keyboard has the specified key released (independent of the logical state o
/// the modifier keys), at which point a KeyRelease event is reported to the
/// grabbing window.
///
/// Note that the logical state of a device (as seen by client applications)
/// may lag the physical state if device event processing is frozen.
///
/// A modifiers argument of AnyModifier is equivalent to issuing the request
/// for all possible modifier combinations (including the combination of no
/// modifiers). It is not required that all modifiers specified have currently
/// assigned KeyCodes. A keycode argument of AnyKey is equivalent to issuing
/// the request for all possible KeyCodes. Otherwise, the specified keycode
/// must be in the range specified by min_keycode and max_keycode in the
/// connection setup, or a BadValue error results.
///
/// If some other client has issued a XGrabKey with the same key combination on
/// the same window, a BadAccess error results. When using AnyModifier or
/// AnyKey, the request fails completely, and a BadAccess error results (no
/// grabs are established) if there is a conflicting grab for any combination.
///
/// XGrabKey can generate BadAccess, BadValue, and BadWindow errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XGrabKey.3.xhtml
pub inline fn XGrabKey(
    display: *Display,
    /// Specifies the KeyCode or AnyKey.
    keycode: c_int,
    modifiers: c_uint,
    grab_window: Window,
    owner_events: bool,
    pointer_mode: GrabMode,
    keyboard_mode: GrabMode,
) void {
    // Meaning of return value is not specified in documentation.
    _ = X.XGrabKey(
        display,
        keycode,
        modifiers,
        grab_window,
        @intFromBool(owner_events),
        @intFromEnum(pointer_mode),
        @intFromEnum(keyboard_mode),
    );
}

/// The XGrabPointer function actively grabs control of the pointer and returns
/// true if the grab was successful. Further pointer events are reported only
/// to the grabbing client. XGrabPointer overrides any active pointer grab by
/// this client. If owner_events is False, all generated pointer events are
/// reported with respect to grab_window and are reported only if selected by
/// event_mask. If owner_events is True and if a generated pointer event would
/// normally be reported to this client, it is reported as usual. Otherwise,
/// the event is reported with respect to the grab_window and is reported only
/// if selected by event_mask. For either value of owner_events, unreported
/// events are discarded.
///
/// XGrabPointer generates EnterNotify and LeaveNotify events.
///
/// Either if grab_window or confine_to window is not viewable or if the
/// confine_to window lies completely outside the boundaries of the root
/// window, XGrabPointer fails and returns GrabNotViewable. If the pointer is
/// actively grabbed by some other client, it fails and returns AlreadyGrabbed.
/// If the pointer is frozen by an active grab of another client, it fails and
/// returns GrabFrozen. If the specified time is earlier than the
/// last-pointer-grab time or later than the current X server time, it fails
/// and returns GrabInvalidTime. Otherwise, the last-pointer-grab time is set
/// to the specified time (CurrentTime is replaced by the current X server
/// time).
///
/// XGrabPointer can generate BadCursor, BadValue, and BadWindow errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XGrabPointer.3.xhtml
pub inline fn XGrabPointer(
    display: *Display,
    grab_window: Window,
    owner_events: bool,
    event_mask: c_uint,
    /// If it's GrabModeAsync, pointer event processing continues as usual. If
    /// the pointer is currently frozen by this client, the processing of
    /// events for the pointer is resumed. If the pointer_mode is GrabModeSync,
    /// the state of the pointer, as seen by client applications, appears to
    /// freeze, and the X server generates no further pointer events until the
    /// grabbing client calls XAllowEvents or until the pointer grab is
    /// released. Actual pointer changes are not lost while the pointer is
    /// frozen; they are simply queued in the server for later processing.
    pointer_mode: GrabMode,
    /// If it's GrabModeAsync, keyboard event processing is unaffected by
    /// activation of the grab. If the keyboard_mode is GrabModeSync, the state
    /// of the keyboard, as seen by client applications, appears to freeze, and
    /// the X server generates no further keyboard events until the grabbing
    /// client calls XAllowEvents or until the pointer grab is released. Actual
    /// keyboard changes are not lost while the pointer is frozen; they are
    /// simply queued in the server for later processing.
    keyboard_mode: GrabMode,
    /// If a confine_to window is specified, the pointer is restricted to stay
    /// contained in that window. The confine_to window need have no
    /// relationship to the grab_window. If the pointer is not initially in the
    /// confine_to window, it is warped automatically to the closest edge just
    /// before the grab activates and enter/leave events are generated as
    /// usual. If the confine_to window is subsequently reconfigured, the
    /// pointer is warped automatically, as necessary, to keep it contained in
    /// the window.
    confine_to: Window,
    /// If a cursor is specified, it is displayed regardless of what window the
    /// pointer is in. If None is specified, the normal cursor for that window
    /// is displayed when the pointer is in grab_window or one of its
    /// subwindows; otherwise, the cursor for grab_window is displayed.
    cursor: Cursor,
    /// The time argument allows you to avoid certain circumstances that come
    /// up if applications take a long time to respond or if there are long
    /// network delays. Consider a situation where you have two applications,
    /// both of which normally grab the pointer when clicked on. If both
    /// applications specify the timestamp from the event, the second
    /// application may wake up faster and successfully grab the pointer before
    /// the first application. The first application then will get an
    /// indication that the other application grabbed the pointer before its
    /// request was processed.
    time: Time,
) bool {
    const result = X.XGrabPointer(
        display,
        grab_window,
        @intFromBool(owner_events),
        event_mask,
        @intFromEnum(pointer_mode),
        @intFromEnum(keyboard_mode),
        confine_to,
        cursor,
        time,
    );
    // From the docs:
    // "The XGrabPointer function actively grabs control of the pointer and
    // returns GrabSuccess if the grab was successful."
    return result == X.GrabSuccess;
}

/// The XGrabServer function disables processing of requests and close downs on
/// all other connections than the one this request arrived on. You should not
/// grab the X server any more than is absolutely necessary.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XGrabServer.3.xhtml
pub inline fn XGrabServer(display: *Display) void {
    // Meaning of return value is not specified in documentation.
    _ = X.XGrabServer(display);
}

/// The XInternAtom function returns the atom identifier associated with the
/// specified atom_name. If the atom name is not in the Host Portable Character
/// Encoding, the result is implementation-dependent. Uppercase and lowercase
/// matter. The atom will remain defined even after the client's connection
/// closes. It will become undefined only when the last connection to the X
/// server closes.
///
/// XInternAtom can generate BadAlloc and BadValue errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XInternAtom.3.xhtml
pub inline fn XInternAtom(
    display: *Display,
    atom_name: [*c]const u8,
    // If only_if_exists is False, the atom is created if it does not exist.
    only_if_exists: bool,
) ?Atom {
    const atom = X.XInternAtom(display, atom_name, @intFromBool(only_if_exists));
    // To quote from X11/X.h:
    // ```c
    // #ifndef None
    // #define None 0L /* universal null resource or null atom */
    // #endif
    // ```
    return if (atom == X.None) null else atom;
}

/// If the specified KeySym is not defined for any KeyCode, XKeysymToKeycode
/// returns zero.
///
/// The inverse function would be `XkbKeycodeToKeysym`.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XStringToKeysym.3.xhtml
pub inline fn XKeysymToKeycode(display: *Display, keysym: KeySym) KeyCode {
    return X.XKeysymToKeycode(display, keysym);
}

/// The XKeycodeToKeysym function uses internal Xlib tables and returns the
/// KeySym defined for the specified KeyCode and the element of the KeyCode
/// vector. If no symbol is defined, XKeycodeToKeysym returns NoSymbol.
/// XKeycodeToKeysym predates the XKB extension. If you want to lookup a KeySym
/// while using XKB you have to use XkbKeycodeToKeysym.
///
/// The inverse function would be `XKeysymToKeycode`.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XStringToKeysym.3.xhtml
pub inline fn XkbKeycodeToKeysym(
    display: *Display,
    keycode: KeyCode,
    group: c_uint,
    level: c_uint,
) KeySym {
    return X.XkbKeycodeToKeysym(display, keycode, group, level);
}

/// The XKillClient function forces a close down of the client that created the
/// resource if a valid resource is specified. If the client has already
/// terminated in either RetainPermanent or RetainTemporary mode, all of the
/// client's resources are destroyed. If AllTemporary is specified, the
/// resources of all clients that have terminated in RetainTemporary are
/// destroyed (see section 2.5). This permits implementation of window manager
/// facilities that aid debugging. A client can set its close-down mode to
/// RetainTemporary. If the client then crashes, its windows would not be
/// destroyed. The programmer can then inspect the application's window tree
/// and use the window manager to destroy the zombie windows.
///
/// XKillClient can generate a BadValue error.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XSetCloseDownMode.3.xhtml
pub inline fn XKillClient(display: *Display, resource: XID) void {
    // Meaning of return value is not specified in documentation.
    _ = X.XKillClient(display, resource);
}

/// The XMapWindow function maps the window and all of its subwindows that have
/// had map requests. Mapping a window that has an unmapped ancestor does not
/// display the window but marks it as eligible for display when the ancestor
/// becomes mapped. Such a window is called unviewable. When all its ancestors
/// are mapped, the window becomes viewable and will be visible on the screen
/// if it is not obscured by another window. This function has no effect if the
/// window is already mapped.
///
/// If the override-redirect of the window is False and if some other client
/// has selected SubstructureRedirectMask on the parent window, then the X
/// server generates a MapRequest event, and the XMapWindow function does not
/// map the window. Otherwise, the window is mapped, and the X server generates
/// a MapNotify event.
///
/// If the window becomes viewable and no earlier contents for it are
/// remembered, the X server tiles the window with its background. If the
/// window's background is undefined, the existing screen contents are not
/// altered, and the X server generates zero or more Expose events. If
/// backing-store was maintained while the window was unmapped, no Expose
/// events are generated. If backing-store will now be maintained, a
/// full-window exposure is always generated. Otherwise, only visible regions
/// may be reported. Similar tiling and exposure take place for any newly
/// viewable inferiors.
///
/// If the window is an InputOutput window, XMapWindow generates Expose events
/// on each InputOutput window that it causes to be displayed. If the client
/// maps and paints the window and if the client begins processing events, the
/// window is painted twice. To avoid this, first ask for Expose events and
/// then map the window, so the client processes input events as usual. The
/// event list will include Expose for each window that has appeared on the
/// screen. The client's normal response to an Expose event should be to
/// repaint the window. This method usually leads to simpler programs and to
/// proper interaction with window managers.
///
/// XMapWindow can generate a BadWindow error.
///
/// The XMapRaised function essentially is similar to XMapWindow in that it
/// maps the window and all of its subwindows that have had map requests.
/// However, it also raises the specified window to the top of the stack.
///
/// XMapRaised can generate a BadWindow error.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XMapWindow.3.xhtml
pub inline fn XMapRaised(display: *Display, window: Window) void {
    // Meaning of return value is not specified in documentation.
    _ = X.XMapRaised(display, window);
}

/// The XMapWindow function maps the window and all of its subwindows that have
/// had map requests. Mapping a window that has an unmapped ancestor does not
/// display the window but marks it as eligible for display when the ancestor
/// becomes mapped. Such a window is called unviewable. When all its ancestors
/// are mapped, the window becomes viewable and will be visible on the screen
/// if it is not obscured by another window. This function has no effect if the
/// window is already mapped.
///
/// If the override-redirect of the window is False and if some other client
/// has selected SubstructureRedirectMask on the parent window, then the X
/// server generates a MapRequest event, and the XMapWindow function does not
/// map the window. Otherwise, the window is mapped, and the X server generates
/// a MapNotify event.
///
/// If the window becomes viewable and no earlier contents for it are
/// remembered, the X server tiles the window with its background. If the
/// window's background is undefined, the existing screen contents are not
/// altered, and the X server generates zero or more Expose events. If
/// backing-store was maintained while the window was unmapped, no Expose
/// events are generated. If backing-store will now be maintained, a
/// full-window exposure is always generated. Otherwise, only visible regions
/// may be reported. Similar tiling and exposure take place for any newly
/// viewable inferiors.
///
/// If the window is an InputOutput window, XMapWindow generates Expose events
/// on each InputOutput window that it causes to be displayed. If the client
/// maps and paints the window and if the client begins processing events, the
/// window is painted twice. To avoid this, first ask for Expose events and
/// then map the window, so the client processes input events as usual. The
/// event list will include Expose for each window that has appeared on the
/// screen. The client's normal response to an Expose event should be to
/// repaint the window. This method usually leads to simpler programs and to
/// proper interaction with window managers.
///
/// XMapWindow can generate a BadWindow error.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XMapWindow.3.xhtml
pub inline fn XMapWindow(display: *Display, window: Window) void {
    // Meaning of return value is not specified in documentation.
    _ = X.XMapWindow(display, window);
}

/// The XMaskEvent function searches the event queue for the events associated
/// with the specified mask. When it finds a match, XMaskEvent removes that
/// event and copies it into the specified XEvent structure. The other events
/// stored in the queue are not discarded. If the event you requested is not in
/// the queue, XMaskEvent flushes the output buffer and blocks until one is
/// received.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XNextEvent.3.xhtml
pub inline fn XMaskEvent(display: *Display, window: Window, event_return: *XEvent) void {
    // Meaning of return value is not specified in documentation.
    _ = X.XMaskEvent(display, window, event_return);
}

/// The XMoveResizeWindow function changes the size and location of the
/// specified window without raising it. Moving and resizing a mapped window
/// may generate an Expose event on the window. Depending on the new size and
/// location parameters, moving and resizing a window may generate Expose
/// events on windows that the window formerly obscured.
///
/// If the override-redirect flag of the window is False and some other client
/// has selected SubstructureRedirectMask on the parent, the X server generates
/// a ConfigureRequest event, and no further processing is performed.
/// Otherwise, the window size and location are changed.
///
/// XMoveResizeWindow can generate BadValue and BadWindow errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XConfigureWindow.3.xhtml
pub inline fn XMoveResizeWindow(
    display: *Display,
    window: Window,
    x: c_int,
    y: c_int,
    width: c_uint,
    height: c_uint,
) void {
    // It is not specified in documentation what the return value of XMoveWindow
    // is, so we shall discard it.
    _ = X.XMoveResizeWindow(display, window, x, y, width, height);
}

/// The XMoveWindow function moves the specified window to the specified x and
/// y coordinates, but it does not change the window's size, raise the window,
/// or change the mapping state of the window. Moving a mapped window may or
/// may not lose the window's contents depending on if the window is obscured
/// by nonchildren and if no backing store exists. If the contents of the
/// window are lost, the X server generates Expose events. Moving a mapped
/// window generates Expose events on any formerly obscured windows.
///
/// If the override-redirect flag of the window is False and some other client
/// has selected SubstructureRedirectMask on the parent, the X server generates
/// a ConfigureRequest event, and no further processing is performed.
/// Otherwise, the window is moved.
///
/// XMoveWindow can generate a BadWindow error.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XConfigureWindow.3.xhtml
pub inline fn XMoveWindow(
    display: *Display,
    window: Window,
    x: c_int,
    y: c_int,
) void {
    // The x and y coordinates are the new location of the top-left pixel of
    // the window's border or the window itself if it has no border or define
    // the new position of the window relative to its parent.

    // It is not specified in documentation what the return value of XMoveWindow
    // is, so we shall discard it.
    _ = X.XMoveWindow(display, window, x, y);
}

/// The XNextEvent function copies the first event from the event queue into
/// the specified XEvent structure and then removes it from the queue. If the
/// event queue is empty, XNextEvent flushes the output buffer and blocks until
/// an event is received.
///
/// Returns true upon success.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XNextEvent.3.xhtml
pub inline fn XNextEvent(display: *Display, event: *XEvent) bool {
    return X.XNextEvent(display, event) == X.Success;
}

/// The XOpenDisplay function returns a Display structure that serves as the
/// connection to the X server and that contains all the information about that
/// X server. XOpenDisplay connects your application to the X server through
/// TCP or DECnet communications protocols, or through some local inter-process
/// communication protocol. If the hostname is a host machine name and a single
/// colon (:) separates the hostname and display number, XOpenDisplay connects
/// using TCP streams. If the hostname is not specified, Xlib uses whatever it
/// believes is the fastest transport. If the hostname is a host machine name
/// and a double colon (::) separates the hostname and display number,
/// XOpenDisplay connects using DECnet. A single X server can support any or
/// all of these transport mechanisms simultaneously. A particular Xlib
/// implementation can support many more of these transport mechanisms.
///
/// If successful, XOpenDisplay returns a pointer to a Display structure, which
/// is defined in <X11/Xlib.h>. If XOpenDisplay does not succeed, it returns
/// NULL. After a successful call to XOpenDisplay, all of the screens in the
/// display can be used by the client. The screen number specified in the
/// display_name argument is returned by the DefaultScreen macro (or the
/// XDefaultScreen function). You can access elements of the Display and Screen
/// structures only by using the information macros or functions. For
/// information about using macros and functions to obtain information from the
/// Display structure, see section 2.2.1.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XOpenDisplay.3.xhtml
pub inline fn XOpenDisplay(display_name: [*c]const u8) ?*Display {
    return X.XOpenDisplay(display_name);
}

/// Custom struct for dealing with XQueryPointer.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XQueryPointer.3.xhtml
pub const YQueryPointerResult = struct {
    /// The root window the pointer is logically on.
    root_window: Window,
    /// The coordinates of the pointer relative to the root window's origin.
    root_pos: Coordinates(c_int),
    /// This is non-null if and only if the pointer is on the same screen as the
    /// specified window. It is the coordinates of the cursor relative to the
    /// origin of the specified window.
    win_pos: ?Coordinates(c_int),
    /// Returns the child window that the pointer is located in, if any.
    child: ?Window,
    /// The current logical state of the keyboard buttons and the modifier
    /// keys. That is, the bitwise inclusive OR of one or more of the button or
    /// modifier key bitmasks to match the current state of the mouse buttons
    /// and the modifier keys.
    mask: c_uint,
};

/// The XQueryPointer function returns the root window the pointer is logically
/// on and the pointer coordinates relative to the root window's origin. If
/// XQueryPointer returns False, the pointer is not on the same screen as the
/// specified window, and XQueryPointer returns None to child_return and zero
/// to win_x_return and win_y_return. If XQueryPointer returns True, the
/// pointer coordinates returned to win_x_return and win_y_return are relative
/// to the origin of the specified window. In this case, XQueryPointer returns
/// the child that contains the pointer, if any, or else None to child_return.
///
/// XQueryPointer returns the current logical state of the keyboard buttons and
/// the modifier keys in mask_return. It sets mask_return to the bitwise
/// inclusive OR of one or more of the button or modifier key bitmasks to match
/// the current state of the mouse buttons and the modifier keys.
///
/// XQueryPointer can generate a BadWindow error.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XQueryPointer.3.xhtml
pub inline fn XQueryPointer(
    display: *Display,
    window: Window,
) YQueryPointerResult {
    var r: YQueryPointerResult = .{
        .root_window = undefined,
        .root_pos = .zero,
        .win_pos = .zero,
        .child = 0,
        .mask = undefined,
    };
    // Bool XQueryPointer(Display *display, Window w,
    //                    Window *root_return,
    //                    Window *child_return,
    //                    int *root_x_return,
    //                    int *root_y_return,
    //                    int *win_x_return,
    //                    int *win_y_return,
    //                    unsigned int *mask_return);
    const result = X.XQueryPointer(display, window, &r.root_window, &r.child.?, //
        &r.root_pos.x, &r.root_pos.y, &r.win_pos.?.x, &r.win_pos.?.y, &r.mask);
    if (result == 0) {
        // If XQueryPointer returns False, the pointer is not on the same
        // screen as the specified window, and XQueryPointer returns None to
        // child_return and zero to win_x_return and win_y_return.
        r.child = null;
        r.win_pos = null;
    } else {
        // If XQueryPointer returns True, the pointer coordinates returned to
        // win_x_return and win_y_return are relative to the origin of the
        // specified window. In this case, XQueryPointer returns the child that
        // contains the pointer, if any, or else None to child_return.
        if (r.child == X.None) r.child = null;
    }
    return r;
}

/// The XQueryTree function returns the root ID, the parent window ID, a
/// pointer to the list of children windows (NULL when there are no children),
/// and the number of children in the list for the specified window. The
/// children are listed in current stacking order, from bottom-most (first) to
/// top-most (last). XQueryTree returns zero if it fails and nonzero if it
/// succeeds. To free a non-NULL children list when it is no longer needed, use
/// XFree.
///
/// XQueryTree can generate a BadWindow error.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XQueryTree.3.xhtml
pub inline fn XQueryTree(
    display: *Display,
    window: Window,
    root_return: *Window,
    parent_return: *Window,
) ?[]Window {
    var c_opt: ?[*]Window = undefined;
    var n: c_uint = undefined;
    const status = X.XQueryTree(display, window, root_return, parent_return, &c_opt, &n);
    if (status == 0) return null;
    var children: []Window = undefined;
    children.ptr = c_opt orelse return null;
    children.len = @intCast(n);
    return children;
}

/// The XRaiseWindow function raises the specified window to the top of the
/// stack so that no sibling window obscures it. If the windows are regarded as
/// overlapping sheets of paper stacked on a desk, then raising a window is
/// analogous to moving the sheet to the top of the stack but leaving its x and
/// y location on the desk constant. Raising a mapped window may generate
/// Expose events for the window and any mapped subwindows that were formerly
/// obscured.
///
/// If the override-redirect attribute of the window is False and some other
/// client has selected SubstructureRedirectMask on the parent, the X server
/// generates a ConfigureRequest event, and no processing is performed.
/// Otherwise, the window is raised.
///
/// XRaiseWindow can generate a BadWindow error.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XRaiseWindow.3.xhtml
pub inline fn XRaiseWindow(display: *Display, window: Window) void {
    // Meaning of return value is not specified in documentation.
    _ = X.XRaiseWindow(display, window);
}

/// The XRefreshKeyboardMapping function refreshes the stored modifier and
/// keymap information. You usually call this function when a MappingNotify
/// event with a request member of MappingKeyboard or MappingModifier occurs.
/// The result is to update Xlib's knowledge of the keyboard.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XLookupKeysym.3.xhtml
pub inline fn XRefreshKeyboardMapping(ev: *XMappingEvent) void {
    // Meaning of return value is not specified in documentation.
    _ = X.XRefreshKeyboardMapping(ev);
}

/// The XSelectInput function requests that the X server report the events
/// associated with the specified event mask. Initially, X will not report any
/// of these events. Events are reported relative to a window. If a window is
/// not interested in a device event, it usually propagates to the closest
/// ancestor that is interested, unless the do_not_propagate mask prohibits it.
///
/// Setting the event-mask attribute of a window overrides any previous call
/// for the same window but not for other clients. Multiple clients can select
/// for the same events on the same window with the following restrictions:
///
/// * Multiple clients can select events on the same window because their event
///   masks are disjoint. When the X server generates an event, it reports it to all
///   interested clients.
///
/// * Only one client at a time can select CirculateRequest, ConfigureRequest, or
///   MapRequest events, which are associated with the event mask
///   SubstructureRedirectMask.
///
/// * Only one client at a time can select a ResizeRequest event, which is
///   associated with the event mask ResizeRedirectMask.
///
/// * Only one client at a time can select a ButtonPress event, which is associated
///   with the event mask ButtonPressMask.
///
/// The server reports the event to all interested clients.
///
/// XSelectInput can generate a BadWindow error.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XSendEvent.3.xhtml
pub inline fn XSelectInput(display: *Display, window: Window, event_mask: c_long) void {
    // Meaning of return value is not specified in documentation.
    _ = X.XSelectInput(display, window, event_mask);
}

/// The XSendEvent function identifies the destination window, determines which
/// clients should receive the specified events, and ignores any active grabs.
/// This function requires you to pass an event mask. For a discussion of the
/// valid event mask names, see section 10.3. This function uses the w argument
/// to identify the destination window as follows:
///
///
/// * If w is PointerWindow, the destination window is the window that contains
///   the pointer.
/// * If w is InputFocus and if the focus window contains the pointer, the
///   destination window is the window that contains the pointer; otherwise, the
///   destination window is the focus window.
///
/// To determine which clients should receive the specified events, XSendEvent uses
/// the propagate argument as follows:
///
/// * If event_mask is the empty set, the event is sent to the client that created
///   the destination window. If that client no longer exists, no event is sent.
/// * If propagate is False, the event is sent to every client selecting on
///   destination any of the event types in the event_mask argument.
/// * If propagate is True and no clients have selected on destination any of
///   the event types in event-mask, the destination is replaced with the
///   closest ancestor of destination for which some client has selected a type
///   in event-mask and for which no intervening window has that type in its
///   do-not-propagate-mask. If no such window exists or if the window is an
///   ancestor of the focus window and InputFocus was originally specified as
///   the destination, the event is not sent to any clients. Otherwise, the
///   event is reported to every client selecting on the final destination any
///   of the types specified in event_mask.
///
/// The event in the XEvent structure must be one of the core events or one of the
/// events defined by an extension (or a BadValue error results) so that the X
/// server can correctly byte-swap the contents as necessary. The contents of the
/// event are otherwise unaltered and unchecked by the X server except to force
/// send_event to True in the forwarded event and to set the serial number in the
/// event correctly; therefore these fields and the display field are ignored by
/// XSendEvent.
///
/// XSendEvent returns zero if the conversion to wire protocol format failed and
/// returns nonzero otherwise. XSendEvent can generate BadValue and BadWindow
/// errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XSendEvent.3.xhtml
pub inline fn XSendEvent(
    display: *Display,
    window: Window,
    propagate: bool,
    event_mask: c_long,
    event: *XEvent,
) void {
    // Meaning of return value is not specified in documentation.
    _ = X.XSendEvent(display, window, @intFromBool(propagate), event_mask, event);
}

/// The XSetClassHint function sets the class hint for the specified window. If
/// the strings are not in the Host Portable Character Encoding, the result is
/// implementation-dependent.
///
/// XSetClassHint can generate BadAlloc and BadWindow errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XAllocClassHint.3.xhtml
pub inline fn XSetClassHint(
    display: *Display,
    window: Window,
    class_hint: *XClassHint,
) void {
    // Meaning of return value is not specified in documentation.
    _ = X.XSetClassHint(display, window, class_hint);
}

/// The XSetCloseDownMode defines what will happen to the client's resources at
/// connection close. A connection starts in DestroyAll mode. For information
/// on what happens to the client's resources when the close_mode argument is
/// RetainPermanent or RetainTemporary, see section 2.6.
///
/// XSetCloseDownMode can generate a BadValue error.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XSetCloseDownMode.3.xhtml
pub inline fn XSetCloseDownMode(
    display: *Display,
    /// Specifies the client close-down mode. You can pass DestroyAll,
    /// RetainPermanent, or RetainTemporary.
    close_mode: CloseMode,
) void {
    // Meaning of return value is not specified in documentation.
    _ = X.XSetCloseDownMode(display, @intFromEnum(close_mode));
}

const ErrHandler = *const fn (?*Display, [*c]XErrorEvent) callconv(.c) c_int;

/// Xlib generally calls the program's supplied error handler whenever an error
/// is received. It is not called on BadName errors from OpenFont, LookupColor,
/// or AllocNamedColor protocol requests or on BadFont errors from a QueryFont
/// protocol request. These errors generally are reflected back to the program
/// through the procedural interface. Because this condition is not assumed to
/// be fatal, it is acceptable for your error handler to return; the returned
/// value is ignored. However, the error handler should not call any functions
/// (directly or indirectly) on the display that will generate protocol
/// requests or that will look for input events. The previous error handler is
/// returned.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XSetErrorHandler.3.xhtml
pub inline fn XSetErrorHandler(f: ?ErrHandler) (?ErrHandler) {
    return X.XSetErrorHandler(f);
}

/// The XSetForeground function sets the foreground in the specified GC.
///
/// XSetForeground can generate BadAlloc and BadGC errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XSetState.3.xhtml
pub inline fn XSetForeground(
    display: *Display,
    gc: GC,
    /// Specifies the foreground you want to set for the specified GC.
    foreground: c_ulong,
) void {
    // Meaning of return value is not specified in documentation.
    _ = X.XSetForeground(display, gc, foreground);
}

/// The XSetInputFocus function changes the input focus and the
/// last-focus-change time. It has no effect if the specified time is earlier
/// than the current last-focus-change time or is later than the current X
/// server time. Otherwise, the last-focus-change time is set to the specified
/// time (CurrentTime is replaced by the current X server time). XSetInputFocus
/// causes the X server to generate FocusIn and FocusOut events.
///
/// Depending on the focus argument, the following occurs:
///
/// * If focus is None, all keyboard events are discarded until a new focus
///   window is set, and the revert_to argument is ignored.
/// * If focus is a window, it becomes the keyboard's focus window. If a
///   generated keyboard event would normally be reported to this window or one
///   of its inferiors, the event is reported as usual. Otherwise, the event is
///   reported relative to the focus window.
/// * If focus is PointerRoot, the focus window is dynamically taken to be the
///   root window of whatever screen the pointer is on at each keyboard event. In
///   this case, the revert_to argument is ignored.
///
/// The specified focus window must be viewable at the time XSetInputFocus is
/// called, or a BadMatch error results. If the focus window later becomes not
/// viewable, the X server evaluates the revert_to argument to determine the
/// new focus window as follows:
///
/// * If revert_to is RevertToParent, the focus reverts to the parent (or the
///   closest viewable ancestor), and the new revert_to value is taken to be
///   RevertToNone.
/// * If revert_to is RevertToPointerRoot or RevertToNone, the focus reverts to
///   PointerRoot or None, respectively. When the focus reverts, the X server
///   generates FocusIn and FocusOut events, but the last-focus-change time is
///   not affected.
///
/// XSetInputFocus can generate BadMatch, BadValue, and BadWindow errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XSetInputFocus.3.xhtml
pub inline fn XSetInputFocus(
    display: *Display,
    window: Window,
    revert_to: RevertTo,
    time: Time,
) void {
    // Meaning of return value is not specified in documentation.
    _ = X.XSetInputFocus(display, window, @intFromEnum(revert_to), time);
}

/// The XSetLineAttributes function sets the line drawing components in the specified GC.
///
/// XSetLineAttributes can generate BadAlloc, BadGC, and BadValue errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XSetLineAttributes.3.xhtml
pub inline fn XSetLineAttributes(
    display: *Display,
    gc: GC,
    line_width: c_uint,
    line_style: LineStyle,
    cap_style: CapStyle,
    join_style: JoinStyle,
) void {
    // Meaning of return value is not specified in documentation.
    _ = X.XSetLineAttributes(
        display,
        gc,
        line_width,
        @intFromEnum(line_style),
        @intFromEnum(cap_style),
        @intFromEnum(join_style),
    );
}

/// The XSetWindowBorder function sets the border of the window to the pixel
/// value you specify. If you attempt to perform this on an InputOnly window, a
/// BadMatch error results.
///
/// XSetWindowBorder can generate BadMatch and BadWindow errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XChangeWindowAttributes.3.xhtml
pub inline fn XSetWindowBorder(
    display: *Display,
    window: Window,
    border_pixel: c_ulong,
) void {
    // Meaning of return value is not specified in documentation.
    _ = X.XSetWindowBorder(display, window, border_pixel);
}

/// The XSetWMHints function sets the window manager hints that include icon
/// information and location, the initial state of the window, and whether the
/// application relies on the window manager to get keyboard input.
///
/// XSetWMHints can generate BadAlloc and BadWindow errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XAllocWMHints.3.xhtml
pub inline fn XSetWMHints(display: *Display, window: Window, value: *XWMHints) void {
    // Meaning of return value is not specified in documentation.
    _ = X.XSetWMHints(display, window, value);
}

/// The XSupportsLocale function returns True if Xlib functions are capable of
/// operating under the current locale. If it returns False, Xlib
/// locale-dependent functions for which the XLocaleNotSupported return status
/// is defined will return XLocaleNotSupported. Other Xlib locale-dependent
/// routines will operate in the "C" locale.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XSupportsLocale.3.xhtml
pub inline fn XSupportsLocale() bool {
    return X.XSupportsLocale() != 0;
}

/// The XSync function flushes the output buffer and then waits until all
/// requests have been received and processed by the X server. Any errors
/// generated must be handled by the error handler. For each protocol error
/// received by Xlib, XSync calls the client application's error handling
/// routine. Any events generated by the server are enqueued into the library's
/// event queue.
///
/// Finally, if you passed False, XSync does not discard the events in the
/// queue. If you passed True, XSync discards all events in the queue,
/// including those events that were on the queue before XSync was called.
/// Client applications seldom need to call XSync.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XFlush.3.xhtml
pub inline fn XSync(display: *Display, discard: bool) void {
    // According to the docs in the source, the c_int output is only important
    // in the other functions documented on that html page, but not XSync. So
    // we discard it.
    _ = X.XSync(display, @intFromBool(discard));
}

/// The XUngrabButton function releases the passive button/key combination on
/// the specified window if it was grabbed by this client. A modifiers of
/// AnyModifier is equivalent to issuing the ungrab request for all possible
/// modifier combinations, including the combination of no modifiers. A button
/// of AnyButton is equivalent to issuing the request for all possible buttons.
/// XUngrabButton has no effect on an active grab.
///
/// XUngrabButton can generate BadValue and BadWindow errors.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XGrabButton.3.xhtml
pub inline fn XUngrabButton(
    display: *Display,
    /// Specifies the pointer button that is to be grabbed or released or
    /// AnyButton.
    button: c_uint,
    /// Specifies the set of keymasks or AnyModifier. The mask is the bitwise
    /// inclusive OR of the valid keymask bits.
    modifiers: c_uint,
    grab_window: Window,
) void {
    // The meaning of the return value was not specified in documentation.
    _ = X.XUngrabButton(display, button, modifiers, grab_window);
}

/// The XUngrabKey function releases the key combination on the specified
/// window if it was grabbed by this client. It has no effect on an active
/// grab. A modifiers of AnyModifier is equivalent to issuing the request for
/// all possible modifier combinations (including the combination of no
/// modifiers). A keycode argument of AnyKey is equivalent to issuing the
/// request for all possible key codes.
///
/// XUngrabKey can generate BadValue and BadWindow error.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XGrabKey.3.xhtml
pub inline fn XUngrabKey(
    display: *Display,
    /// Specifies the KeyCode or AnyKey.
    keycode: c_uint,
    /// Specifies the set of keymasks or AnyModifier. The mask is the bitwise
    /// inclusive OR of the valid keymask bits.
    modifiers: c_uint,
    grab_window: Window,
) void {
    // The meaning of the return value was not specified in documentation.
    _ = X.XUngrabKey(display, keycode, modifiers, grab_window);
}

/// The XUngrabPointer function releases the pointer and any queued events if
/// this client has actively grabbed the pointer from XGrabPointer,
/// XGrabButton, or from a normal button press. XUngrabPointer does not release
/// the pointer if the specified time is earlier than the last-pointer-grab
/// time or is later than the current X server time. It also generates
/// EnterNotify and LeaveNotify events. The X server performs an UngrabPointer
/// request automatically if the event window or confine_to window for an
/// active pointer grab becomes not viewable or if window reconfiguration
/// causes the confine_to window to lie completely outside the boundaries of
/// the root window.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XGrabPointer.3.xhtml
pub inline fn XUngrabPointer(display: *Display, time: Time) void {
    // The meaning of the return value was not specified in documentation.
    _ = X.XUngrabPointer(display, time);
}

/// The XUngrabServer function restarts processing of requests and close downs
/// on other connections. You should avoid grabbing the X server as much as
/// possible.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XGrabServer.3.xhtml
pub inline fn XUngrabServer(display: *Display) void {
    // Meaning of return value is not specified in documentation.
    _ = X.XUngrabServer(display);
}

/// The XUnmapWindow function unmaps the specified window and causes the X
/// server to generate an UnmapNotify event. If the specified window is already
/// unmapped, XUnmapWindow has no effect. Normal exposure processing on
/// formerly obscured windows is performed. Any child window will no longer be
/// visible until another map call is made on the parent. In other words, the
/// subwindows are still mapped but are not visible until the parent is mapped.
/// Unmapping a window will generate Expose events on windows that were
/// formerly obscured by it.
///
/// XUnmapWindow can generate a BadWindow error.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XUnmapWindow.3.xhtml
pub inline fn XUnmapWindow(display: *Display, window: Window) void {
    // Meaning of return value is not specified in documentation.
    _ = X.XUnmapWindow(display, window);
}

/// If dest_w is None, XWarpPointer moves the pointer by the offsets (dest_x,
/// dest_y) relative to the current position of the pointer. If dest_w is a
/// window, XWarpPointer moves the pointer to the offsets (dest_x, dest_y)
/// relative to the origin of dest_w. However, if src_w is a window, the move
/// only takes place if the window src_w contains the pointer and if the
/// specified rectangle of src_w contains the pointer.
///
/// The src_x and src_y coordinates are relative to the origin of src_w. If
/// src_height is zero, it is replaced with the current height of src_w minus
/// src_y. If src_width is zero, it is replaced with the current width of src_w
/// minus src_x.
///
/// There is seldom any reason for calling this function. The pointer should
/// normally be left to the user. If you do use this function, however, it
/// generates events just as if the user had instantaneously moved the pointer
/// from one position to another. Note that you cannot use XWarpPointer to move
/// the pointer outside the confine_to window of an active pointer grab. An
/// attempt to do so will only move the pointer as far as the closest edge of
/// the confine_to window.
///
/// XWarpPointer can generate a BadWindow error.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XWarpPointer.3.xhtml
pub inline fn XWarpPointer(
    display: *Display,
    src_w: Window,
    dest_w: Window,
    src: Rect,
    dest_x: c_int,
    dest_y: c_int,
) void {
    // Meaning of return value is not specified in documentation.
    _ = X.XWarpPointer(display, src_w, dest_w, src.x, src.y, src.w, src.h, dest_x, dest_y);
}

/// The XmbTextPropertyToTextList, XwcTextPropertyToTextList and
/// Xutf8TextPropertyToTextList functions return a list of text strings
/// representing the null-separated elements of the specified XTextProperty
/// structure. The returned strings are encoded using the current locale
/// encoding (for XmbTextPropertyToTextList and XwcTextPropertyToTextList) or
/// in UTF-8 (for Xutf8TextPropertyToTextList). The data in text_prop must be
/// format 8.
///
/// Multiple elements of the property (for example, the strings in a disjoint
/// text selection) are separated by a null byte. The contents of the property
/// are not required to be null-terminated; any terminating null should not be
/// included in text_prop.nitems.
///
/// If insufficient memory is available for the list and its elements,
/// XmbTextPropertyToTextList, XwcTextPropertyToTextList and
/// Xutf8TextPropertyToTextList return XNoMemory. If the current locale is not
/// supported, the functions return XLocaleNotSupported. Otherwise, if the
/// encoding field of text_prop is not convertible to the encoding of the
/// current locale, the functions return XConverterNotFound. For supported
/// locales, existence of a converter from COMPOUND_TEXT, STRING, UTF8_STRING
/// or the encoding of the current locale is guaranteed if XSupportsLocale
/// returns True for the current locale (but the actual text may contain
/// unconvertible characters). Conversion of other encodings is
/// implementation-dependent. In all of these error cases, the functions do not
/// set any return values.
///
/// Otherwise, XmbTextPropertyToTextList, XwcTextPropertyToTextList and
/// Xutf8TextPropertyToTextList return the list of null-terminated text strings
/// to list_return and the number of text strings to count_return.
///
/// If the value field of text_prop is not fully convertible to the encoding of
/// the current locale, the functions return the number of unconvertible
/// characters. Each unconvertible character is converted to a string in the
/// current locale that is specific to the current locale. To obtain the value
/// of this string, use XDefaultString. Otherwise, XmbTextPropertyToTextList,
/// XwcTextPropertyToTextList and Xutf8TextPropertyToTextList return Success.
///
/// To free the storage for the list and its contents returned by
/// XmbTextPropertyToTextList or Xutf8TextPropertyToTextList, use
/// XFreeStringList. To free the storage for the list and its contents returned
/// by XwcTextPropertyToTextList, use XwcFreeStringList.
///
/// source: https://x.org/releases/X11R7.7/doc/man/man3/XmbTextListToTextProperty.3.xhtml
pub inline fn XmbTextPropertyToTextList(
    display: *Display,
    text_prop: *const XTextProperty,
) ?[][*c]u8 {
    var list_return: [*c][*c]u8 = undefined;
    var count_return: c_int = undefined;
    const result = X.XmbTextPropertyToTextList(
        display,
        text_prop,
        &list_return,
        &count_return,
    );
    switch (result) {
        X.XNoMemory => return null,
        X.XLocaleNotSupported => return null,
        X.XConverterNotFound => return null,
        else => {},
    }
    if (list_return) |list| {
        if (count_return > 0) {
            return list[0..@intCast(count_return)];
        }
    }
    return null;
}

// -----------------------------------------------------------------------------
// ++ Xft Functions
// -----------------------------------------------------------------------------

/// An XftFont's glyph or character coverage can be determined with
/// XftFontCheckGlyph() or XftCharExists(). XftCharIndex() returns the
/// XftFont-specific character index corresponding to a given Unicode
/// codepoint.
///
/// source: https://github.com
pub inline fn XftCharExists(
    display: *Display,
    font: *XftFont,
    codepoint: c_uint,
) bool {
    return X.XftCharExists(display, font, codepoint) != 0;
}

/// Use XAllocNamedColor() to look up the named color name for the screen
/// associated with the colormap cmap.
///
/// If XAllocNamedColor() returns nonzero, XftColorAllocName() fills in the
/// resulting XftColor pixel field with the closest color supported by the
/// screen, as well as the exact red, green and blue fields from the database,
/// and returns True.
///
/// If XAllocNamedColor() returns zero, XftColorAllocName() returns False, and
/// does not update the XftColor referenced by result.
///
/// The visual parameter is unused.
///
/// source: https://man.archlinux.org/man/XftColorAllocName.3
pub inline fn XftColorAllocName(
    display: *Display,
    visual: *Visual,
    cmap: Colormap,
    name: []const u8,
    result: *XftColor,
) bool {
    const status = X.XftColorAllocName(display, visual, cmap, name.ptr, result);
    return status != X.False;
}

/// If the visual class is not TrueColor, Xft calls XFreeColors() to free the
/// entry from the colormap cmap whose pixel value in the color parameter was
/// allocated by XftColorAllocName().
///
/// source: https://man.archlinux.org/man/XftColorAllocName.3
pub inline fn XftColorFree(
    display: *Display,
    visual: *Visual,
    cmap: Colormap,
    color: *XftColor,
) void {
    // Meaning of return value is not specified in documentation.
    _ = X.XftColorFree(display, visual, cmap, color);
}

/// XftDrawCreate creates a structure that can be used to render text and
/// rectangles using the specified drawable, visual, and colormap on display.
///
/// source: https://man.archlinux.org/man/XftColorAllocName.3
pub inline fn XftDrawCreate(
    display: *Display,
    drawable: Drawable,
    visual: *Visual,
    cmap: Colormap,
) ?*XftDraw {
    return X.XftDrawCreate(display, drawable, visual, cmap);
}

/// XftDrawDestroy destroys draw (created by one of the XftDrawCreate*()
/// functions) and frees the memory that was allocated for it.
///
/// source: https://man.archlinux.org/man/Xft.3
pub inline fn XftDrawDestroy(draw: *XftDraw) void {
    X.XftDrawDestroy(draw);
}

/// Draws no more than len glyphs of string to Xft drawable d using font in
/// color at position x, y.
///
/// source: https://man.archlinux.org/man/Xft.3
pub inline fn XftDrawStringUtf8(
    d: *XftDraw,
    color: *const XftColor,
    font: *XftFont,
    x: c_int,
    y: c_int,
    text: []const u8,
    len: c_int,
) void {
    X.XftDrawStringUtf8(d, color, font, x, y, text.ptr, len);
}

/// XftFonts are populated with any of XftFontOpen(), XftFontOpenName(),
/// XftFontOpenXlfd(), XftFontOpenInfo(), or XftFontOpenPattern().
/// XftFontCopy() is used to duplicate XftFonts, and XftFontClose() is used to
/// mark an XftFont as unused. XftFonts are internally allocated,
/// reference-counted, and freed by Xft; the programmer does not ordinarily
/// need to allocate or free storage for them.
///
/// source: https://man.archlinux.org/man/Xft.3
pub inline fn XftFontClose(display: *Display, font: *XftFont) void {
    X.XftFontClose(display, font);
}

/// Also used internally by the XftFontOpen* functions, XftFontMatch can also
/// be used directly to determine the Fontconfig font pattern resulting from an
/// Xft font open request.
///
/// source: https://man.archlinux.org/man/Xft.3
pub inline fn XftFontMatch(
    display: *Display,
    screen: c_int,
    pattern: *FcPattern,
    result: *FcResult,
) ?*FcPattern {
    return X.XftFontMatch(display, screen, pattern, result);
}

/// XftFontOpenName behaves as XftFontOpen does, except that it takes a
/// Fontconfig pattern string (which is passed to the Fontconfig library's
/// FcNameParse() function).
///
/// source: https://man.archlinux.org/man/Xft.3
pub inline fn XftFontOpenName(
    display: *Display,
    screen: c_int,
    name: []const u8,
) ?*XftFont {
    return X.XftFontOpenName(display, screen, name.ptr);
}

/// XftFonts are populated with any of XftFontOpen(), XftFontOpenName(),
/// XftFontOpenXlfd(), XftFontOpenInfo(), or XftFontOpenPattern().
/// XftFontCopy() is used to duplicate XftFonts, and XftFontClose() is used to
/// mark an XftFont as unused. XftFonts are internally allocated,
/// reference-counted, and freed by Xft; the programmer does not ordinarily
/// need to allocate or free storage for them.
///
/// source: https://man.archlinux.org/man/Xft.3
pub inline fn XftFontOpenPattern(display: *Display, pattern: *FcPattern) ?*XftFont {
    return X.XftFontOpenPattern(display, pattern);
}

/// XftTextExtentsUtf8 computes the pixel extents on display dpy of no more
/// than len bytes of a UTF-8 encoded string when drawn with font, storing them
/// in extents.
///
/// source: https://man.archlinux.org/man/Xft.3
pub inline fn XftTextExtentsUtf8(
    display: *Display,
    font: *XftFont,
    text: []const u8,
    extents: *XGlyphInfo,
) void {
    X.XftTextExtentsUtf8(display, font, text.ptr, @intCast(text.len), extents);
}

// -----------------------------------------------------------------------------
// ++ Enums
// -----------------------------------------------------------------------------

/// At the conceptual level, atoms are unique names that clients can use to
/// communicate information to each other. They can be thought of as a bundle
/// of octets, like a string but without an encoding being specified. The
/// elements are not necessarily ASCII characters, and no case folding happens.
///
/// The protocol designers felt that passing these sequences of bytes back and
/// forth across the wire would be too costly. Further, they thought it
/// important that events as they appear on the wire have a fixed size (in
/// fact, 32 bytes) and that because some events contain atoms, a fixed-size
/// representation for them was needed.
///
/// To allow a fixed-size representation, a protocol request (InternAtom) was
/// provided to register a byte sequence with the server, which returns a
/// 32-bit value (with the top three bits zero) that maps to the byte sequence.
/// The inverse operator is also available (GetAtomName).
///
/// source: https://x.org/releases/X11R7.7/doc/xorg-docs/icccm/icccm.html
pub const Atom = X.Atom;

pub const Below = X.Below;
pub const ButtonPress = X.ButtonPress;
pub const ClientMessage = X.ClientMessage;
pub const ConfigureNotify = X.ConfigureNotify;
pub const ConfigureRequest = X.ConfigureRequest;
pub const CopyFromParent = X.CopyFromParent;
pub const CurrentTime = X.CurrentTime;
pub const DestroyAll = X.DestroyAll;
pub const DestroyNotify = X.DestroyNotify;
pub const EnterNotify = X.EnterNotify;
pub const Expose = X.Expose;
pub const FocusIn = X.FocusIn;
pub const IconicState = X.IconicState;
pub const IsViewable = X.IsViewable;
pub const KeyPress = X.KeyPress;
pub const LockMask = X.LockMask;
pub const MapRequest = X.MapRequest;
pub const MappingKeyboard = X.MappingKeyboard;
pub const MappingNotify = X.MappingNotify;
pub const MotionNotify = X.MotionNotify;
pub const NormalState = X.NormalState;
pub const ParentRelative = X.ParentRelative;
pub const PointerRoot = X.PointerRoot;
pub const PropertyDelete = X.PropertyDelete;
pub const PropertyNotify = X.PropertyNotify;
pub const ReplayPointer = X.ReplayPointer;
pub const UnmapNotify = X.UnmapNotify;

pub const NotifyInferior = X.NotifyInferior;
pub const NotifyNormal = X.NotifyNormal;

pub const FC_CHARSET = X.FC_CHARSET;
pub const FC_SCALABLE = X.FC_SCALABLE;

pub const LASTEvent = X.LASTEvent;
pub const XA_ATOM = X.XA_ATOM;
pub const XA_STRING = X.XA_STRING;
pub const XA_WINDOW = X.XA_WINDOW;
pub const XA_WM_HINTS = X.XA_WM_HINTS;
pub const XA_WM_NAME = X.XA_WM_NAME;
pub const XA_WM_NORMAL_HINTS = X.XA_WM_NORMAL_HINTS;
pub const XA_WM_TRANSIENT_FOR = X.XA_WM_TRANSIENT_FOR;

/// Specifies whether the data should be viewed as a list of 8-bit, 16-bit, or
/// 32-bit quantities. Used in XGetWindowProperty, among other places.
pub const Format = enum {
    // Data should be read as an 8-bit value.
    Fmt8,
    // Data should be read as an 16-bit value.
    Fmt16,
    // Data should be read as an 32-bit value.
    Fmt32,
};

pub const FormattedData = union(Format) {
    const Self = @This();
    Fmt8: []u8,
    Fmt16: []u16,
    Fmt32: []u32,

    pub inline fn len(self: *const Self) usize {
        return switch (self.*) {
            .Fmt8 => |v| v.len,
            .Fmt16 => |v| v.len,
            .Fmt32 => |v| v.len,
        };
    }

    pub inline fn deinit(self: Self) void {
        switch (self) {
            .Fmt8 => |v| XFree(v.ptr),
            .Fmt16 => |v| XFree(v.ptr),
            .Fmt32 => |v| XFree(v.ptr),
        }
    }
};

pub const EventMode = enum(c_int) {
    AsyncPointer = X.AsyncPointer,
    SyncPointer = X.SyncPointer,
    ReplayPointer = X.ReplayPointer,
    AsyncKeyboard = X.AsyncKeyboard,
    SyncKeyboard = X.SyncKeyboard,
    ReplayKeyboard = X.ReplayKeyboard,
    AsyncBoth = X.AsyncBoth,
    SyncBoth = X.SyncBoth,
};

pub const GrabMode = enum(c_int) {
    Sync = X.GrabModeSync,
    Async = X.GrabModeAsync,
};

pub const PropMode = enum(c_int) {
    Replace = X.PropModeReplace,
    Prepend = X.PropModePrepend,
    Append = X.PropModeAppend,
};

pub const FcMatch = enum(c_int) {
    Pattern = X.FcMatchPattern,
    Font = X.FcMatchFont,
    Scan = X.FcMatchScan,
};

pub const RevertTo = enum(c_int) {
    None = X.RevertToNone,
    PointerRoot = X.RevertToPointerRoot,
    Parent = X.RevertToParent,
};

pub const WindowState = enum(c_int) {
    WithdrawnState = X.WithdrawnState,
    NormalState = X.NormalState,
    IconicState = X.IconicState,
};

pub const CloseMode = enum(c_int) {
    DestroyAll = X.DestroyAll,
    RetainPermanent = X.RetainPermanent,
    RetainTemporary = X.RetainTemporary,
};

/// There are many more enums than this, just check out any of the X.XC_* stuff.
/// and the rest should be adjacent.
pub const PointerShape = enum(@TypeOf(X.XC_left_ptr)) {
    /// Commonly used for dragging things around. The four arrows spread out
    /// from the center.
    Fleur = X.XC_fleur,
    Left_ptr = X.XC_left_ptr,
    Sizing = X.XC_sizing,
};

pub const JoinStyle = enum(c_int) {
    Miter = X.JoinMiter,
    Round = X.JoinRound,
    Bevel = X.JoinBevel,
};

pub const LineStyle = enum(c_int) {
    Solid = X.LineSolid,
    OnOffDash = X.LineOnOffDash,
    DoubleDash = X.LineDoubleDash,
};

pub const CapStyle = enum(c_int) {
    NotLast = X.CapNotLast,
    Butt = X.CapButt,
    Round = X.CapRound,
    Projecting = X.CapProjecting,
};

pub const None = X.None;

pub const False = X.False;
pub const True = X.True;

// -----------------------------------------------------------------------------
// ++ Bitmasks
// -----------------------------------------------------------------------------

pub const masks = struct {
    pub const ShiftMask = X.ShiftMask;
    pub const ControlMask = X.ControlMask;
    pub const ButtonPressMask = X.ButtonPressMask;
    pub const ButtonReleaseMask = X.ButtonReleaseMask;

    pub const Mod1Mask = X.Mod1Mask;
    pub const Mod2Mask = X.Mod2Mask;
    pub const Mod3Mask = X.Mod3Mask;
    pub const Mod4Mask = X.Mod4Mask;
    pub const Mod5Mask = X.Mod5Mask;

    pub const CWX = X.CWX;
    pub const CWY = X.CWY;
    pub const CWWidth = X.CWWidth;
    pub const CWHeight = X.CWHeight;
    pub const CWBorderWidth = X.CWBorderWidth;
    pub const CWCursor = X.CWCursor;
    pub const CWEventMask = X.CWEventMask;

    // For XSelectInput
    pub const EnterWindowMask = X.EnterWindowMask;
    pub const FocusChangeMask = X.FocusChangeMask;
    pub const PropertyChangeMask = X.PropertyChangeMask;
    pub const StructureNotifyMask = X.StructureNotifyMask;

    pub const ExposureMask = X.ExposureMask;
    pub const LeaveWindowMask = X.LeaveWindowMask;
    pub const PointerMotionMask = X.PointerMotionMask;
    pub const SubstructureNotifyMask = X.SubstructureNotifyMask;
    pub const SubstructureRedirectMask = X.SubstructureRedirectMask;
    pub const NoEventMask = X.NoEventMask;

    pub const PAspect = X.PAspect;
    pub const PBaseSize = X.PBaseSize;
    pub const PMaxSize = X.PMaxSize;
    pub const PMinSize = X.PMinSize;
    pub const PResizeInc = X.PResizeInc;
    pub const PSize = X.PSize;

    pub const InputHint = X.InputHint;
    pub const XUrgencyHint = X.XUrgencyHint;
};

// -----------------------------------------------------------------------------
// ++ Keys and buttons
// -----------------------------------------------------------------------------

pub const keys = struct {
    // zig fmt: off
    pub const XK_a = X.XK_a; pub const XK_b = X.XK_b; pub const XK_c = X.XK_c; pub const XK_d = X.XK_d;
    pub const XK_e = X.XK_e; pub const XK_f = X.XK_f; pub const XK_g = X.XK_g; pub const XK_h = X.XK_h;
    pub const XK_i = X.XK_i; pub const XK_j = X.XK_j; pub const XK_k = X.XK_k; pub const XK_l = X.XK_l;
    pub const XK_m = X.XK_m; pub const XK_n = X.XK_n; pub const XK_o = X.XK_o; pub const XK_p = X.XK_p;
    pub const XK_q = X.XK_q; pub const XK_r = X.XK_r; pub const XK_s = X.XK_s; pub const XK_t = X.XK_t;
    pub const XK_u = X.XK_u; pub const XK_v = X.XK_v; pub const XK_w = X.XK_w; pub const XK_x = X.XK_x;
    pub const XK_y = X.XK_y; pub const XK_z = X.XK_z; // lower caae
    pub const XK_A = X.XK_A; pub const XK_B = X.XK_B; pub const XK_C = X.XK_C; pub const XK_D = X.XK_D;
    pub const XK_E = X.XK_E; pub const XK_F = X.XK_F; pub const XK_G = X.XK_G; pub const XK_H = X.XK_H;
    pub const XK_I = X.XK_I; pub const XK_J = X.XK_J; pub const XK_K = X.XK_K; pub const XK_L = X.XK_L;
    pub const XK_M = X.XK_M; pub const XK_N = X.XK_N; pub const XK_O = X.XK_O; pub const XK_P = X.XK_P;
    pub const XK_Q = X.XK_Q; pub const XK_R = X.XK_R; pub const XK_S = X.XK_S; pub const XK_T = X.XK_T;
    pub const XK_U = X.XK_U; pub const XK_V = X.XK_V; pub const XK_W = X.XK_W; pub const XK_X = X.XK_X;
    pub const XK_Y = X.XK_Y; pub const XK_Z = X.XK_Z; // upper case
    pub const XK_0 = X.XK_0; pub const XK_1 = X.XK_1; pub const XK_2 = X.XK_2; pub const XK_3 = X.XK_3;
    pub const XK_4 = X.XK_4; pub const XK_5 = X.XK_5; pub const XK_6 = X.XK_6; pub const XK_7 = X.XK_7;
    pub const XK_8 = X.XK_8; pub const XK_9 = X.XK_9; // numbers
    // zig fmt: on
    pub const XK_Return = X.XK_Return;
    pub const XK_Tab = X.XK_Tab;
    pub const XK_comma = X.XK_comma;
    pub const XK_equal = X.XK_equal;
    pub const XK_minus = X.XK_minus;
    pub const XK_period = X.XK_period;
    pub const XK_space = X.XK_space;
    pub const XK_Num_Lock = X.XK_Num_Lock;

    // AwesomeWM provides a very helpful graphic here:
    // https://awesomewm.org/doc/api/libraries/mouse.html

    /// Left click.
    pub const Button1 = X.Button1;
    /// Middle click.
    pub const Button2 = X.Button2;
    /// Right click.
    pub const Button3 = X.Button3;
    pub const Button4 = X.Button4;
    pub const Button5 = X.Button5;
};

// -----------------------------------------------------------------------------
// ++ Macros
// -----------------------------------------------------------------------------

pub const ConnectionNumber = X.ConnectionNumber;
pub const DefaultColormap = X.DefaultColormap;
pub const DefaultDepth = X.DefaultDepth;
pub const DefaultRootWindow = X.DefaultRootWindow;
pub const DefaultScreen = X.DefaultScreen;
pub const DefaultVisual = X.DefaultVisual;
pub const DisplayHeight = X.DisplayHeight;
pub const DisplayWidth = X.DisplayWidth;
pub const RootWindow = X.RootWindow;

// -----------------------------------------------------------------------------
// ++ Errors
// -----------------------------------------------------------------------------

pub const err = struct {
    pub const BadAccess = X.BadAccess;
    pub const BadDrawable = X.BadDrawable;
    pub const BadGC = X.BadGC;
    pub const BadMatch = X.BadMatch;
};

// -----------------------------------------------------------------------------
// ++ RequestCodes
// -----------------------------------------------------------------------------

/// Request Code of a XErrorEvent.
pub const rq = struct {
    pub const ConfigureWindow = X.X_ConfigureWindow;
    pub const GrabButton = X.X_GrabButton;
    pub const GrabKey = X.X_GrabKey;
    pub const SetInputFocus = X.X_SetInputFocus;
    pub const CopyArea = X.X_CopyArea;
    pub const PolySegment = X.X_PolySegment;
    pub const PolyFillRectangle = X.X_PolyFillRectangle;
    pub const PolyText8 = X.X_PolyText8;
};

// -----------------------------------------------------------------------------
// ++ FontConfig
// -----------------------------------------------------------------------------

/// Adds a single unicode char to the set, returning FcFalse on failure, either
/// as a result of a constant set or from running out of memory.
///
/// source: https://xorg.freedesktop.org/archive/X11R7.0/doc/html/FcCharSetAddChar.3.html
/// source: https://fontconfig.pages.freedesktop.org/fontconfig/fontconfig-devel/
pub inline fn FcCharSetAddChar(fcs: *FcCharSet, ucs4: c_uint) bool {
    return X.FcCharSetAddChar(fcs, ucs4) != X.FcFalse;
}

/// Allocates and initializes a new empty character set object.
///
/// source: https://xorg.freedesktop.org/archive/X11R7.0/doc/html/FcCharSetCreate.3.html
pub inline fn FcCharSetCreate() ?*FcCharSet {
    return X.FcCharSetCreate();
}

/// Destroy a character set. Decrements the reference count fcs. If the
/// reference count becomes zero, all memory referenced is freed.
///
/// source: https://xorg.freedesktop.org/archive/X11R7.0/doc/html/FcCharSetDestroy.3.html
pub inline fn FcCharSetDestroy(fcs: *FcCharSet) void {
    X.FcCharSetDestroy(fcs);
}

/// Execute substitutions. Calls FcConfigSubstituteWithPat setting p_pat to NULL.
///
/// [FcConfigSubstituteWithPat] Performs the sequence of pattern modification
/// operations, if kind is FcMatchPattern, then those tagged as pattern
/// operations are applied, else if kind is FcMatchFont, those tagged as font
/// operations are applied and p_pat is used for <test> elements with
/// target=pattern.
///
/// source: https://xorg.freedesktop.org/archive/X11R7.0/doc/html/FcConfigSubstitute.3.html
pub inline fn FcConfigSubstitute(config: ?*FcConfig, p: *FcPattern, kind: FcMatch) bool {
    return X.FcConfigSubstitute(config, p, @intFromEnum(kind)) != X.FcFalse;
}

/// Perform default substitutions in a pattern.
///
/// Supplies default values for underspecified font patterns:
///
/// * Patterns without a specified style or weight are set to Medium.
/// * Patterns without a specified style or slant are set to Roman.
/// * Patterns without a specified pixel size are given one computed from any
///   specified point size (default 12), dpi (default 75) and scale (default 1).
///
/// source: https://xorg.freedesktop.org/archive/X11R7.0/doc/html/FcDefaultSubstitute.3.html
pub inline fn FcDefaultSubstitute(p: *FcPattern) void {
    X.FcDefaultSubstitute(p);
}

/// Parse a pattern string.
///
/// Converts name from the standard text format described above into a pattern.
///
/// source: https://xorg.freedesktop.org/archive/X11R7.0/doc/html/FcNameParse.3.html
pub inline fn FcNameParse(name: []const u8) ?*FcPattern {
    return X.FcNameParse(name.ptr);
}

/// [FcPatternAdd-Type] These are all convenience functions that insert objects
/// of the specified type into the pattern. Use these in preference to
/// FcPatternAdd as they will provide compile-time typechecking. These all
/// append values to any existing list of values.
///
/// source: https://xorg.freedesktop.org/archive/X11R7.0/doc/html/FcPatternAdd-Type.3.html
pub inline fn FcPatternAddBool(p: *FcPattern, object: [*c]const u8, value: bool) bool {
    return X.FcPatternAddBool(p, object, @intFromBool(value)) != X.FcFalse;
}

/// [FcPatternAdd-Type] These are all convenience functions that insert objects
/// of the specified type into the pattern. Use these in preference to
/// FcPatternAdd as they will provide compile-time typechecking. These all
/// append values to any existing list of values.
///
/// source: https://xorg.freedesktop.org/archive/X11R7.0/doc/html/FcPatternAdd-Type.3.html
pub inline fn FcPatternAddCharSet(p: *FcPattern, object: [*c]const u8, charset: *FcCharSet) bool {
    return X.FcPatternAddCharSet(p, object, charset) != X.FcFalse;
}

/// Destroys a pattern, in the process destroying all related values.
///
/// source: https://xorg.freedesktop.org/archive/X11R7.0/doc/html/FcPatternDestroy.3.html
pub inline fn FcPatternDestroy(p: *FcPattern) void {
    X.FcPatternDestroy(p);
}

/// Copy a pattern, returning a new pattern that matches p. Each pattern may be
/// modified without affecting the other.
///
/// source: https://freedesktop.org/software/fontconfig/fontconfig-devel/fcpatternduplicate.html
pub inline fn FcPatternDuplicate(p: *const FcPattern) ?*FcPattern {
    return X.FcPatternDuplicate(p);
}

////////////////////////////////////////////////////////////////////////////////
// Resources
// * https://x.org/releases/X11R7.7/doc/xproto/x11protocol.html
// * https://x.org/releases/X11R7.7/doc/man/man3/
// * https://xorg.freedesktop.org/archive/X11R7.0/doc/html/manindex3.html
