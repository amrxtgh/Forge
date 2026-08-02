use forge::log::Logger;
fn main() {
    Logger::init();
    Logger::info("Engine started");
    Logger::warn("Texture missing");
    Logger::error("Renderer crashed");
    Logger::debug("Renderer crashed");
    forge::print();
}
