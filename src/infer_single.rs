use std::path::Path;

pub fn infer_single() {
    let path =
        "/home/rafal/.nuget/packages/system.runtime/4.1.0/ref/netstandard1.5/fr/System.Runtime.xml";

    println!("{:?}", infer::get_from_path(Path::new(&path)))
}
