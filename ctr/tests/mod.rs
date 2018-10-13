extern crate aes;
extern crate ctr;
#[macro_use] extern crate stream_cipher;

type Aes128Ctr = ctr::Ctr128<aes::Aes128>;
type Aes256Ctr = ctr::Ctr128<aes::Aes256>;

// Random tests generated by OpenSSL
new_core_test!(aes128_ctr_core, Aes128Ctr, "ctr_aes128");
new_seek_test!(aes128_ctr_seek, Aes128Ctr, "ctr_aes128");
new_core_test!(aes256_ctr_core, Aes256Ctr, "ctr_aes256");
new_seek_test!(aes256_ctr_seek, Aes256Ctr, "ctr_aes256");