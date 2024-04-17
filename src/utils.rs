pub fn bytes_to_word(msb: u8, lsb: u8) -> u16
{
    return ((msb as u16) << 8) + lsb as u16;
}

