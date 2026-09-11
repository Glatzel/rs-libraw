use criterion::{Criterion, black_box, criterion_group, criterion_main};
use fornax_core::{BayerImage, FornaxPrimitive, IDecoder};
use fornax_dalim::demosaic::{DemosaicLinear, IDemosaic};

fn bench_methods<M, T>(c: &mut Criterion, name: &str, method: M, image: BayerImage<T>)
where
    M: IDemosaic<T>,
    T: FornaxPrimitive,
{
    c.bench_function(name, |b| b.iter(|| method.demosaic(black_box(&image))));
}
fn benches(c: &mut Criterion) {
    let libraw = libraw::Libraw::default();
    IDecoder::<f32>::decode_file(&libraw, &fornax_devtool::raw_file()).unwrap();
    let image = libraw.bayer_image().unwrap();
    bench_methods::<_, f32>(c, "linear", DemosaicLinear, image.clone());
}
criterion_group!(benches_group, benches);
criterion_main!(benches_group);
