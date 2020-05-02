// extern crate tun_tap;
use std::io;

fn main() -> io::Result<()> {
    /*
     * Create Virtual Interface.
     */
    let nic = tun_tap::new("tun_0", tun_tap::Mode::Tun).except("failed to virtual interface")?;
    let mut buf = [0u8; 1504];
    let nbytes = nic.recv(&mut buf[..])?;
    eprintln!("read {} bytes: {:x}", nbytes, &buf[..nbytes]);

    /* TODO must resolve error
     * running: "cc" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-Wall" "-Wextra" "-o" "/Users/flekystyley/Source/tcp-rust/target/release/build/tun-tap-f4c3be278150ae9c/out/src/tuntap.o" "-c" "src/tuntap.c"
        cargo:warning=src/tuntap.c:11:10: fatal error: 'linux/if.h' file not found
        cargo:warning=#include <linux/if.h>
        cargo:warning=         ^~~~~~~~~~~~
        cargo:warning=1 error generated.
        exit code: 1

     * 多分だけどlinxカーネルのversionが古いか,if.hが入ってない(これはどのライブラリに依存してる？)
    */

    Ok(())
}
