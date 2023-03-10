mod render;

use render::render;

use gui::GuiFactoryDynamic;
use macos_gui::factory::MacFactory;
use windows_gui::factory::WindowsFactory;

fn main() {
    let windows = false;

    // Allocate a factory object in runtime depending on unpredictable input.
    let factory: &dyn GuiFactoryDynamic = if windows {
        &WindowsFactory
    } else {
        &MacFactory
    };

    // Factory invocation can be inlined right here.
    factory.button1.press();
    factory.checkbox1.switch();

    // Factory object can be passed to a function as a parameter.
    render(factory);
}