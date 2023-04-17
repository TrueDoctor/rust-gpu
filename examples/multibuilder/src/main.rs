use spirv_builder::{MetadataPrintout, SpirvBuilder, SpirvMetadata};

fn main() {
    let result = SpirvBuilder::new(
        concat!(env!("CARGO_MANIFEST_DIR"), "/../shaders/sky-shader"),
        "spirv-unknown-spv1.5",
    )
    .print_metadata(MetadataPrintout::DependencyOnly)
    .multimodule(false)
    .preserve_bindings(true)
    .release(true)
    //.relax_struct_store(true)
    //.relax_block_layout(true)
    .spirv_metadata(SpirvMetadata::Full)
    //.print_metadata(MetadataPrintout::DependencyOnly)
    //.multimodule(true)
    .build()
    .unwrap();
    println!("{result:#?}");
}
