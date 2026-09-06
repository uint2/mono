/// C standard library.
pub const C = @cImport({
    @cInclude("locale.h");
    @cInclude("signal.h");
    @cInclude("unistd.h");
    @cInclude("time.h");
});
