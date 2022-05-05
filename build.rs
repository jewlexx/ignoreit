fn main() {
    cc::Build::new().file("src/sys/online.c").compile("online");
}
