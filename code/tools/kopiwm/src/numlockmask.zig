const X = @import("x11.zig");
const M = @import("x11.zig").masks;

pub const NumLockMask = struct {
    const Self = @This();

    /// [0]: 0
    /// [1]: X.LockMask
    /// [2]: numlockmask  <----  important!
    /// [3]: numlockmask | X.LockMask
    modifiers: [4]c_uint,

    pub const empty = Self{
        .modifiers = .{ 0, X.LockMask, 0, X.LockMask },
    };

    /// Updates the numlockmask, which is located at `self.modifiers[2]`.
    pub fn update(self: *Self, dpy: *X.Display) void {
        // Reset numlockmask.
        self.modifiers[2] = 0;

        const modmap = X.XGetModifierMapping(dpy) orelse return;
        defer X.XFreeModifiermap(modmap);
        const mkpm: usize = @intCast(modmap.*.max_keypermod);
        for (0..8) |i| {
            for (0..mkpm) |j| {
                const keycode = modmap.*.modifiermap[i * mkpm + j];
                if (keycode == X.XKeysymToKeycode(dpy, X.keys.XK_Num_Lock)) {
                    self.modifiers[2] = @as(u32, 1) << @intCast(i);
                }
            }
        }

        self.modifiers[3] = self.modifiers[2] | X.LockMask;
    }

    /// (dwm) CLEANMASK
    pub fn cleanMask(self: *const Self, mask: c_uint) c_uint {
        return (mask & ~self.modifiers[3]) &
            (M.ShiftMask | M.ControlMask | M.Mod1Mask | M.Mod2Mask | M.Mod3Mask | M.Mod4Mask | M.Mod5Mask);
    }
};
