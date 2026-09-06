pub const Rect = struct {
    const Self = @This();

    /// X-coordinate. Increases from left to right. (i.e., the value here
    /// represents the left-most x-value of the rectangle)
    x: c_int,
    /// Y-coordinate. Increases from top to bottom. (i.e., the value here
    /// represents the top-most y-value of the rectangle)
    y: c_int,
    /// Width.
    w: c_uint,
    /// Height.
    h: c_uint,

    pub const zero = Self{ .x = 0, .y = 0, .w = 0, .h = 0 };

    /// Translate from this to an X11 struct. Use keys [x, y, width, height].
    pub fn toX(self: *const Self, comptime T: type) T {
        return .{
            .x = @intCast(self.x),
            .y = @intCast(self.y),
            .width = @intCast(self.w),
            .height = @intCast(self.h),
        };
    }

    /// Translate from an X11 struct to this. Use keys [x, y, width, height].
    pub fn fromX(comptime T: type, z: *T) Self {
        return .{ .x = @intCast(z.x), .y = @intCast(z.y), .w = @intCast(z.width), .h = @intCast(z.height) };
    }

    pub fn eq(lhs: *const Self, rhs: *const Self) bool {
        return lhs.x == rhs.x and lhs.y == rhs.y and lhs.w == rhs.w and lhs.h == rhs.h;
    }

    /// The left-most coordinate. Use `self.x` where it's sufficiently clear.
    pub inline fn l(self: *const Self) c_int {
        return self.x;
    }

    /// The right-most coordinate.
    pub inline fn r(self: *const Self) c_int {
        return self.x + @as(c_int, @intCast(self.w));
    }

    /// The top-most coordinate. Use `self.y` where it's sufficiently clear.
    pub inline fn t(self: *const Self) c_int {
        return self.y;
    }

    /// The bottom-most coordinate.
    pub inline fn b(self: *const Self) c_int {
        return self.y + @as(c_int, @intCast(self.h));
    }

    /// (dwm) INTERSECT
    /// Get the area of intersection. Always returns a non-negative value.
    fn intersect(lhs: *const Self, rhs: *const Self) c_int {
        const width: c_int = @min(lhs.r(), rhs.r()) - @max(lhs.l(), rhs.l());
        const height: c_int = @min(lhs.b(), rhs.b()) - @max(lhs.t(), rhs.t());
        return @max(0, width) * @max(0, height);
    }

    /// Calculates the snap location within the parent.
    pub fn snap(self: *Self, parent: *const Self, padding: c_int, snapDist: c_int) void {
        const p = 2 * padding;
        // Too far left -> snap to the left vertical bound.
        if (@abs(parent.x - self.x) < snapDist) {
            self.x = parent.x;
        }
        // Too far right -> snap to the right vertical bound.
        else if (@abs(parent.r() - (self.x + p)) < snapDist) {
            self.x = parent.r() - p;
        }
        // Too far up -> snap to the top horizontal bound.
        if (@abs(parent.y - self.y) < snapDist) {
            self.y = parent.y;
        }
        // Too far down -> snap to the bottom horizontal bound.
        else if (@abs(parent.b() - (self.y + p)) < snapDist) {
            self.y = parent.b() - p;
        }
    }
};

pub const Coordinates = struct {
    const Self = @This();

    x: c_int,
    y: c_int,

    pub const zero: Self = .{ .x = 0, .y = 0 };

    pub inline fn eq(lhs: *const Self, rhs: *const Self) bool {
        return lhs.x == rhs.x and lhs.y == rhs.y;
    }
};
