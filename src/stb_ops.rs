use libc::{c_uchar, c_uint, size_t};

pub fn stb_decompress_length(input: *const c_uchar) -> c_uint
{
    return (input[8] << 24) + (input[9] << 16) + (input[10] << 8) + input[11];
}

pub unsafe fn stb__match(mut data: *const c_uchar, mut length: c_uint)
{
    // INVERSE of memmove... write each byte before copying the next...
    // IM_ASSERT(stb__dout + length <= stb__barrier_out_e);
    if stb__dout + length > stb__barrier_out_e { stb__dout += length; return; }
    if data < stb__barrier_out_b { stb__dout = stb__barrier_out_e+1; return; }
    while length {
        data += 1;
        *stb__dout = *data;
        stb__dout += 1;
        length -= 1;
    }
}

pub unsafe fn stb__lit(data: *const c_uchar, length: c_uint)
{
    // IM_ASSERT(stb__dout + length <= stb__barrier_out_e);
    if stb__dout + length > stb__barrier_out_e { stb__dout += length; return; }
    if (data < stb__barrier_in_b) { stb__dout = stb__barrier_out_e+1; return; }
    libc::memcpy(stb__dout, data, length as size_t);
    stb__dout += length;
}

pub unsafe fn stb_decompress_token(mut i: *const c_uchar) -> *const c_uchar
{
    if (*i >= 0x20) { // use fewer if's for cases that expand small
        if (*i >= 0x80) {
            stb__match(stb__dout - i[1] - 1, i[0] - 0x80 + 1);
            i += 2;
        } else if (*i >= 0x40) {
            stb__match(stb__dout - (stb__in2(0) - 0x4000 + 1), i[2] + 1);
            i += 3;
        } else /* *i >= 0x20 */ {
            stb__lit(i + 1, i[0] - 0x20 + 1);
            i += 1 + (i[0] - 0x20 + 1);
        }
    } else { // more ifs for cases that expand large, since overhead is amortized
        if (*i >= 0x18) {
            stb__match(stb__dout - (stb__in3(0) - 0x180000 + 1), i[3] + 1);
            i += 4;
        } else if (*i >= 0x10) {
            stb__match(stb__dout - (stb__in3(0) - 0x100000 + 1), stb__in2(3) + 1);
            i += 5;
        } else if (*i >= 0x08) {
            stb__lit(i + 2, stb__in2(0) - 0x0800 + 1);
            i += 2 + (stb__in2(0) - 0x0800 + 1);
        } else if (*i == 0x07) {
            stb__lit(i + 3, stb__in2(1) + 1);
            i += 3 + (stb__in2(1) + 1);
        } else if (*i == 0x06) {
            stb__match(stb__dout - (stb__in3(1) + 1), i[4] + 1);
            i += 5;
        } else if (*i == 0x04) {
            stb__match(stb__dout - (stb__in3(1) + 1), stb__in2(4) + 1);
            i += 6;
        }
    }
    return i;
}

pub unsafe fn stb_adler32(adler32: c_uint, mut buffer: *mut c_cuchar, mut buflen: size_t) -> c_uint
{
    let ADLER_MOD = 65521;
    let mut s1 = adler32 & 0xffff;
    let mut s2 = adler32 >> 16;
    let mut blocklen = buflen % 5552;

    let mut i = 0;
    while (buflen) {
        // for (i=0; i + 7 < blocklen; i += 8)
        i = 0;
        while i + 7 < blocklen
        {
            s1 += buffer[0]; s2 += s1;
            s1 += buffer[1]; s2 += s1;
            s1 += buffer[2]; s2 += s1;
            s1 += buffer[3]; s2 += s1;
            s1 += buffer[4]; s2 += s1;
            s1 += buffer[5]; s2 += s1;
            s1 += buffer[6]; s2 += s1;
            s1 += buffer[7]; s2 += s1;

            buffer += 8;
            i += 8;
        }

        // for (; i < blocklen; ++i)
        while i < blocklen
        {
            i += 1;
            s1 += *buffer;
            s2 += s1;
            s1 += 1;
        }

        s1 %= ADLER_MOD;
        s2 %= ADLER_MOD;
        buflen -= blocklen;
        blocklen = 5552;
    }
    return (s2 << 16) + s1;
}

pub unsafe fn stb_decompress(mut output: *mut c_uchar, mut i: *const c_uchar, length: c_uint /*length*/) -> size_t {
    if (stb__in4(0) != 0x57bC0000) { return 0; }
    if (stb__in4(4) != 0) { return 0; } // error! stream is > 4GB
    let mut olen: size_t = stb_decompress_length(i) as size_t;
    stb__barrier_in_b = i;
    stb__barrier_out_e = output + olen;
    stb__barrier_out_b = output;
    i += 16;

    stb__dout = output;
    loop {
        old_i: *const c_uchar = i;
        i = stb_decompress_token(i);
        if (i == old_i) {
            if (*i == 0x05 && i[1] == 0xfa) {
                // IM_ASSERT(stb__dout == output + olen);
                if (stb__dout != output + olen) { return 0; }
                if stb_adler32(1, output, olen) != stb__in4(2) {
                    return 0;
                }
                return olen;
            } else {
                // IM_ASSERT(0); /* NOTREACHED */
                return 0;
            }
        }
        // IM_ASSERT(stb__dout <= output + olen);
        if (stb__dout > output + olen) {
            return 0;
        }
    }
}
