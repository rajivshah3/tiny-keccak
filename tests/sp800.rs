use tiny_keccak::{Hasher, CShake, KMac, TupleHash};

#[test]
fn test_cshake128() {
    let input = b"\x00\x01\x02\x03";
    let name = b"";
    let custom = b"Email Signature";
    let output = b"\xC1\xC3\x69\x25\xB6\x40\x9A\x04\xF1\xB5\x04\xFC\xBC\xA9\xD8\x2B\x40\x17\x27\x7C\xB5\xED\x2B\x20\x65\xFC\x1D\x38\x14\xD5\xAA\xF5";

    let mut buf = vec![0; output.len()];
    let mut cshake = CShake::v128(name, custom);
    cshake.update(input);
    cshake.finalize(&mut buf);
    assert_eq!(buf, output);

    let input = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F\
                        \x20\x21\x22\x23\x24\x25\x26\x27\x28\x29\x2A\x2B\x2C\x2D\x2E\x2F\x30\x31\x32\x33\x34\x35\x36\x37\x38\x39\x3A\x3B\x3C\x3D\x3E\x3F\
                        \x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A\x5B\x5C\x5D\x5E\x5F\
                        \x60\x61\x62\x63\x64\x65\x66\x67\x68\x69\x6A\x6B\x6C\x6D\x6E\x6F\x70\x71\x72\x73\x74\x75\x76\x77\x78\x79\x7A\x7B\x7C\x7D\x7E\x7F\
                        \x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8A\x8B\x8C\x8D\x8E\x8F\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9A\x9B\x9C\x9D\x9E\x9F\
                        \xA0\xA1\xA2\xA3\xA4\xA5\xA6\xA7\xA8\xA9\xAA\xAB\xAC\xAD\xAE\xAF\xB0\xB1\xB2\xB3\xB4\xB5\xB6\xB7\xB8\xB9\xBA\xBB\xBC\xBD\xBE\xBF\
                        \xC0\xC1\xC2\xC3\xC4\xC5\xC6\xC7";
    let name = b"";
    let custom = b"Email Signature";
    let output = b"\xC5\x22\x1D\x50\xE4\xF8\x22\xD9\x6A\x2E\x88\x81\xA9\x61\x42\x0F\x29\x4B\x7B\x24\xFE\x3D\x20\x94\xBA\xED\x2C\x65\x24\xCC\x16\x6B";

    let mut buf = vec![0; output.len()];
    let mut cshake = CShake::v128(name, custom);
    cshake.update(input);
    cshake.finalize(&mut buf);
    assert_eq!(buf, output);
}

#[test]
fn test_cshake256() {
    let input = b"\x00\x01\x02\x03";
    let name = b"";
    let custom = b"Email Signature";
    let output = b"\xD0\x08\x82\x8E\x2B\x80\xAC\x9D\x22\x18\xFF\xEE\x1D\x07\x0C\x48\xB8\xE4\xC8\x7B\xFF\x32\xC9\x69\x9D\x5B\x68\x96\xEE\xE0\xED\xD1\
                        \x64\x02\x0E\x2B\xE0\x56\x08\x58\xD9\xC0\x0C\x03\x7E\x34\xA9\x69\x37\xC5\x61\xA7\x4C\x41\x2B\xB4\xC7\x46\x46\x95\x27\x28\x1C\x8C";

    let mut buf = vec![0; output.len()];
    let mut cshake = CShake::v256(name, custom);
    cshake.update(input);
    cshake.finalize(&mut buf);
    assert_eq!(buf, &output[..]);


    let input = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F\
                        \x20\x21\x22\x23\x24\x25\x26\x27\x28\x29\x2A\x2B\x2C\x2D\x2E\x2F\x30\x31\x32\x33\x34\x35\x36\x37\x38\x39\x3A\x3B\x3C\x3D\x3E\x3F\
                        \x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A\x5B\x5C\x5D\x5E\x5F\
                        \x60\x61\x62\x63\x64\x65\x66\x67\x68\x69\x6A\x6B\x6C\x6D\x6E\x6F\x70\x71\x72\x73\x74\x75\x76\x77\x78\x79\x7A\x7B\x7C\x7D\x7E\x7F\
                        \x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8A\x8B\x8C\x8D\x8E\x8F\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9A\x9B\x9C\x9D\x9E\x9F\
                        \xA0\xA1\xA2\xA3\xA4\xA5\xA6\xA7\xA8\xA9\xAA\xAB\xAC\xAD\xAE\xAF\xB0\xB1\xB2\xB3\xB4\xB5\xB6\xB7\xB8\xB9\xBA\xBB\xBC\xBD\xBE\xBF\
                        \xC0\xC1\xC2\xC3\xC4\xC5\xC6\xC7";
    let name = b"";
    let custom = b"Email Signature";
    let output = b"\x07\xDC\x27\xB1\x1E\x51\xFB\xAC\x75\xBC\x7B\x3C\x1D\x98\x3E\x8B\x4B\x85\xFB\x1D\xEF\xAF\x21\x89\x12\xAC\x86\x43\x02\x73\x09\x17\
                        \x27\xF4\x2B\x17\xED\x1D\xF6\x3E\x8E\xC1\x18\xF0\x4B\x23\x63\x3C\x1D\xFB\x15\x74\xC8\xFB\x55\xCB\x45\xDA\x8E\x25\xAF\xB0\x92\xBB";

    let mut buf = vec![0; output.len()];
    let mut cshake = CShake::v256(name, custom);
    cshake.update(input);
    cshake.finalize(&mut buf);
    assert_eq!(buf, &output[..]);
}

#[test]
fn test_kmac128() {
    let key = b"\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A\x5B\x5C\x5D\x5E\x5F";
    let data = b"\x00\x01\x02\x03";
    let custom = b"";
    let output = b"\xE5\x78\x0B\x0D\x3E\xA6\xF7\xD3\xA4\x29\xC5\x70\x6A\xA4\x3A\x00\xFA\xDB\xD7\xD4\x96\x28\x83\x9E\x31\x87\x24\x3F\x45\x6E\xE1\x4E";

    let mut buf = vec![0; output.len()];
    let mut kmac = KMac::v128(key, custom);
    kmac.update(data);
    kmac.finalize(&mut buf);
    assert_eq!(buf, output);


    let key = b"\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A\x5B\x5C\x5D\x5E\x5F";
    let data = b"\x00\x01\x02\x03";
    let custom = b"My Tagged Application";
    let output = b"\x3B\x1F\xBA\x96\x3C\xD8\xB0\xB5\x9E\x8C\x1A\x6D\x71\x88\x8B\x71\x43\x65\x1A\xF8\xBA\x0A\x70\x70\xC0\x97\x9E\x28\x11\x32\x4A\xA5";

    let mut buf = vec![0; output.len()];
    let mut kmac = KMac::v128(key, custom);
    kmac.update(data);
    kmac.finalize(&mut buf);
    assert_eq!(buf, output);


    let key = b"\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A\x5B\x5C\x5D\x5E\x5F";
    let data = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F\
                    \x20\x21\x22\x23\x24\x25\x26\x27\x28\x29\x2A\x2B\x2C\x2D\x2E\x2F\x30\x31\x32\x33\x34\x35\x36\x37\x38\x39\x3A\x3B\x3C\x3D\x3E\x3F\
                    \x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A\x5B\x5C\x5D\x5E\x5F\
                    \x60\x61\x62\x63\x64\x65\x66\x67\x68\x69\x6A\x6B\x6C\x6D\x6E\x6F\x70\x71\x72\x73\x74\x75\x76\x77\x78\x79\x7A\x7B\x7C\x7D\x7E\x7F\
                    \x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8A\x8B\x8C\x8D\x8E\x8F\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9A\x9B\x9C\x9D\x9E\x9F\
                    \xA0\xA1\xA2\xA3\xA4\xA5\xA6\xA7\xA8\xA9\xAA\xAB\xAC\xAD\xAE\xAF\xB0\xB1\xB2\xB3\xB4\xB5\xB6\xB7\xB8\xB9\xBA\xBB\xBC\xBD\xBE\xBF\
                    \xC0\xC1\xC2\xC3\xC4\xC5\xC6\xC7";
    let custom = b"My Tagged Application";
    let output = b"\x1F\x5B\x4E\x6C\xCA\x02\x20\x9E\x0D\xCB\x5C\xA6\x35\xB8\x9A\x15\xE2\x71\xEC\xC7\x60\x07\x1D\xFD\x80\x5F\xAA\x38\xF9\x72\x92\x30";

    let mut buf = vec![0; output.len()];
    let mut kmac = KMac::v128(key, custom);
    kmac.update(data);
    kmac.finalize(&mut buf);
    assert_eq!(buf, output);
}

#[test]
fn test_kmac256() {
    let key = b"\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A\x5B\x5C\x5D\x5E\x5F";
    let data = b"\x00\x01\x02\x03";
    let custom = b"My Tagged Application";
    let output = b"\x20\xC5\x70\xC3\x13\x46\xF7\x03\xC9\xAC\x36\xC6\x1C\x03\xCB\x64\xC3\x97\x0D\x0C\xFC\x78\x7E\x9B\x79\x59\x9D\x27\x3A\x68\xD2\xF7\
                    \xF6\x9D\x4C\xC3\xDE\x9D\x10\x4A\x35\x16\x89\xF2\x7C\xF6\xF5\x95\x1F\x01\x03\xF3\x3F\x4F\x24\x87\x10\x24\xD9\xC2\x77\x73\xA8\xDD";

    let mut buf = vec![0; output.len()];
    let mut kmac = KMac::v256(key, custom);
    kmac.update(data);
    kmac.finalize(&mut buf);
    assert_eq!(buf, &output[..]);


    let key = b"\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A\x5B\x5C\x5D\x5E\x5F";
    let data = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F\
                    \x20\x21\x22\x23\x24\x25\x26\x27\x28\x29\x2A\x2B\x2C\x2D\x2E\x2F\x30\x31\x32\x33\x34\x35\x36\x37\x38\x39\x3A\x3B\x3C\x3D\x3E\x3F\
                    \x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A\x5B\x5C\x5D\x5E\x5F\
                    \x60\x61\x62\x63\x64\x65\x66\x67\x68\x69\x6A\x6B\x6C\x6D\x6E\x6F\x70\x71\x72\x73\x74\x75\x76\x77\x78\x79\x7A\x7B\x7C\x7D\x7E\x7F\
                    \x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8A\x8B\x8C\x8D\x8E\x8F\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9A\x9B\x9C\x9D\x9E\x9F\
                    \xA0\xA1\xA2\xA3\xA4\xA5\xA6\xA7\xA8\xA9\xAA\xAB\xAC\xAD\xAE\xAF\xB0\xB1\xB2\xB3\xB4\xB5\xB6\xB7\xB8\xB9\xBA\xBB\xBC\xBD\xBE\xBF\
                    \xC0\xC1\xC2\xC3\xC4\xC5\xC6\xC7";
    let custom = b"";
    let output = b"\x75\x35\x8C\xF3\x9E\x41\x49\x4E\x94\x97\x07\x92\x7C\xEE\x0A\xF2\x0A\x3F\xF5\x53\x90\x4C\x86\xB0\x8F\x21\xCC\x41\x4B\xCF\xD6\x91\
                    \x58\x9D\x27\xCF\x5E\x15\x36\x9C\xBB\xFF\x8B\x9A\x4C\x2E\xB1\x78\x00\x85\x5D\x02\x35\xFF\x63\x5D\xA8\x25\x33\xEC\x6B\x75\x9B\x69";

    let mut buf = vec![0; output.len()];
    let mut kmac = KMac::v256(key, custom);
    kmac.update(data);
    kmac.finalize(&mut buf);
    assert_eq!(buf, &output[..]);


    let key = b"\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A\x5B\x5C\x5D\x5E\x5F";
    let data = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F\
                    \x20\x21\x22\x23\x24\x25\x26\x27\x28\x29\x2A\x2B\x2C\x2D\x2E\x2F\x30\x31\x32\x33\x34\x35\x36\x37\x38\x39\x3A\x3B\x3C\x3D\x3E\x3F\
                    \x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A\x5B\x5C\x5D\x5E\x5F\
                    \x60\x61\x62\x63\x64\x65\x66\x67\x68\x69\x6A\x6B\x6C\x6D\x6E\x6F\x70\x71\x72\x73\x74\x75\x76\x77\x78\x79\x7A\x7B\x7C\x7D\x7E\x7F\
                    \x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8A\x8B\x8C\x8D\x8E\x8F\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9A\x9B\x9C\x9D\x9E\x9F\
                    \xA0\xA1\xA2\xA3\xA4\xA5\xA6\xA7\xA8\xA9\xAA\xAB\xAC\xAD\xAE\xAF\xB0\xB1\xB2\xB3\xB4\xB5\xB6\xB7\xB8\xB9\xBA\xBB\xBC\xBD\xBE\xBF\
                    \xC0\xC1\xC2\xC3\xC4\xC5\xC6\xC7";
    let custom = b"My Tagged Application";
    let output = b"\xB5\x86\x18\xF7\x1F\x92\xE1\xD5\x6C\x1B\x8C\x55\xDD\xD7\xCD\x18\x8B\x97\xB4\xCA\x4D\x99\x83\x1E\xB2\x69\x9A\x83\x7D\xA2\xE4\xD9\
                    \x70\xFB\xAC\xFD\xE5\x00\x33\xAE\xA5\x85\xF1\xA2\x70\x85\x10\xC3\x2D\x07\x88\x08\x01\xBD\x18\x28\x98\xFE\x47\x68\x76\xFC\x89\x65";

    let mut buf = vec![0; output.len()];
    let mut kmac = KMac::v256(key, custom);
    kmac.update(data);
    kmac.finalize(&mut buf);
    assert_eq!(buf, &output[..]);
}

//#[test]
//fn test_kmac128_xof() {
    //let key = b"\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A\x5B\x5C\x5D\x5E\x5F";
    //let data = b"\x00\x01\x02\x03";
    //let custom = b"";
    //let output = b"\xCD\x83\x74\x0B\xBD\x92\xCC\xC8\xCF\x03\x2B\x14\x81\xA0\xF4\x46\x0E\x7C\xA9\xDD\x12\xB0\x8A\x0C\x40\x31\x17\x8B\xAC\xD6\xEC\x35";

    //let mut buf = vec![0; output.len()];
    //let mut kmac = kmac128(key, custom);
    //kmac.update(data);
    //let mut xof = kmac.xof();
    //xof.squeeze(&mut buf);
    //assert_eq!(buf, output);


    //let key = b"\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A\x5B\x5C\x5D\x5E\x5F";
    //let data = b"\x00\x01\x02\x03";
    //let custom = b"My Tagged Application";
    //let output = b"\x31\xA4\x45\x27\xB4\xED\x9F\x5C\x61\x01\xD1\x1D\xE6\xD2\x6F\x06\x20\xAA\x5C\x34\x1D\xEF\x41\x29\x96\x57\xFE\x9D\xF1\xA3\xB1\x6C";

    //let mut buf = vec![0; output.len()];
    //let mut kmac = kmac128(key, custom);
    //kmac.update(data);
    //let mut xof = kmac.xof();
    //xof.squeeze(&mut buf);
    //assert_eq!(buf, output);


    //let key = b"\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A\x5B\x5C\x5D\x5E\x5F";
    //let data = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F\
                    //\x20\x21\x22\x23\x24\x25\x26\x27\x28\x29\x2A\x2B\x2C\x2D\x2E\x2F\x30\x31\x32\x33\x34\x35\x36\x37\x38\x39\x3A\x3B\x3C\x3D\x3E\x3F\
                    //\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A\x5B\x5C\x5D\x5E\x5F\
                    //\x60\x61\x62\x63\x64\x65\x66\x67\x68\x69\x6A\x6B\x6C\x6D\x6E\x6F\x70\x71\x72\x73\x74\x75\x76\x77\x78\x79\x7A\x7B\x7C\x7D\x7E\x7F\
                    //\x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8A\x8B\x8C\x8D\x8E\x8F\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9A\x9B\x9C\x9D\x9E\x9F\
                    //\xA0\xA1\xA2\xA3\xA4\xA5\xA6\xA7\xA8\xA9\xAA\xAB\xAC\xAD\xAE\xAF\xB0\xB1\xB2\xB3\xB4\xB5\xB6\xB7\xB8\xB9\xBA\xBB\xBC\xBD\xBE\xBF\
                    //\xC0\xC1\xC2\xC3\xC4\xC5\xC6\xC7";
    //let custom = b"My Tagged Application";
    //let output = b"\x47\x02\x6C\x7C\xD7\x93\x08\x4A\xA0\x28\x3C\x25\x3E\xF6\x58\x49\x0C\x0D\xB6\x14\x38\xB8\x32\x6F\xE9\xBD\xDF\x28\x1B\x83\xAE\x0F";

    //let mut buf = vec![0; output.len()];
    //let mut kmac = kmac128(key, custom);
    //kmac.update(data);
    //let mut xof = kmac.xof();
    //xof.squeeze(&mut buf);
    //assert_eq!(buf, output);
//}

//#[test]
//fn test_kmac256_xof() {
    //let key = b"\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A\x5B\x5C\x5D\x5E\x5F";
    //let data = b"\x00\x01\x02\x03";
    //let custom = b"My Tagged Application";
    //let output = b"\x17\x55\x13\x3F\x15\x34\x75\x2A\xAD\x07\x48\xF2\xC7\x06\xFB\x5C\x78\x45\x12\xCA\xB8\x35\xCD\x15\x67\x6B\x16\xC0\xC6\x64\x7F\xA9\
                    //\x6F\xAA\x7A\xF6\x34\xA0\xBF\x8F\xF6\xDF\x39\x37\x4F\xA0\x0F\xAD\x9A\x39\xE3\x22\xA7\xC9\x20\x65\xA6\x4E\xB1\xFB\x08\x01\xEB\x2B";

    //let mut buf = vec![0; output.len()];
    //let mut kmac = kmac256(key, custom);
    //kmac.update(data);
    //let mut xof = kmac.xof();
    //xof.squeeze(&mut buf);
    //assert_eq!(buf, &output[..]);


    //let key = b"\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A\x5B\x5C\x5D\x5E\x5F";
    //let data = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F\
                    //\x20\x21\x22\x23\x24\x25\x26\x27\x28\x29\x2A\x2B\x2C\x2D\x2E\x2F\x30\x31\x32\x33\x34\x35\x36\x37\x38\x39\x3A\x3B\x3C\x3D\x3E\x3F\
                    //\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A\x5B\x5C\x5D\x5E\x5F\
                    //\x60\x61\x62\x63\x64\x65\x66\x67\x68\x69\x6A\x6B\x6C\x6D\x6E\x6F\x70\x71\x72\x73\x74\x75\x76\x77\x78\x79\x7A\x7B\x7C\x7D\x7E\x7F\
                    //\x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8A\x8B\x8C\x8D\x8E\x8F\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9A\x9B\x9C\x9D\x9E\x9F\
                    //\xA0\xA1\xA2\xA3\xA4\xA5\xA6\xA7\xA8\xA9\xAA\xAB\xAC\xAD\xAE\xAF\xB0\xB1\xB2\xB3\xB4\xB5\xB6\xB7\xB8\xB9\xBA\xBB\xBC\xBD\xBE\xBF\
                    //\xC0\xC1\xC2\xC3\xC4\xC5\xC6\xC7";
    //let custom = b"";
    //let output = b"\xFF\x7B\x17\x1F\x1E\x8A\x2B\x24\x68\x3E\xED\x37\x83\x0E\xE7\x97\x53\x8B\xA8\xDC\x56\x3F\x6D\xA1\xE6\x67\x39\x1A\x75\xED\xC0\x2C\
                    //\xA6\x33\x07\x9F\x81\xCE\x12\xA2\x5F\x45\x61\x5E\xC8\x99\x72\x03\x1D\x18\x33\x73\x31\xD2\x4C\xEB\x8F\x8C\xA8\xE6\xA1\x9F\xD9\x8B";

    //let mut buf = vec![0; output.len()];
    //let mut kmac = kmac256(key, custom);
    //kmac.update(data);
    //let mut xof = kmac.xof();
    //xof.squeeze(&mut buf);
    //assert_eq!(buf, &output[..]);


    //let key = b"\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A\x5B\x5C\x5D\x5E\x5F";
    //let data = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F\
                    //\x20\x21\x22\x23\x24\x25\x26\x27\x28\x29\x2A\x2B\x2C\x2D\x2E\x2F\x30\x31\x32\x33\x34\x35\x36\x37\x38\x39\x3A\x3B\x3C\x3D\x3E\x3F\
                    //\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A\x5B\x5C\x5D\x5E\x5F\
                    //\x60\x61\x62\x63\x64\x65\x66\x67\x68\x69\x6A\x6B\x6C\x6D\x6E\x6F\x70\x71\x72\x73\x74\x75\x76\x77\x78\x79\x7A\x7B\x7C\x7D\x7E\x7F\
                    //\x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8A\x8B\x8C\x8D\x8E\x8F\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9A\x9B\x9C\x9D\x9E\x9F\
                    //\xA0\xA1\xA2\xA3\xA4\xA5\xA6\xA7\xA8\xA9\xAA\xAB\xAC\xAD\xAE\xAF\xB0\xB1\xB2\xB3\xB4\xB5\xB6\xB7\xB8\xB9\xBA\xBB\xBC\xBD\xBE\xBF\
                    //\xC0\xC1\xC2\xC3\xC4\xC5\xC6\xC7";
    //let custom = b"My Tagged Application";
    //let output = b"\xD5\xBE\x73\x1C\x95\x4E\xD7\x73\x28\x46\xBB\x59\xDB\xE3\xA8\xE3\x0F\x83\xE7\x7A\x4B\xFF\x44\x59\xF2\xF1\xC2\xB4\xEC\xEB\xB8\xCE\
                         //\x67\xBA\x01\xC6\x2E\x8A\xB8\x57\x8D\x2D\x49\x9B\xD1\xBB\x27\x67\x68\x78\x11\x90\x02\x0A\x30\x6A\x97\xDE\x28\x1D\xCC\x30\x30\x5D";

    //let mut buf = vec![0; output.len()];
    //let mut kmac = kmac256(key, custom);
    //kmac.update(data);
    //let mut xof = kmac.xof();
    //xof.squeeze(&mut buf);
    //assert_eq!(buf, &output[..]);
//}

#[test]
fn test_tuple_hash128() {
    let te3 = b"\x00\x01\x02";
    let te6 = b"\x10\x11\x12\x13\x14\x15";
    let te9 = b"\x20\x21\x22\x23\x24\x25\x26\x27\x28";
    let s0 = b"";
    let s1 = b"My Tuple App";


    let output = b"\xC5\xD8\x78\x6C\x1A\xFB\x9B\x82\x11\x1A\xB3\x4B\x65\xB2\xC0\x04\x8F\xA6\x4E\x6D\x48\xE2\x63\x26\x4C\xE1\x70\x7D\x3F\xFC\x8E\xD1";
    let mut buf = vec![0; output.len()];
    let mut hasher = TupleHash::v128(s0);
    hasher.update(te3);
    hasher.update(te6);
    hasher.finalize(&mut buf);
    assert_eq!(buf, output);


    let output = b"\x75\xCD\xB2\x0F\xF4\xDB\x11\x54\xE8\x41\xD7\x58\xE2\x41\x60\xC5\x4B\xAE\x86\xEB\x8C\x13\xE7\xF5\xF4\x0E\xB3\x55\x88\xE9\x6D\xFB";
    let mut buf = vec![0; output.len()];
    let mut hasher = TupleHash::v128(s1);
    hasher.update(te3);
    hasher.update(te6);
    hasher.finalize(&mut buf);
    assert_eq!(buf, output);


    let output = b"\xE6\x0F\x20\x2C\x89\xA2\x63\x1E\xDA\x8D\x4C\x58\x8C\xA5\xFD\x07\xF3\x9E\x51\x51\x99\x8D\xEC\xCF\x97\x3A\xDB\x38\x04\xBB\x6E\x84";
    let mut buf = vec![0; output.len()];
    let mut hasher = TupleHash::v128(s1);
    hasher.update(te3);
    hasher.update(te6);
    hasher.update(te9);
    hasher.finalize(&mut buf);
    assert_eq!(buf, output);
}

#[test]
fn test_tuple_hash256() {
    let te3 = b"\x00\x01\x02";
    let te6 = b"\x10\x11\x12\x13\x14\x15";
    let te9 = b"\x20\x21\x22\x23\x24\x25\x26\x27\x28";
    let s0 = b"";
    let s1 = b"My Tuple App";


    let output = b"\xCF\xB7\x05\x8C\xAC\xA5\xE6\x68\xF8\x1A\x12\xA2\x0A\x21\x95\xCE\x97\xA9\x25\xF1\xDB\xA3\xE7\x44\x9A\x56\xF8\x22\x01\xEC\x60\x73\
                    \x11\xAC\x26\x96\xB1\xAB\x5E\xA2\x35\x2D\xF1\x42\x3B\xDE\x7B\xD4\xBB\x78\xC9\xAE\xD1\xA8\x53\xC7\x86\x72\xF9\xEB\x23\xBB\xE1\x94";
    let mut buf = vec![0; output.len()];
    let mut hasher = TupleHash::v256(s0);
    hasher.update(te3);
    hasher.update(te6);
    hasher.finalize(&mut buf);
    assert_eq!(buf, &output[..]);


    let output = b"\x14\x7C\x21\x91\xD5\xED\x7E\xFD\x98\xDB\xD9\x6D\x7A\xB5\xA1\x16\x92\x57\x6F\x5F\xE2\xA5\x06\x5F\x3E\x33\xDE\x6B\xBA\x9F\x3A\xA1\
                    \xC4\xE9\xA0\x68\xA2\x89\xC6\x1C\x95\xAA\xB3\x0A\xEE\x1E\x41\x0B\x0B\x60\x7D\xE3\x62\x0E\x24\xA4\xE3\xBF\x98\x52\xA1\xD4\x36\x7E";
    let mut buf = vec![0; output.len()];
    let mut hasher = TupleHash::v256(s1);
    hasher.update(te3);
    hasher.update(te6);
    hasher.finalize(&mut buf);
    assert_eq!(buf, &output[..]);


    let output = b"\x45\x00\x0B\xE6\x3F\x9B\x6B\xFD\x89\xF5\x47\x17\x67\x0F\x69\xA9\xBC\x76\x35\x91\xA4\xF0\x5C\x50\xD6\x88\x91\xA7\x44\xBC\xC6\xE7\
                    \xD6\xD5\xB5\xE8\x2C\x01\x8D\xA9\x99\xED\x35\xB0\xBB\x49\xC9\x67\x8E\x52\x6A\xBD\x8E\x85\xC1\x3E\xD2\x54\x02\x1D\xB9\xE7\x90\xCE";
    let mut buf = vec![0; output.len()];
    let mut hasher = TupleHash::v256(s1);
    hasher.update(te3);
    hasher.update(te6);
    hasher.update(te9);
    hasher.finalize(&mut buf);
    assert_eq!(buf, &output[..]);
}
