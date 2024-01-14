# scrypt

Encryption and Decryption using Electronic Cookbook (ECB), Cipher block chaining (CBC)

The network is defined by a substitution-permutation network implementing an
8-bit block cipher with keys of a length of 32 bits.

- The key step takes 8 bits from the key and performs a bitwise exclusive or with current 8-bit value.
- The substitution step uses 4-bit S-boxes applied to the lower and upper 4 bits of an 8-bit word. The substitution $S : \{0,1\}^4 \to \{0,1\}^4$ is given by $x \mapsto ((x + 1) \cdot 7) \mod (17 − 1)$. This is a bijection of $\{0, 1\}^4$, where 4-bit chunks are seen as natural numbers via their binary encoding.
- The permutation step uses an 8-bit P-box $P : \{0,1\}^8 \to \{0,1\}^8$, which does a cyclic 2-bit left-shift of its argument.

The substitution-permutation network uses the following rounds:

- Round 0: Key step with the first (most significant) 8 bits of the key.

- Round 1: Substitution step followed by a permutation step followed by a key step with the next 8 bits of the key.

- Round 2: Substitution step followed by a permutation step followed by a key step with the next 8 bits of the key.

- Round 3: Substitution step followed by a key step with the last (least significant) 8 bits of the key.
