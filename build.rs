// Builds C sources
use std::env;
use std::path::Path;
fn build(source: &str, target: &str) {
    cc::Build::new()
	.file(source)
        .flag("-mavx2")
        .include("src/")
        //.include("arch/x86/include/")
        //.include("src/crypto/include/")
        //.include("/usr/include/linux/")
        //.include("/usr/src/linux-hwe-5.8-headers-5.8.0-38/arch/x86/include/")
        //.include("/usr/src/linux-hwe-5.8-headers-5.8.0-38/include/linux/")
        //.include("/usr/src/linux-hwe-5.8-headers-5.8.0-38/include/")
        .compile(target);
}

fn main() {
    env::set_current_dir(Path::new("src/pq_wireguard/")).unwrap();
    build("src/mceliece/operations.c", "mceliece_operations");
    //build("src/mceliece/benes.c", "mceliece_benes");dd
    //build("src/mceliece/decrypt.c", "mceliece_decrypt");
    //build("src/mceliece/fft.c", "mceliece_fft");
    //build("src/mceliece/fft_tr.c", "mceliece_fft_tr");
    //build("src/mceliece/gf.c", "mceliece_gf");
    //build("src/mceliece/int32_minmax_x86.c", "mceliece_int32_minmax_x86");
    //build("src/mceliece/int32_sort.c", "mceliece_int32_sort");
    //build("src/mceliece/mcbuf.c", "mceliece_mcbuf");

    //build("src/mceliece/transpose.c", "mceliece_transpose");
    //build("src/mceliece/uint32_sort.c", "mceliece_uint32_sort");
    //build("src/mceliece/vec128.c", "mceliece_vec128");
    //build("src/mceliece/vec256.c", "mceliece_vec256");
    // build("src/mceliece/bm.c", "mceliece_bm");
    // build("src/mceliece/synd.c", "mceliece_synd");  // implicit declaration gf_add
    //build("src/mceliece/encrypt.c", "mceliece_encrypt"); // needs asm/bug.h
}
// ./src/pq_wireguard/src/crypto/include/zinc/blake2s.h
