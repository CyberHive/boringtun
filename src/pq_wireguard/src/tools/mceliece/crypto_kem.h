#ifndef crypto_kem_H
#define crypto_kem_H

#include "crypto_kem_mceliece.h"

#define crypto_kem_keypair crypto_kem_mceliece_keypair

#define crypto_kem_PUBLICKEYBYTES crypto_kem_mceliece_PUBLICKEYBYTES
#define crypto_kem_SECRETKEYBYTES crypto_kem_mceliece_SECRETKEYBYTES
#define crypto_kem_BYTES crypto_kem_mceliece_BYTES
#define crypto_kem_CIPHERTEXTBYTES crypto_kem_mceliece_CIPHERTEXTBYTES
#define crypto_kem_PRIMITIVE "mceliece"

#endif
