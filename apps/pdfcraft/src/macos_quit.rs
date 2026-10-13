//! Quit through `-[NSApplication terminate:]` instead of closing the window (storytold/pdfcraft#1136).
//!
//! eframe drops the window while AppKit's run loop is still running, and in the next display cycle
//! AppKit's Touch Bar finder removes a KVO observer from the view that is gone
//! (`-[_NSTouchBarFinderObservation invalidate]`): an uncaught Objective-C exception, so the app
//! aborted when its window was closed, also on Macs without a Touch Bar. `terminate:` (what ⌘Q
//! does in winit's menu) ends the process before another display cycle runs. The same approach as
//! craft-libs' craft-quit (storytold/craft-libs#3). All calls are safe objc2 calls.

use dispatch2::DispatchQueue;
use objc2::MainThreadMarker;
use objc2_app_kit::NSApplication;

/// Schedules `terminate:` and returns `true`: the caller keeps its window open and the process
/// ends once the current event has been handled.
///
/// It is sent from the main queue, not from the caller: the caller is inside winit's event
/// handler, and `terminate:` makes winit deliver its exit event, which it can't do while another
/// event is being handled. From the main queue winit is idle: its `applicationWillTerminate:`
/// closes the windows and runs eframe's exit handling (saving settings), and AppKit ends the
/// process.
pub fn terminate_later() -> bool {
    DispatchQueue::main().exec_async(|| {
        // The main queue runs on the main thread, so there is always a marker.
        if let Some(mtm) = MainThreadMarker::new() {
            NSApplication::sharedApplication(mtm).terminate(None);
        }
    });
    true
}
