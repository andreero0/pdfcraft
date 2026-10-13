//! The macOS app menu, with a Quit (⌘Q) that asks about unsaved changes first.
//!
//! winit's default app menu sends Quit straight to `-[NSApplication terminate:]`, and winit does
//! not implement `applicationShouldTerminate:`, so PdfCraft was never asked: unsaved work was lost
//! without a prompt. This menu has the same items, built with muda, but its Quit asks the main
//! window to close ([`PdfCraftApp::quit_from_menu`]). That goes through `guard_quit` like the close
//! button, and `macos_quit` then ends the process. The same approach as craft-libs' craft-quit
//! menu (storytold/craft-libs#3).

use std::cell::RefCell;

use muda::accelerator::{Accelerator, Code, Modifiers};
use muda::{Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu};
use pdfcraft_ui_egui::PdfCraftApp;

const QUIT_ID: &str = "pdfcraft.quit";

thread_local! {
    /// The installed menu: muda's items must outlive the native menu that shows them.
    static MENU: RefCell<Option<Menu>> = const { RefCell::new(None) };
}

/// Replaces winit's app menu. Call from the eframe app creator: winit builds its default menu
/// before that and would replace one installed earlier. On error winit's menu stays, so ⌘Q still
/// quits, without asking.
pub fn install(ctx: &egui::Context) -> muda::Result<()> {
    if MENU.with_borrow(Option::is_some) {
        return Ok(());
    }
    let quit = MenuItem::with_id(QUIT_ID, "Quit PdfCraft", true, Some(Accelerator::new(Modifiers::META, Code::KeyQ)));
    let app_menu = Submenu::new("PdfCraft", true);
    app_menu.append_items(&[
        &PredefinedMenuItem::about(Some("About PdfCraft"), None),
        &PredefinedMenuItem::separator(),
        &PredefinedMenuItem::services(None),
        &PredefinedMenuItem::separator(),
        &PredefinedMenuItem::hide(Some("Hide PdfCraft")),
        &PredefinedMenuItem::hide_others(None),
        &PredefinedMenuItem::show_all(None),
        &PredefinedMenuItem::separator(),
        &quit,
    ])?;
    let menu = Menu::new();
    menu.append(&app_menu)?;
    menu.init_for_nsapp();

    let ctx = ctx.clone();
    MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
        if event.id.as_ref() == QUIT_ID {
            PdfCraftApp::quit_from_menu(&ctx);
        }
    }));
    MENU.set(Some(menu));
    Ok(())
}
