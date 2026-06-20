const Monitor = @import("monitor.zig").Monitor;
const Coordinates = @import("enums.zig").Coordinates;

pub fn Rect2(
    /// Generic type for the coordinates (and area).
    C: type,
    /// Generic type for the lengths.
    L: type,
) type {
    return struct {
        const Self = @This();

        /// X-coordinate. Increases from left to right. (i.e., the value here
        /// represents the left-most x-value of the rectangle)
        x: C,
        /// Y-coordinate. Increases from top to bottom. (i.e., the value here
        /// represents the top-most y-value of the rectangle)
        y: C,
        /// Width.
        w: L,
        /// Height.
        h: L,

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

        pub fn toCoordinates(self: *const Self) Coordinates(C) {
            return .{ .x = self.x, .y = self.y };
        }

        pub fn convert(z: *const Self, X2: type, W2: type) Rect2(X2, W2) {
            return .{ .x = @intCast(z.x), .y = @intCast(z.y), .w = @intCast(z.width), .h = @intCast(z.height) };
        }

        /// (dwm) recttomon
        /// Searches the list of monitors for the one with the biggest intersection
        /// with `self` (using Monitor.w), and returns that one.
        pub fn toMonitor(self: *const Self, mons: ?*Monitor) ?*Monitor {
            var ret: ?*Monitor = null;
            var max_area: C = 0;
            var area: C = 0;
            var m_opt = mons;
            while (m_opt) |m| : (m_opt = m.next) {
                area = self.intersect(&m.w);
                if (area > max_area) {
                    max_area = area;
                    ret = m;
                }
            }
            return ret;
        }

        pub fn eq(lhs: *const Self, rhs: *const Self) bool {
            return lhs.x == rhs.x and lhs.y == rhs.y and lhs.w == rhs.w and lhs.h == rhs.h;
        }

        /// The left-most coordinate. Use `self.x` where it's sufficiently clear.
        pub inline fn l(self: *const Self) C {
            return self.x;
        }

        /// The right-most coordinate.
        pub inline fn r(self: *const Self) C {
            return self.x + @as(C, @intCast(self.w));
        }

        /// The top-most coordinate. Use `self.y` where it's sufficiently clear.
        pub inline fn t(self: *const Self) C {
            return self.y;
        }

        /// The bottom-most coordinate.
        pub inline fn b(self: *const Self) C {
            return self.y + @as(C, @intCast(self.h));
        }

        /// (dwm) INTERSECT
        /// Get the area of intersection.
        fn intersect(lhs: *const Self, rhs: *const Self) C {
            return @max(0, @min(lhs.r(), rhs.r()) - @max(lhs.x, rhs.x)) *
                @max(0, @min(lhs.b(), rhs.b()) - @max(lhs.y, rhs.y));
        }
    };
}

pub const Rect = Rect2(c_int, c_uint);
