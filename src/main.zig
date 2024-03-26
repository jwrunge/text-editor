const std = @import("std");
const mem = std.mem;
const heap = std.heap;
const os = std.os;
const meta = std.meta;
const spoon = @import("../lib/zig-spoon/import.zig");

var term: spoon.Term = undefined;
var loop: bool = true;

const title = "Zig spoon test";
var counter: usize = 0;

pub fn main() !void {
    // When init'ing the terminal, you install a render function. This is where
    // you will do all your drawing.
    try term.init(render);
    defer term.deinit();

    // SIGWINCH is send when the terminal size is changed, for silly legacy reasons.
    os.sigaction(os.SIG.WINCH, &os.Sigaction{
        .handler = .{ .handler = handleSigWinch },
        .mask = os.empty_sigset,
        .flags = 0,
    }, null);

    var fds: [1]os.pollfd = undefined;
    fds[0] = .{
        .fd = term.tty.handle,
        .events = os.POLL.IN,
        .revents = undefined,
    };

    try term.uncook();
    defer term.cook() catch {};

    try term.hideCursor();

    // Before entering the main loop, you probably want to render once. Since
    // we never rendered before, we do not know the terminal size, so that needs
    // to be fetched first.
    try term.fetchSize();
    try term.setWindowTitle(title);
    try term.updateContent();

    var buf: [16]u8 = undefined;
    while (loop) {
        _ = try os.poll(&fds, -1);

        const read = try term.readInput(&buf);
        var it = spoon.inputParser(buf[0..read]);
        while (it.next()) |in| {
            if (inputEql(in, "C-c") or inputEql(in, "q")) {
                loop = false;
            } else if (inputEql(in, "space")) {
                counter += 1;

                // Call this function whenever you want to trigger a render.
                try term.updateContent();
            } else if (inputEql(in, "r")) {
                counter = 0;
                try term.updateContent();
            }
        }
    }
}

fn inputEql(in: spoon.Input, comptime descr: []const u8) bool {
    const bind = comptime spoon.Input.fromDescription(descr) catch @compileError("Bad input descriptor");
    return meta.eql(in, bind);
}

fn render(_: *spoon.Term, _: usize, columns: usize) !void {
    // This is a super lazy example program, so we clear the entire terminal and
    // draw everything again each time. However it is actually pretty simple to
    // roughly keep track of what has changed since last render and only
    // re-render that.
    try term.clear();

    // In a real application you probably want to check if the terminal is large
    // enough for the contents you plan to draw. However this is an abbreviated
    // example, so whatever.

    try term.moveCursorTo(0, 0);
    try term.setAttribute(.{ .fg = .green, .reverse = true });
    const rest = try term.writeLine(columns, " " ++ title);
    try term.writeByteNTimes(' ', rest);

    try term.moveCursorTo(1, 1);
    try term.setAttribute(.{ .fg = .red, .bold = true });
    _ = try term.writeLine(columns - 1, "Press space to increase the counter, r to reset, Ctrl-C or q to exit.");

    // The way you write things to the terminal is in my opinion the weakest
    // part of the API right now and will probably change quite a bit. However
    // for now you can just grab the writer and use the common write functions
    // you know and love from zigs std.
    try term.setAttribute(.{});
    try term.moveCursorTo(3, 3);
    const writer = term.stdout.writer();
    try writer.print("counter: {}", .{counter});
}

fn handleSigWinch(_: c_int) callconv(.C) void {
    // The size has changed, so we need to fetch it again and then render.
    term.fetchSize() catch {};
    term.updateContent() catch {};
}

/// Custom panic handler, so that we can try to cook the terminal on a crash,
/// as otherwise all messages will be mangled.
pub fn panic(msg: []const u8, trace: ?*std.builtin.StackTrace) noreturn {
    // I'll likely add a better version of this to the library itself, so you
    // can just import the panic handler.
    @setCold(true);
    term.cook() catch {};
    std.builtin.default_panic(msg, trace);
}
