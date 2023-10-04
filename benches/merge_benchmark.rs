use std::borrow::Cow;

use cobalt_xml_merge::*;
use criterion::{Criterion, criterion_group, criterion_main};

const ASSET_TABLE_PATH: &str = "test_files/AssetTable/!original.xml";

lazy_static::lazy_static! {
    static ref ASSET_TABLE: String = std::fs::read_to_string(ASSET_TABLE_PATH).unwrap();
}

criterion_group!(benches, bench_add_two);
criterion_main!(benches);

fn bench_add_two(c: &mut Criterion) {
    c.bench_function("boamo + playable bosses merge", |b| {
        let path_patches = [
            "test_files/AssetTable/Boamo.xml",
            "test_files/AssetTable/PlayableBoss.xml"
        ];
        let patches = path_patches.iter().map(|path| std::fs::read_to_string(path).unwrap()).collect::<Vec<_>>();
        let table = &*ASSET_TABLE;

        let boamo_playableboss = read_fs_into_strs("test_files/AssetTable/examples/Boamo_Playableboss.xml");
        let mut merged = vec![];
        b.iter(|| {
            let mut merger = Merger::new(table);
        
            for patch in &patches {
                merger.patch(patch);
            }
        
            merged = merger.finalize();
        });
        assert!(merged == boamo_playableboss);
    });
}

pub fn read_fs_into_strs<'a>(path: &'a str) -> Vec<Cow<'a, str>> {
    let file = std::fs::read_to_string(path).unwrap();
    slice(&file, |s| Cow::Owned(s.to_owned()))
}

fn slice<'a, T>(s: &'a str, map: impl Fn(&'a str) -> T) -> Vec<T> {
    let lines = s.lines();
    let mut v = Vec::with_capacity(lines.size_hint().0); // hint size to avoid some reallocations
    for slice in lines {
        let s = slice.trim();
        if s.is_empty() { continue };
        if s.starts_with("<!") { continue }; // skip comments
        if s.starts_with("<") {
            v.push(map(s));
        };
    }
    v
}