use forge::log::Logger;
fn main() {
    forge::print();
    Logger::init();
    Logger::core_info("Engine started");
    Logger::core_warn("Texture missing");
    Logger::client_error("Renderer crashed");
    Logger::client_debug("Renderer crashed");
}
