use core::str::FromStr;
use std::{hint::black_box, sync::LazyLock};

use criterion::{Criterion, criterion_group, criterion_main};
use darkforge::data::compositor::Composite;

const LONG_LOREM: &str = "
    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis ultricies, sem eu sollicitudin porta, mauris lacus ornare eros, nec egestas felis ligula sit amet ligula.
    Ut lobortis posuere ligula ac porttitor. Aliquam erat volutpat. Sed at nisi eget dolor elementum lobortis et ut turpis. Donec vitae odio in tortor tempus aliquet nec mollis nibh.
    Etiam aliquam est nec commodo mattis. Aenean a nunc ac elit vehicula varius. Integer malesuada lobortis dui, id mattis ligula hendrerit non. Donec et varius dui, sed posuere elit.
    In placerat tincidunt felis, et finibus libero laoreet non. Donec posuere sapien a est hendrerit, ut ultrices quam dictum.
    Quisque turpis orci, aliquet eget neque nec, porttitor mattis diam. Aenean scelerisque id felis in maximus. Praesent sit amet pretium tellus. Nunc cursus nibh id convallis cursus.
    Aenean vehicula nibh non tortor sollicitudin, in condimentum ipsum aliquam.

    Phasellus placerat, augue non eleifend molestie, dui erat pretium metus, cursus pretium sapien dolor eu lorem.
    Nulla tempus, eros sed luctus iaculis, neque purus dictum risus, ac posuere lectus nisl quis risus. Donec vitae metus dui. Donec at pellentesque mauris.
    Maecenas ut justo convallis, posuere enim quis, suscipit dui. Ut ac lobortis quam. Maecenas a lectus id lacus dignissim tincidunt vel non ligula.
    Nam urna diam, tempus eu nulla eu, tincidunt bibendum justo. Phasellus dictum finibus augue in auctor. Vivamus ac maximus sapien.
    Nunc lobortis tortor tortor, ac semper lacus sodales quis.

    Phasellus auctor volutpat arcu, ut facilisis eros semper quis. Vestibulum non ex ligula.
    Integer ut ornare neque, vitae mattis turpis. Vivamus ultricies purus orci, id dapibus erat luctus id. Proin eros lacus, fermentum ac tortor vitae, ultrices ullamcorper leo.
    Aliquam sed elit ut lacus pulvinar aliquam in rhoncus enim. Donec risus elit, suscipit ut pretium vel, varius a est. Donec venenatis vestibulum massa volutpat auctor.
    Mauris sit amet hendrerit dolor. Ut porta et nunc at fringilla. Aenean non consectetur nisl. Aenean egestas hendrerit turpis, et tristique urna luctus quis.
    Integer rhoncus, risus ut malesuada venenatis, magna sem vestibulum sem, ut luctus mauris arcu quis ante. Curabitur eget scelerisque velit.
    Mauris accumsan interdum augue vitae porta. Sed accumsan nunc non convallis blandit.

    Morbi lobortis, nibh id ultrices placerat, mi odio dapibus purus, id cursus ligula neque vitae tellus. Fusce egestas ligula a eleifend convallis.
    Morbi eleifend aliquet mauris, ac pulvinar tortor luctus at. Quisque ultricies diam ut libero luctus euismod. Nullam imperdiet faucibus nibh eget auctor.
    Etiam eu diam sagittis, vulputate nisi quis, ultricies mi. Fusce in purus justo. Suspendisse potenti.

    Duis venenatis quis metus a accumsan. Etiam arcu dolor, dignissim ac pretium et, dictum a dui. Aenean ut gravida magna, non tempor libero. Curabitur vel arcu elit.
    Quisque non nisi quis neque imperdiet placerat. Sed eros enim, vestibulum sit amet leo eget, placerat posuere ante. Nam ut est dolor.
    Ut aliquet libero pellentesque orci maximus condimentum. Phasellus ut viverra mi.

    Phasellus gravida ullamcorper odio quis fermentum. Maecenas blandit risus vel ultricies ornare. Vivamus sed lacus dapibus, iaculis elit nec, placerat erat.
    Nunc sollicitudin mollis sodales. Pellentesque et pulvinar nunc. Fusce viverra eget nulla quis egestas. Duis a nisl ante.
    Maecenas urna orci, mattis at nunc sollicitudin, eleifend posuere arcu. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos.
    Curabitur commodo maximus est, at varius nibh varius non. Aliquam fermentum volutpat turpis, eu venenatis libero condimentum ut.
    Aliquam nec nisi et neque faucibus feugiat vitae vitae enim. Donec lobortis ipsum in arcu pulvinar pulvinar.

    Praesent nec enim in ante rhoncus iaculis. Praesent ut diam dignissim, semper dolor ut, accumsan urna. Aliquam erat volutpat.
    Maecenas ultricies porta metus, nec pharetra ipsum. Mauris sed pellentesque lorem, in vestibulum nunc. Donec elementum turpis blandit ultrices pretium.
    Pellentesque venenatis pulvinar orci, ut nec.
";

const LONG_PLACEHOLDER_NAME: &str = "lorem_ipsum_dolor_sit_amet_consectetur_adipiscing_elit";

static LONG_PLACEHOLDER: LazyLock<String> = LazyLock::new(|| format!("{{{}}} Lorem ipsum dolor sit amet, consectetur.", LONG_PLACEHOLDER_NAME));
static UNIQUE_PLACEHOLDERS: LazyLock<String> = LazyLock::new(|| (1..=100).map(|i| format!("{{foo{i}}}")).collect::<Vec<_>>().join(""));
static LONG_UNIQUE_LONG_PLACEHOLDERS: LazyLock<String> = LazyLock::new(|| {
    (1..=100)
        .map(|i| format!("{{{}{}}}", LONG_PLACEHOLDER_NAME, i))
        .collect::<Vec<_>>()
        .join("")
});

fn bench_composite_from(c: &mut Criterion) {
    let short_no_placeholder = "hello";
    let long_no_placeholder = LONG_LOREM;
    let short_one_placeholder = "{foo}";
    let long_placeholder_start = "{foo} Lorem ipsum dolor sit amet, consectetur.";
    let long_placeholder_middle = "Lorem {foo} ipsum dolor sit amet, consectetur.";
    let long_placeholder_end = "Lorem ipsum dolor sit amet, consectetur. {foo}";
    let short_all_placeholders = "{foo}{bar}{baz}";
    let long_all_placeholders = "{foo}{bar}{baz}{qux}{quux}{corge}{grault}{garply}{waldo}{fred}{plugh}{xyzzy}{thud}";
    let cluster_start = "{foo}{bar}{baz} Lorem ipsum dolor sit amet, consectetur.";
    let cluster_middle = "Lorem ipsum {foo}{bar}{baz} dolor sit amet, consectetur.";
    let cluster_end = "Lorem ipsum dolor sit amet, consectetur. {foo}{bar}{baz}";
    let cluster_spread = "lorem {foo}{bar} ipsum {baz}{qux} dolor {quux}{corge} sit {grault}{garply} amet";
    let repeated_placeholders = "{foo}{bar}{baz}{foo}{bar}{baz}{foo}{bar}{baz}";
    let long_placeholder = LONG_PLACEHOLDER.as_str();
    let unique_placeholders = UNIQUE_PLACEHOLDERS.as_str();
    let long_unique_long_placeholders = LONG_UNIQUE_LONG_PLACEHOLDERS.as_str();

    let cases = [
        ("short_no_placeholder", short_no_placeholder),
        ("long_no_placeholder", long_no_placeholder),
        ("short_one_placeholder", short_one_placeholder),
        ("long_placeholder_start", long_placeholder_start),
        ("long_placeholder_middle", long_placeholder_middle),
        ("long_placeholder_end", long_placeholder_end),
        ("short_all_placeholders", short_all_placeholders),
        ("long_all_placeholders", long_all_placeholders),
        ("cluster_start", cluster_start),
        ("cluster_middle", cluster_middle),
        ("cluster_end", cluster_end),
        ("cluster_spread", cluster_spread),
        ("repeated_placeholders", repeated_placeholders),
        ("long_placeholder", &long_placeholder),
        ("unique_placeholders", &unique_placeholders),
        ("long_unique_long_placeholders", &long_unique_long_placeholders),
    ];

    for (name, input) in cases {
        // String
        c.bench_function(&format!("Composite::parse [{}]", name), |b| {
            let s = input.to_string();
            b.iter(|| Composite::parse(black_box(s.clone())).unwrap())
        });
        // &str
        c.bench_function(&format!("Composite::from_str [{}]", name), |b| {
            b.iter(|| Composite::from_str(black_box(input)).unwrap())
        });
    }
}

criterion_group!(benches, bench_composite_from);
criterion_main!(benches);
