fn main() {
  cc::Build::new()
    .file("lib/hati.c")
    .compile("hati");
}