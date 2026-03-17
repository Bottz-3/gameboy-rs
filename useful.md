## Bit patterns for self reference:
Set bit: f |= 1 << n - OR with a 1 at pos n
Clear bit: f &= !(1 << n) - AND with a 0 at pos n
Flip bit: f ^= 1 << n - XOR with a 1 at pos n
Read bit: (f >> n) & 1 (made sense already)
