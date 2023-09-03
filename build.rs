use semver::Version;

fn main() {
    let libclamav = pkg_config::Config::new()
        .atleast_version("0.103")
        .probe("libclamav")
        .unwrap();

    let version = Version::parse(&libclamav.version).unwrap();

    if version >= Version::new(1, 0, 0) {
        println!("cargo:rustc-cfg=clamav_1_0");
    }
    if version >= Version::new(1, 2, 0) {
        println!("cargo:rustc-cfg=clamav_1_2");
    }
}
