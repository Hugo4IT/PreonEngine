use preon_module_wgpu::preon;
use preon_module_xml::get_engine_from_xml;

fn main() {
    env_logger::init();

    let engine = get_engine_from_xml(include_str!("../res/exampleApp.xml"));
    preon::run(engine, |_, _, _| {

    });
}
